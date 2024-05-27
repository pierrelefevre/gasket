use crate::state::App;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub(crate) enum Status {
    Creating,
    Running,
    Stopping,
    Finished,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Copy)]
pub(crate) enum Codec {
    H264,
    H265,
    AV1,
}
impl Codec {
    pub fn ffmpeg_name(&self) -> &'static str {
        match self {
            Codec::H264 => "h264",
            Codec::H265 => "hevc",
            Codec::AV1 => "av1",
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub(crate) struct Options {
    pub(crate) pixel_format: Option<String>,
    pub(crate) bitrate: Option<String>,
    pub(crate) framerate: Option<String>,
    pub(crate) gop_size: Option<String>,
    pub(crate) debug_text: Option<bool>,
    pub(crate) output_format: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub(crate) struct Output {
    pub(crate) id: Uuid,
    pub(crate) uri: String,
    pub(crate) codec: Codec,
    pub(crate) options: Option<Options>,
    pub(crate) status: Status,
    pub(crate) worker: Option<Uuid>,
    pub(crate) logs: Vec<String>,
    pub(crate) last_error: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Stream {
    pub(crate) id: Uuid,
    pub(crate) name: String,
    pub(crate) input: String,
    pub(crate) output: Vec<Output>,
    pub(crate) enabled: bool,
    pub(crate) status: Status,
}

//
// Small util functions to update the state
// Seems to be the meta so we don't lock the state too long in the big monitor functions
//

pub(crate) async fn get_all(state: Arc<App>) -> Vec<Stream> {
    let streams_list = state.streams.lock().await;
    return streams_list.clone();
}

// pub(crate) async fn by_id(state: Arc<App>, stream_id: Uuid) -> Option<Stream> {
//     let streams_list = state.streams.lock().await;
//     return streams_list.iter().find(|x| x.id == stream_id).cloned();
// }

pub(crate) async fn set_output_status(
    state: Arc<App>,
    stream_id: Uuid,
    output_id: Uuid,
    status: Status,
    worker_id: Option<Uuid>,
) {
    let mut streams_list = state.streams.lock().await;

    if let Some(stream) = streams_list.iter_mut().find(|x| x.id == stream_id) {
        if let Some(output) = stream.output.iter_mut().find(|x| x.id == output_id) {
            output.worker = worker_id;
            output.status = status;

            if output.status == Status::Running {
                tokio::spawn(log(
                    state.clone(),
                    stream_id,
                    output_id,
                    "Output is running".to_string(),
                    LogLevel::Info,
                ));
            }
        }
    }
}

pub(crate) async fn check_all_outputs_running(state: Arc<App>, stream_id: Uuid) -> bool {
    let mut streams_list = state.streams.lock().await;

    if let Some(stream) = streams_list.iter_mut().find(|x| x.id == stream_id) {
        for output in stream.output.iter() {
            if output.status != Status::Running {
                stream.status = Status::Creating;
                return false;
            }
        }

        stream.status = Status::Running;
    }

    true
}

#[derive(Debug, Deserialize, Serialize, PartialOrd, Ord, PartialEq, Eq, Clone, Copy)]
pub(crate) enum LogLevel {
    Debug,
    Info,
    Error,
}

pub(crate) async fn log(
    state: Arc<App>,
    stream_id: Uuid,
    output_id: Uuid,
    log: String,
    level: LogLevel,
) {
    let mut streams_list = state.streams.lock().await;

    if let Some(stream) = streams_list.iter_mut().find(|x| x.id == stream_id) {
        if let Some(output) = stream.output.iter_mut().find(|x| x.id == output_id) {
            // if last log is the same as the current log, don't add it
            if let Some(last_log) = output.logs.last() {
                if last_log == &log {
                    return;
                }
            }

            // prepent iso timestamp
            let dated_log = format!("{}: {}", chrono::Utc::now().to_rfc3339(), log);

            output.logs.push(dated_log.clone());
            if level > LogLevel::Info {
                output.last_error = Some(dated_log.clone());
            }
        }
    }
}
