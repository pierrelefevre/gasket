use crate::state::{self, Codec, Encoder};
use crate::transcode;
use crate::{args, encoder};
use clap::Parser;
use std::sync::Arc;
use tokio::sync::broadcast;

async fn set_server_running(state: &Arc<state::App>) {
    let mut status = state.server_status.lock().await;
    if *status != state::ServerStatus::Ready {
        log::info!("Stream monitor ready");
    }
    *status = state::ServerStatus::Ready;
}

async fn set_stream_exited(state: &Arc<state::App>, uuid: &uuid::Uuid, name: &String) {
    log::info!("Setting stream {} as Exited", name);
    let mut streams = state.streams.lock().await;
    for stream in streams.iter_mut() {
        if stream.id == *uuid {
            stream.status = state::StreamStatus::Exited;
        }
    }
}

async fn set_server_capabilities(
    state: &Arc<state::App>,
    enc: Option<Encoder>,
    codecs: Vec<state::Codec>,
) {
    let mut encoder = state.encoder.lock().await;
    *encoder = enc;
    let mut supported_codecs: tokio::sync::MutexGuard<'_, Vec<state::Codec>> =
        state.codecs.lock().await;
    *supported_codecs = codecs;
}

pub(crate) async fn start(state: Arc<state::App>) {
    loop {
        let state_clone = state.clone();
        tokio::spawn(async move {
            run(state_clone).await;
        })
        .await
        .unwrap();
        log::warn!("Stream monitor thread has exited, restarting");
    }
}

// Thread to monitor the state of the system, create new streams and remove old ones depending on the app state
pub(crate) async fn run(state: Arc<state::App>) {
    let args = args::Args::parse();

    let mut enc: Option<Encoder> = None;

    if !args.cpu_only {
        let codecs: Vec<Codec>;
        // set server capabilities
        (enc, codecs) = transcode::list_hardware_support();
        let state_clone = state.clone();
        tokio::spawn(async move {
            set_server_capabilities(&state_clone, enc, codecs).await;
        });
    } else {
        log::info!("--cpu-only provided: Forcing CPU encoding and ignoring any hardware encoders");
        let state_clone = state.clone();
        tokio::spawn(async move {
            set_server_capabilities(&state_clone, None, Codec::all()).await;
        });
    }

    loop {
        // Set status as running if it's not already
        set_server_running(&state).await;

        // Scope so we can release the lock on state.streams
        {
            let mut streams = state.streams.lock().await;

            // Remove streams that are exited
            streams.retain(|stream| stream.status != state::StreamStatus::Exited);

            // Update streams
            for stream in streams.iter_mut() {
                // Start the stream if it's waiting
                if stream.status == state::StreamStatus::Waiting {
                    // Start the stream
                    log::info!(
                        "Starting stream {}, with file {}",
                        stream.name,
                        stream.input
                    );

                    let (tx, rx) = broadcast::channel(256);
                    let mut rx_clone = tx.subscribe();

                    let encoder_stats = state.encoder_status.lock().await;

                    let pid = transcode::stream(
                        encoder_stats.clone(),
                        stream.id.clone(),
                        stream.name.clone(),
                        stream.input.clone(),
                        stream.output.clone(),
                        stream.codec.clone(),
                        stream.options.clone(),
                        tx,
                    )
                    .await;

                    stream.rx = Some(rx);
                    stream.pid = pid;
                    stream.status = state::StreamStatus::Running;

                    let stream_name = stream.name.clone();
                    let stream_uuid = stream.id.clone();

                    let state_clone = state.clone();
                    // Spawn a thread to read the log messages and log them
                    tokio::spawn(async move {
                        loop {
                            match rx_clone.recv().await {
                                Ok(msg) => {
                                    if msg.error > state::StreamLogLevel::Stderr {
                                        log::error!("Stream {}: {}", stream_name, msg.message);
                                        set_stream_exited(&state_clone, &stream_uuid, &stream_name)
                                            .await;
                                    } else {
                                        log::info!("Stream {}: {}", stream_name, msg.message);
                                    }
                                }
                                Err(e) => {
                                    log::info!("Stream {}: Error reading log: {}", stream_name, e);
                                    set_stream_exited(&state_clone, &stream_uuid, &stream_name)
                                        .await;
                                    break;
                                }
                            }
                        }
                    });

                    // Allow the stream to start before checking the next one, to avoid starting too many at once.
                    break;
                }

                // Update status of the stream if it's running
                if stream.status == state::StreamStatus::Running {
                    // Check if the process is still running
                    match stream.pid {
                        Some(pid_u32) => {
                            let pid = nix::unistd::Pid::from_raw(pid_u32 as i32);
                            match nix::sys::wait::waitpid(
                                pid,
                                Some(nix::sys::wait::WaitPidFlag::WNOHANG),
                            ) {
                                Ok(nix::sys::wait::WaitStatus::Exited(_, _)) => {
                                    log::info!("Stream {} has exited", stream.name);
                                    stream.status = state::StreamStatus::Exited;
                                }
                                _ => {}
                            }
                        }
                        None => {}
                    }
                }

                // Remove the stream if it's Stopping
                if stream.status == state::StreamStatus::Stopping {
                    log::info!("Monitor stopping stream {}", stream.name);
                    match stream.pid {
                        Some(pid_u32) => {
                            let pid = nix::unistd::Pid::from_raw(pid_u32 as i32);
                            match nix::sys::signal::kill(pid, nix::sys::signal::Signal::SIGTERM) {
                                Ok(_) => {
                                    log::info!("Stream {} killed", stream.name);
                                }
                                Err(e) => {
                                    log::error!("Error stopping stream {}: {}", stream.name, e);
                                }
                            }
                        }
                        None => {
                            log::error!("Stream {} has no PID", stream.name);
                        }
                    }

                    // Mark the stream as exited, we can't be 100% sure it's stopped but we'll assume it has
                    stream.status = state::StreamStatus::Exited;
                }
            }
        }

        // Log encoder status
        if enc == Some(Encoder::NVENC) {
            let stats = encoder::nvenc_stats();
            match stats {
                Ok(stats) => {
                    log::info!("NVENC stats: {:?}", stats);
                    let mut encoder_status = state.encoder_status.lock().await;
                    *encoder_status = stats;
                }
                Err(e) => {
                    log::error!("Error getting NVENC stats: {}", e);
                }
            }
        }

        if enc == Some(Encoder::U30) {
            let stats = encoder::xilinx_stats();
            match stats {
                Ok(stats) => {
                    log::info!("Xilinx xrmadm stats: {:?}", stats);
                    let mut encoder_status = state.encoder_status.lock().await;
                    *encoder_status = stats;
                }
                Err(e) => {
                    log::error!("Error getting Xilinx stats: {}", e);
                }
            }
        }

        if enc == Some(Encoder::NIT2A) {
            let stats = encoder::netint_stats();
            match stats {
                Ok(stats) => {
                    log::info!("Netint stats: {:?}", stats);
                    let mut encoder_status = state.encoder_status.lock().await;
                    *encoder_status = stats;
                }
                Err(e) => {
                    log::error!("Error getting Netint stats: {}", e);
                }
            }
        }

        if enc.is_none() {
            // set utilization to 100 + 10 * stream count to ensure not all streams are started on the same worker
            let mut encoder_status = state.encoder_status.lock().await;
            encoder_status.utilization = 100 + 10 * state.streams.lock().await.len() as u32;
        }

        // Sleep for a while
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    }
}
