use crate::{
    state::{self, Worker},
    stream::{self, Output, Stream},
};
use reqwest::Client;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Debug, serde::Serialize)]
struct StartStreamData {
    id: Uuid,
    name: String,
    input: String,
    output: String,
    codec: String,
    options: Option<stream::Options>,
}

pub(crate) async fn start_stream(
    worker: Worker,
    stream: Stream,
    state: Arc<state::App>,
    output: Output,
) {
    log::info!("Starting stream {} on worker {}", stream.name, worker.host);
    // Start stream on best_worker
    let url = format!("{}://{}/stream", worker.protocol, worker.host);
    let client = Client::new();

    // pick out name, input, output, codec from stream
    // todo here
    let start_stream_data = StartStreamData {
        id: output.id,
        name: stream.name.clone(),
        input: stream.input.clone(),
        output: output.uri.clone(),
        codec: output.codec.clone().ffmpeg_name().to_string(),
        options: output.options.clone(),
    };

    let response = client
        .post(&url)
        .json(&start_stream_data)
        .send()
        .await
        .expect("Error starting stream");

    // if response is ok, set the worker as the stream's worker, and set the stream as up
    if response.status().is_success() {
        log::info!(
            "Stream {} output {} started on worker {}",
            stream.name,
            output.id,
            worker.host
        );

        tokio::spawn(stream::set_output_status(
            state.clone(),
            stream.id,
            output.id,
            stream::Status::Running,
            Some(worker.id),
        ));
    } else {
        log::error!(
            "Stream {} output {} failed to start on worker {}",
            stream.name,
            output.id,
            worker.host
        );
        log::error!("{:?}", response.text().await.unwrap());
    }
}

pub(crate) async fn stop_stream(worker: Worker, stream_id: Uuid, output_id: Uuid) {
    if stream_id != Uuid::nil() {
        log::info!("Stopping stream {} on worker {}", stream_id, worker.host);
    }

    // Start stream on best_worker
    let url = format!("{}://{}/stream/{}", worker.protocol, worker.host, output_id);
    let client = Client::new();

    let response = client
        .delete(&url)
        .send()
        .await
        .expect("Error stopping stream");

    if response.status().is_success() {
        let log_message = if stream_id != Uuid::nil() {
            format!(
                "Stream {} output {} stopped on worker {}",
                stream_id, output_id, worker.host
            )
        } else {
            format!("Output {} stopped on worker {}", output_id, worker.host)
        };

        log::info!("{}", log_message);
    } else {
        let log_message = if stream_id != Uuid::nil() {
            format!(
                "Stream {} output {} failed to stop on worker {}",
                stream_id, output_id, worker.host
            )
        } else {
            format!(
                "Output {} failed to stop on worker {}",
                output_id, worker.host
            )
        };
        log::error!("{}", log_message);
        log::error!("{:?}", response.text().await.unwrap());
    }
}

pub(crate) async fn get_all(state: Arc<state::App>) -> Vec<Worker> {
    let workers = state.workers.lock().await;
    return workers.clone();
}

// pub(crate) async fn by_id(state: Arc<state::App>, worker_id: Uuid) -> Option<Worker> {
//     let workers = state.workers.lock().await;
//     return workers.iter().find(|x| x.id == worker_id).cloned();
// }
