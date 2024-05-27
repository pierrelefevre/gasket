use crate::stream::{Codec, Stream};
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub(crate) enum WorkerStatus {
    Configuring,
    Up,
    Crashed,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub(crate) struct WorkerStats {
    pub utilization: u32,
    pub devices: Vec<u32>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone, Copy, Eq)]
pub(crate) enum Encoder {
    NVENC,
    VideoToolbox,
    U30,
    NIT2A,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub(crate) struct Worker {
    pub(crate) id: Uuid,
    pub(crate) protocol: String,
    pub(crate) host: String,
    pub(crate) public_ip: Option<String>,
    pub(crate) udp_ports: Vec<u16>,
    pub(crate) codecs: Vec<Codec>,
    pub(crate) encoder: Option<Encoder>,
    pub(crate) status: WorkerStatus,
    pub(crate) stats: WorkerStats,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub(crate) struct StateDump {
    pub(crate) streams: Vec<Stream>,
    pub(crate) workers: Vec<Worker>,
}

pub(crate) struct App {
    pub(crate) streams: Mutex<Vec<Stream>>,
    pub(crate) workers: Mutex<Vec<Worker>>,
}
impl App {
    pub(crate) fn new() -> App {
        return App {
            streams: Mutex::new(Vec::new()),
            workers: Mutex::new(Vec::new()),
        };
    }

    pub(crate) async fn dump(&self) -> StateDump {
        let streams = self.streams.lock().await;
        let workers = self.workers.lock().await;
        return StateDump {
            streams: streams.clone(),
            workers: workers.clone(),
        };
    }
}
