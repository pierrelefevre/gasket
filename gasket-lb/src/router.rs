use std::sync::Arc;

use axum::extract::Path;
use axum::http::StatusCode;
use axum::{extract::State, Json};
use clap::Parser;
use json_patch::merge;
use serde_json::{from_value, json, Error, Value};

use crate::stream::Stream;
use crate::{args, state};
use crate::{stream, utils};

pub(crate) async fn index(State(data): State<Arc<state::App>>) -> Json<Value> {
    // get number of streams
    let streams_list = data.streams.lock().await;
    let workers_list = data.workers.lock().await;

    let state_file = args::Args::parse().state_file;

    return Json(json!({
        "server": format!("gasket-lb {}", utils::get_build_info()),
        "streams": streams_list.len(),
        "workers": workers_list.len(),
        "state_file": state_file,
    }));
}

pub(crate) async fn reset_state(State(data): State<Arc<state::App>>) -> StatusCode {
    let mut streams_list = data.streams.lock().await;
    let mut workers_list = data.workers.lock().await;

    streams_list.clear();
    workers_list.clear();

    return StatusCode::NO_CONTENT;
}

// GET /stream

pub(crate) async fn get_all_streams(State(data): State<Arc<state::App>>) -> Json<Value> {
    let streams_list = data.streams.lock().await;

    return Json(json!(streams_list.clone()));
}

// GET /stream:uuid
pub(crate) async fn get_stream(
    State(data): State<Arc<state::App>>,
    Path(uuid): Path<String>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let streams_list = data.streams.lock().await;

    let stream = streams_list
        .iter()
        .find(|x| x.id.to_string() == uuid)
        .cloned();
    if stream.is_none() {
        return Err((
            StatusCode::NOT_FOUND,
            format!("Stream with id {} not found", uuid),
        ));
    }

    return Ok(Json(json!(stream.unwrap())));
}

// POST /stream
#[derive(serde::Deserialize)]
pub(crate) struct CreateStreamOutput {
    uri: String,
    codec: stream::Codec,
    options: Option<stream::Options>,
}
#[derive(serde::Deserialize)]
pub(crate) struct CreateStream {
    name: String,
    input: String,
    output: Vec<CreateStreamOutput>,
}
pub(crate) async fn create_stream(
    State(data): State<Arc<state::App>>,
    Json(payload): Json<CreateStream>,
) -> Json<Value> {
    let mut streams_list = data.streams.lock().await;

    let new_stream = stream::Stream {
        id: uuid::Uuid::new_v4(),
        name: payload.name,
        input: payload.input,
        output: payload
            .output
            .iter()
            .map(|x| stream::Output {
                id: uuid::Uuid::new_v4(),
                uri: x.uri.clone(),
                codec: x.codec.clone(),
                options: x.options.clone(),
                status: stream::Status::Creating,
                worker: None,
                logs: Vec::new(),
                last_error: None,
            })
            .collect(),
        enabled: true,
        status: stream::Status::Creating,
    };

    streams_list.push(new_stream.clone());

    return Json(json!(new_stream));
}

// PATCH /stream:uuid
pub(crate) async fn patch_stream(
    State(data): State<Arc<state::App>>,
    Path(uuid): Path<String>,
    Json(patch): Json<Value>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let mut streams_list = data.streams.lock().await;

    let index = streams_list.iter().position(|x| x.id.to_string() == uuid);
    if index.is_none() {
        return Err((
            StatusCode::NOT_FOUND,
            format!("Stream with id {} not found", uuid),
        ));
    }

    let stream = streams_list[index.unwrap()].clone();

    // if patch.output, check if patch.output[...].id exists, otherwise create new with uuid::Uuid::new_v4()
    let mut patch_clone = patch.clone();
    if let Some(outputs) = patch_clone.get_mut("output") {
        if outputs.is_array() {
            for output in outputs.as_array_mut().unwrap() {
                if output.get("id").is_none() {
                    output["id"] = json!(uuid::Uuid::new_v4().to_string());
                }
                if output.get("status").is_none() {
                    output["status"] = json!("Creating");
                }
                if output.get("logs").is_none() {
                    output["logs"] = json!([]);
                }
            }
        }
    }

    let mut stream_json = serde_json::to_value(&stream).unwrap();
    merge(&mut stream_json, &patch_clone);

    let new_stream: Result<Stream, Error> = from_value(stream_json.clone());

    if new_stream.is_err() {
        let err_msg = format!(
            "Error parsing stream json\nReason: {:?}",
            new_stream.unwrap_err()
        );
        log::info!("{err_msg}");
        return Err((StatusCode::BAD_REQUEST, err_msg));
    }

    streams_list[index.unwrap()] = new_stream.unwrap().clone();

    return Ok(Json(stream_json));
}

// DELETE /stream:uuid
pub(crate) async fn delete_stream(
    State(data): State<Arc<state::App>>,
    Path(uuid): Path<String>,
) -> Result<StatusCode, (StatusCode, String)> {
    let mut streams_list = data.streams.lock().await;

    let index = streams_list.iter().position(|x| x.id.to_string() == uuid);
    if index.is_none() {
        return Err((
            StatusCode::NOT_FOUND,
            format!("Stream with id {} not found", uuid),
        ));
    }

    streams_list.remove(index.unwrap());

    return Ok(StatusCode::NO_CONTENT);
}

// GET /worker
pub(crate) async fn get_all_workers(State(data): State<Arc<state::App>>) -> Json<Value> {
    let workers_list = data.workers.lock().await;

    return Json(json!(workers_list.clone()));
}

// GET /worker:uuid
pub(crate) async fn get_worker(
    State(data): State<Arc<state::App>>,
    Path(uuid): Path<String>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let workers_list = data.workers.lock().await;

    let worker = workers_list
        .iter()
        .find(|x| x.id.to_string() == uuid)
        .cloned();
    if worker.is_none() {
        return Err((
            StatusCode::NOT_FOUND,
            format!("Worker with id {} not found", uuid),
        ));
    }

    return Ok(Json(json!(worker.unwrap())));
}

// POST /worker
#[derive(serde::Deserialize)]
pub(crate) struct CreateWorker {
    protocol: Option<String>,
    host: String,
    public_ip: Option<String>,
}
pub(crate) async fn create_worker(
    State(data): State<Arc<state::App>>,
    Json(payload): Json<CreateWorker>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let new_worker = state::Worker {
        id: uuid::Uuid::new_v4(),
        // payload.protocol or "http"
        protocol: payload.protocol.unwrap_or("http".to_string()),
        host: payload.host,
        public_ip: payload.public_ip,
        udp_ports: Vec::new(),
        codecs: Vec::new(),
        encoder: None,
        status: state::WorkerStatus::Configuring,
        stats: state::WorkerStats {
            utilization: 100,
            devices: Vec::new(),
        },
        server: None,
        streams: None,
    };

    let mut workers_list = data.workers.lock().await;

    // Check if worker already exists
    if let Some(existing_worker) = workers_list.iter().find(|x| x.host == new_worker.host) {
        return Err((
            StatusCode::CONFLICT,
            format!(
                "Worker with host {} already exists with id: {}",
                new_worker.host, existing_worker.id
            ),
        ));
    }
    workers_list.push(new_worker.clone());

    return Ok(Json(json!(new_worker)));
}

// PATCH /worker:uuid
pub(crate) async fn patch_worker(
    State(data): State<Arc<state::App>>,
    Path(uuid): Path<String>,
    Json(patch): Json<Value>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let mut workers_list = data.workers.lock().await;

    let index = workers_list.iter().position(|x| x.id.to_string() == uuid);
    if index.is_none() {
        return Err((
            StatusCode::NOT_FOUND,
            format!("Worker with id {} not found", uuid),
        ));
    }
    let worker = workers_list[index.unwrap()].clone();

    let mut worker_json = serde_json::to_value(&worker).unwrap();
    merge(&mut worker_json, &patch);

    let new_worker: state::Worker = from_value(worker_json.clone()).unwrap();
    workers_list[index.unwrap()] = new_worker.clone();

    return Ok(Json(worker_json));
}

// DELETE /worker:uuid
pub(crate) async fn delete_worker(
    State(data): State<Arc<state::App>>,
    Path(uuid): Path<String>,
) -> Result<StatusCode, (StatusCode, String)> {
    let mut workers_list = data.workers.lock().await;

    let index = workers_list.iter().position(|x| x.id.to_string() == uuid);
    if index.is_none() {
        return Err((
            StatusCode::NOT_FOUND,
            format!("Worker with id {} not found", uuid),
        ));
    }

    workers_list.remove(index.unwrap());
    log::info!("Worker with id {} deleted", uuid);
    return Ok(StatusCode::NO_CONTENT);
}

// GET /livez
// Check if the server is alive
pub(crate) async fn livez() -> &'static str {
    "ok"
}

// GET /readyz
// Check if the server is ready
pub(crate) async fn readyz() -> &'static str {
    "ok"
}
