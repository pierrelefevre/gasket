use crate::args;
use crate::state;
use crate::state::StateDump;
use crate::state::Worker;
use crate::stream;
use crate::stream::Options;
use crate::stream::Output;
use crate::worker;
use clap::Parser;
use rand;
use reqwest;
use serde_json::Error;
use std::collections::HashSet;
use std::sync::Arc;
use uuid::Uuid;

// Load state from file, apply to state and return Dump
pub(crate) async fn load_state(state: Arc<state::App>) -> Option<StateDump> {
    let args = args::Args::parse();
    log::info!("Restoring state from {}", args.state_file);
    let file = std::fs::read_to_string(args.state_file);
    if file.is_err() {
        log::info!("No state file found");
        return None;
    }

    let result: Result<state::StateDump, Error> = serde_json::from_str(&file.unwrap());
    if result.is_err() {
        log::error!("Error deserializing state file");
        return None;
    }
    let loaded_dump = result.unwrap();

    let mut workers = state.workers.lock().await;
    for worker in &loaded_dump.workers {
        let mut found = false;
        for w in workers.iter_mut() {
            if w.host == worker.host {
                found = true;
                break;
            }
        }
        if !found {
            workers.push(worker.clone());
        }
    }
    log::info!("Restored {} workers", workers.len());

    let mut streams = state.streams.lock().await;
    for stream in &loaded_dump.streams {
        let mut found = false;
        for s in streams.iter_mut() {
            if s.id == stream.id {
                found = true;
                break;
            }
        }
        if !found {
            streams.push(stream.clone());
        }
    }
    log::info!("Restored {} streams", streams.len());

    Some(loaded_dump)
}

pub(crate) async fn save_state(state: Arc<state::App>) {
    let args = args::Args::parse();

    let path = std::path::Path::new(&args.state_file);
    let prefix = path.parent().unwrap();
    std::fs::create_dir_all(prefix).unwrap();
    let file = std::fs::File::create(path);
    if file.is_err() {
        log::error!("Error creating state file");
        return;
    }

    let dump = state.dump().await;
    let dump_json = serde_json::to_string(&dump);
    if dump_json.is_err() {
        log::error!("Error serializing state");
    }
    std::fs::write(path, dump_json.unwrap()).expect("Unable to write file");
}

pub(crate) async fn load_worker_discovery(state: Arc<state::App>) {
    let args = args::Args::parse();

    if args.worker_discovery.is_none() {
        return;
    }

    let worker_discovery = args.worker_discovery.unwrap();
    let workers: Vec<&str> = worker_discovery.split(';').collect();

    let mut discovered: u32 = 0;
    let mut already_exists: u32 = 0;

    for worker in workers {
        let proto_and_host: &str;
        let public_ip: Option<String>;

        let worker_info: Vec<&str> = worker.split('@').collect();
        if worker_info.len() == 2 {
            proto_and_host = worker_info[0];
            public_ip = Some(worker_info[1].to_string());
        } else if worker_info.len() == 1 {
            proto_and_host = worker_info[0];
            public_ip = None;
        } else {
            log::error!("Invalid worker discovery format {}", worker);
            continue;
        }

        let proto_and_host_info: Vec<&str> = proto_and_host.split("://").collect();

        let protocol: String;
        let host: String;

        if proto_and_host_info.len() == 2 {
            protocol = proto_and_host_info[0].to_lowercase();
            host = proto_and_host_info[1].to_lowercase();
        } else if proto_and_host_info.len() == 1 {
            protocol = "http".to_string();
            host = proto_and_host_info[0].to_lowercase();
        } else {
            log::error!("Invalid worker discovery format {}", worker);
            continue;
        }

        // Check if worker already exists
        let mut workers = state.workers.lock().await;
        let mut found = false;
        for w in workers.iter_mut() {
            if w.host == host {
                found = true;
                already_exists += 1;
                break;
            }
        }

        if !found {
            let new_worker = Worker {
                id: Uuid::new_v4(),
                protocol: protocol.to_string(),
                host: host.to_string(),
                public_ip,
                udp_ports: vec![],
                codecs: vec![],
                encoder: None,
                status: state::WorkerStatus::Configuring,
                stats: state::WorkerStats {
                    utilization: 0,
                    devices: vec![],
                },
                server: None,
                streams: None,
            };
            workers.push(new_worker);
            discovered += 1;
        }
    }

    log::info!(
        "Discovered {} new workers, {} already existed",
        discovered,
        already_exists
    );
}

pub(crate) async fn get_worker_info(state: Arc<state::App>, worker: state::Worker) {
    let url = format!("{}://{}/", worker.protocol, worker.host);
    let client = reqwest::Client::new();
    let response = client.get(&url).send().await;
    if response.is_err() {
        return;
    }

    let worker_info: state::WorkerInfo =
        serde_json::from_str(&response.unwrap().text().await.unwrap()).unwrap();

    let mut workers = state.workers.lock().await;
    for w in workers.iter_mut() {
        if w.id == worker.id {
            w.server = Some(worker_info.server);
            w.streams = Some(worker_info.streams);
            return;
        }
    }
}

pub(crate) async fn ping_worker(state: Arc<state::App>, worker: state::Worker) {
    // Ping worker host /encoder
    let url = format!("{}://{}/encoder", worker.protocol, worker.host);
    let client = reqwest::Client::new();
    let response = client.get(&url).send().await;
    if response.is_err() {
        // Lock workers and update status
        let mut workers = state.workers.lock().await;
        for w in workers.iter_mut() {
            if w.id == worker.id {
                w.status = state::WorkerStatus::Crashed;
            }
            for s in state.streams.lock().await.iter_mut() {
                for o in s.output.iter_mut() {
                    if o.worker.is_some() && o.worker.unwrap() == worker.id {
                        tokio::spawn(stream::set_output_status(
                            state.clone(),
                            s.id,
                            o.id,
                            stream::Status::Creating,
                            None,
                        ));
                    }
                }
            }
        }

        log::warn!("Worker {} is down", worker.host);
        log::info!("{:?}", response.err());
        return;
    }

    // Response is ok
    // Encoder stats are in response, parse as JSON WorkerStats
    let worker_stats = serde_json::from_str(&response.unwrap().text().await.unwrap()).unwrap();

    // Check worker outputs
    tokio::spawn(check_worker_outputs(state.clone(), worker.clone()));

    // Lock workers and update status
    let mut workers = state.workers.lock().await;
    for w in workers.iter_mut() {
        if w.id == worker.id {
            w.status = state::WorkerStatus::Up;
            w.stats = worker_stats;
            return;
        }
    }
}

#[derive(serde::Deserialize, Clone)]
struct WorkerStream {
    id: Uuid,
    // status: String,
    name: String,
    input: String,
    output: String,
    codec: String,
    options: Option<Options>,
    // pid: Option<u32>,
}

fn compare_configs(worker_stream: WorkerStream, stream: stream::Stream, output: Output) -> bool {
    if worker_stream.codec != output.codec.ffmpeg_name()
        || worker_stream.input != stream.input
        || worker_stream.output != output.uri
        || worker_stream.name != stream.name
        || worker_stream.options != output.options
    {
        return true;
    }
    return false;
}

pub(crate) async fn check_worker_outputs(state: Arc<state::App>, worker: Worker) {
    // Get worker streams
    let url = format!("{}://{}/stream", worker.protocol, worker.host);
    let client = reqwest::Client::new();
    let response = client.get(&url).send().await;
    if response.is_err() {
        log::error!("Worker {} is down", worker.host);
        log::info!("{:?}", response.err());
        return;
    }

    let worker_streams: Vec<WorkerStream> =
        serde_json::from_str(&response.unwrap().text().await.unwrap()).unwrap();

    // Stream IDs in worker (represents lb stream output IDs)
    let mut worker_stream_ids: HashSet<Uuid> = HashSet::new();
    for worker_stream in worker_streams.clone() {
        worker_stream_ids.insert(worker_stream.id);
    }

    // Output IDs expected on worker
    let mut worker_output_ids: HashSet<Uuid> = HashSet::new();

    let streams = stream::get_all(state.clone()).await;
    for stream in streams {
        for output in stream.output.clone() {
            if output.worker.is_some() && output.worker.unwrap() == worker.id {
                worker_output_ids.insert(output.id);

                if !stream.enabled {
                    log::info!(
                        "Stream {} is disabled, stopping output {} on worker {}",
                        stream.id,
                        output.id,
                        worker.host
                    );
                    // Remove output worker
                    tokio::spawn(stream::set_output_status(
                        state.clone(),
                        stream.id,
                        output.id,
                        stream::Status::Creating,
                        None,
                    ));

                    // Delete stream output
                    tokio::spawn(worker::stop_stream(worker.clone(), stream.id, output.id));
                    continue;
                }

                // check if output.id is in worker_stream_ids
                let mut found = false;
                for worker_stream in worker_streams.clone() {
                    if worker_stream.id == output.id {
                        // check if there is a diff in codec, input, output
                        if compare_configs(worker_stream.clone(), stream.clone(), output.clone()) {
                            log::warn!(
                                "Stream {} output {} on worker {} has invalid configuration. Restarting",
                                stream.id,
                                output.id,
                                worker.host
                            );

                            tokio::spawn(stream::log(
                                state.clone(),
                                stream.id,
                                output.id,
                                format!(
                                    "Output has invalid configuration on worker {}. Restarting",
                                    worker.id
                                ),
                                stream::LogLevel::Info,
                            ));
                            tokio::spawn(worker::stop_stream(
                                worker.clone(),
                                stream.id.clone(),
                                output.id.clone(),
                            ));
                        } else {
                            found = true;
                        }
                        break;
                    }
                }

                if !found {
                    log::warn!(
                        "Stream {} output {} is not running on worker {}",
                        stream.id,
                        output.id,
                        worker.host
                    );

                    tokio::spawn(stream::log(
                        state.clone(),
                        stream.id,
                        output.id,
                        format!("Output is not running on worker {}. Restarting", worker.id),
                        stream::LogLevel::Error,
                    ));

                    tokio::spawn(stream::set_output_status(
                        state.clone(),
                        stream.id,
                        output.id,
                        stream::Status::Creating,
                        None,
                    ));
                }
            }
        }
    }

    // Stop any orphan streams in worker_stream_ids but not in worker_output_ids
    for orphan_id in worker_stream_ids.difference(&worker_output_ids) {
        log::info!(
            "Stopping orphan stream {} is running on worker {}",
            orphan_id,
            worker.host.clone()
        );

        tokio::spawn(worker::stop_stream(worker.clone(), Uuid::nil(), *orphan_id));
    }
}

pub(crate) async fn get_worker_capabilities(state: Arc<state::App>, worker: Worker) {
    // Get worker capabilities
    let url = format!("{}://{}/capabilities", worker.protocol, worker.host);
    let client = reqwest::Client::new();
    let response = client.get(&url).send().await;
    if response.is_err() {
        log::error!("Worker {} is down", worker.host);
        log::info!("{:?}", response.err());
        return;
    }

    #[derive(serde::Deserialize)]
    struct CapabilitiesResponse {
        codecs: Vec<stream::Codec>,
        encoder: Option<state::Encoder>,
    }
    let worker_capabilities: CapabilitiesResponse =
        serde_json::from_str(&response.unwrap().text().await.unwrap()).unwrap();

    // Lock workers and update capabilities
    let mut workers = state.workers.lock().await;
    for w in workers.iter_mut() {
        if w.id == worker.id {
            w.codecs = worker_capabilities.codecs;
            w.status = state::WorkerStatus::Up;
            if worker_capabilities.encoder.is_some() {
                w.encoder = worker_capabilities.encoder;
            }
            return;
        }
    }
}

pub(crate) async fn update_worker_status(state: Arc<state::App>) {
    let mut workers = state.workers.lock().await;
    for worker in workers.iter_mut() {
        tokio::spawn(get_worker_capabilities(state.clone(), worker.clone()));
        tokio::spawn(ping_worker(state.clone(), worker.clone()));
        tokio::spawn(get_worker_info(state.clone(), worker.clone()));
    }
}

pub(crate) async fn start_new_streams(state: Arc<state::App>) {
    let streams = stream::get_all(state.clone()).await;
    let workers = worker::get_all(state.clone()).await;

    for stream in streams {
        if !stream::check_all_outputs_running(state.clone(), stream.id).await {
            for output in stream.output.clone() {
                if output.status != stream::Status::Creating || !stream.enabled {
                    continue;
                }

                // find worker with least utilization which supports stream.output.codec
                let mut best_worker: Option<Worker> = None;

                // 100 should be max but let's use u32::MAX for now in case some worker manages to go over
                let mut best_utilization = u32::MAX;

                for worker in workers.clone() {
                    if worker.status != state::WorkerStatus::Up {
                        continue;
                    }

                    if worker.codecs.contains(&output.codec) {
                        if worker.stats.utilization < best_utilization {
                            best_worker = Some(worker.clone());
                            best_utilization = worker.stats.utilization;
                        } else if worker.stats.utilization == best_utilization {
                            // If utilization is the same, randomly choose one
                            if rand::random() {
                                best_worker = Some(worker.clone());
                            }
                        }
                    }
                }

                if best_worker.is_none() {
                    log::warn!(
                        "No worker available for stream {} output {}",
                        stream.id,
                        output.id
                    );
                    tokio::spawn(stream::log(
                        state.clone(),
                        stream.id,
                        output.id,
                        "No worker available for stream".to_string(),
                        stream::LogLevel::Error,
                    ));
                    continue;
                }

                worker::start_stream(best_worker.unwrap(), stream.clone(), state.clone(), output)
                    .await;
            }
        }
    }
}

pub(crate) async fn update_stream_state(state: Arc<state::App>) {
    let mut streams = state.streams.lock().await;

    // if all outputs are up, set stream as up
    for stream in streams.iter_mut() {
        if stream.status != stream::Status::Running {
            let mut all_up = true;
            for output in stream.output.clone() {
                if output.status != stream::Status::Running {
                    all_up = false;
                }
            }
            if all_up {
                stream.status = stream::Status::Running;
            }
        }
    }
}

pub(crate) async fn start(state: Arc<state::App>) {
    loop {
        let state_clone = state.clone();
        let _ = tokio::spawn(async move {
            monitor(state_clone).await;
        })
        .await;
        log::warn!("lb monitor thread has exited, restarting");
    }
}

// Thread to monitor the state of the system, create new streams and remove old ones depending on the app state
pub(crate) async fn monitor(state: Arc<state::App>) {
    // Restore state from file
    let loaded = load_state(state.clone()).await;
    let mut last_state: StateDump;

    if loaded.is_some() {
        last_state = loaded.unwrap();
    } else {
        last_state = state.dump().await;
    }

    load_worker_discovery(state.clone()).await;

    loop {
        let dump = state.dump().await;
        if dump != last_state {
            log::info!("State changed, updating state file");
            save_state(state.clone()).await;
            last_state = dump;
        }

        // Update worker status
        update_worker_status(state.clone()).await;

        // Check for new streams
        start_new_streams(state.clone()).await;

        // If all outputs are up, set stream as up
        update_stream_state(state.clone()).await;

        // sleep 10 seconds
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }
}
