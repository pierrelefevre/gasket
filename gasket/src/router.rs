use crate::state;
use crate::state::StreamOptions;
use crate::utils;
use axum::{extract::Path, extract::State, http::StatusCode, response::Json};
use serde_json::{json, Value};
use std::sync::Arc;
use uuid::Uuid;

// GET /
// Return server build version, number of current streams
pub(crate) async fn index(State(data): State<Arc<state::App>>) -> Json<Value> {
    // get number of streams
    let streams_list = data.streams.lock().await;

    return Json(json!({
        "server": format!("gasket {}", utils::get_build_info()),
        "streams": streams_list.len(),
    }));
}
// GET /stream
// Get current streams
pub(crate) async fn get_streams(State(data): State<Arc<state::App>>) -> Json<Value> {
    let streams = data.streams.lock().await;

    let mut response = Vec::new();
    for stream in streams.iter() {
        let stream_info = json!({
            "id": stream.id.to_string(),
            "name": stream.name,
            "input": stream.input,
            "output": stream.output,
            "codec": stream.codec,
            "status": format!("{:?}", stream.status),
            "options": stream.options,
            "pid": stream.pid.unwrap_or(0),
        });

        response.push(stream_info);
    }

    Json(json!(response))
}

// POST /stream
// Create new stream
#[derive(serde::Deserialize)]
pub(crate) struct AddStreamParams {
    id: Option<Uuid>,
    name: String,
    input: String,
    output: String,
    codec: String,
    options: Option<StreamOptions>,
}
pub(crate) async fn create_stream(
    State(data): State<Arc<state::App>>,
    Json(payload): Json<AddStreamParams>,
) -> Result<Json<Value>, StatusCode> {
    let mut streams_list = data.streams.lock().await;

    // If stream with same ID already exists, return 204
    for stream in streams_list.iter() {
        if stream.id.to_string() == payload.id.unwrap_or(Uuid::new_v4()).to_string() {
            let stream_info = state::StreamInfo::from(stream);
            return Ok(Json(json!(stream_info)));
        }
    }

    let stream = state::Stream {
        name: payload.name,
        id: payload.id.unwrap_or(Uuid::new_v4()),
        input: payload.input,
        output: payload.output,
        codec: payload.codec,
        options: payload.options,
        status: state::StreamStatus::Waiting,
        pid: None,
        rx: None,
    };
    let stream_info = state::StreamInfo::from(&stream);
    streams_list.push(stream);

    log::info!("Stream added");
    return Ok(Json(json!(stream_info)));
}

// DELETE /stream/:uuid
// Remove stream  by setting the stream's status to StreamStatus::Stopping, the monitor thread will take care of the rest
pub(crate) async fn delete_stream(
    State(data): State<Arc<state::App>>,
    Path(uuid): Path<String>,
) -> Result<Json<Value>, StatusCode> {
    log::info!("Requesting delete of stream {}", uuid);

    let mut streams_list = data.streams.lock().await;

    let mut found: Option<&state::Stream> = None;

    for stream in streams_list.iter_mut() {
        if stream.id.to_string() == uuid.to_string() {
            stream.status = state::StreamStatus::Stopping;
            found = Some(&*stream);
            break;
        }
    }

    match found {
        Some(stream) => {
            let stream_info = Some(state::StreamInfo::from(stream));
            log::info!("Stream {} marked as Stopping", uuid);
            return Ok(Json(json!(stream_info)));
        }
        None => {
            return Err(StatusCode::NOT_FOUND);
        }
    }
}

// GET /encoder
// Return encoder status
pub(crate) async fn get_encoder_status(State(data): State<Arc<state::App>>) -> Json<Value> {
    let encoder_status = data.encoder_status.lock().await;

    return Json(json!(*encoder_status));
}

// GET /capabilities
// Return encoder capabilities
pub(crate) async fn get_capabilities(State(data): State<Arc<state::App>>) -> Json<Value> {
    let codecs = data.codecs.lock().await;
    let encoder = data.encoder.lock().await;

    if encoder.is_none() {
        return Json(json!({"codecs": &**codecs}));
    }
    return Json(json!({"encoder": encoder.unwrap(), "codecs": *codecs}));
}

// GET /livez
// Check if the server is alive
pub(crate) async fn livez() -> &'static str {
    "ok"
}

// GET /readyz
// Check if the server is ready (Set when the monitor thread is up and running)
pub(crate) async fn readyz(State(data): State<Arc<state::App>>) -> Result<String, StatusCode> {
    let status = data.server_status.lock().await;

    if *status == state::ServerStatus::Ready {
        return Ok("ok".to_string());
    } else {
        return Err(StatusCode::PROCESSING);
    }
}
