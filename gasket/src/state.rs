use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;
use uuid::Uuid;

///
///
/// Stream state
///

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub(crate) enum StreamStatus {
    Waiting,
    Running,
    Stopping,
    Exited,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub(crate) struct StreamOptions {
    pub(crate) pixel_format: Option<String>,
    pub(crate) bitrate: Option<String>,
    pub(crate) framerate: Option<String>,
    pub(crate) gop_size: Option<String>,
    pub(crate) debug_text: Option<bool>,
    pub(crate) output_format: Option<String>,
}

// Internal stream state
pub(crate) struct Stream {
    // Name of the stream
    pub(crate) name: String,

    // UUID of the stream
    pub(crate) id: Uuid,

    // Input URI (udp://host_ip:port)
    pub(crate) input: String,

    // Output URI (udp://host_ip:port)
    pub(crate) output: String,

    // Codec
    pub(crate) codec: String,

    // Options
    pub(crate) options: Option<StreamOptions>,

    // Status (waiting, running, stopping, exited, etc.)
    pub(crate) status: StreamStatus,

    // PID of the stream/transcode process
    pub(crate) pid: Option<u32>,

    // Log channel
    pub(crate) rx: Option<tokio::sync::broadcast::Receiver<StreamLogMessage>>,
}

// DTO/Clonable Stream info
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub(crate) struct StreamInfo {
    pub(crate) id: Uuid,
    pub(crate) name: String,
    pub(crate) input: String,
    pub(crate) output: String,
    pub(crate) codec: String,
    pub(crate) status: StreamStatus,
    pub(crate) pid: Option<u32>,
}

// Convert Stream to StreamInfo
impl From<&Stream> for StreamInfo {
    fn from(stream: &Stream) -> StreamInfo {
        return StreamInfo {
            id: stream.id,
            name: stream.name.clone(),
            input: stream.input.clone(),
            output: stream.output.clone(),
            codec: stream.codec.clone(),
            status: stream.status.clone(),
            pid: stream.pid,
        };
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd)]
pub(crate) enum StreamLogLevel {
    Stdout,
    Stderr,
    Exit,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub(crate) struct StreamLogMessage {
    pub(crate) stream_id: Uuid,
    pub(crate) message: String,
    pub(crate) error: StreamLogLevel,
}

///
///
/// Encoder state
///

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub(crate) struct EncoderStats {
    pub utilization: u32,
    pub devices: Vec<u32>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone, Copy)]
pub(crate) enum Encoder {
    NVENC,
    VideoToolbox,
    U30,
    NIT2A,
}
impl Encoder {
    pub fn all() -> Vec<Encoder> {
        return vec![
            Encoder::NVENC,
            Encoder::VideoToolbox,
            Encoder::U30,
            Encoder::NIT2A,
        ];
    }

    pub fn ffmpeg_name(&self) -> &'static str {
        match self {
            Encoder::NVENC => "nvenc",
            Encoder::VideoToolbox => "videotoolbox",
            Encoder::U30 => "mpsoc_vcu",
            Encoder::NIT2A => "ni_quadra",
        }
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone, Copy)]
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
    pub fn all() -> Vec<Codec> {
        return vec![Codec::H264, Codec::H265, Codec::AV1];
    }
}

///
///
/// App state
///

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub(crate) enum ServerStatus {
    Starting,
    Ready,
}

pub(crate) struct App {
    // Streams (array)
    pub(crate) streams: Mutex<Vec<Stream>>,
    pub(crate) server_status: Mutex<ServerStatus>,
    pub(crate) encoder_status: Mutex<EncoderStats>,
    pub(crate) encoder: Mutex<Option<Encoder>>,
    pub(crate) codecs: Mutex<Vec<Codec>>,
}

pub(crate) fn new_app() -> App {
    return App {
        streams: Mutex::new(Vec::new()),
        server_status: Mutex::new(ServerStatus::Starting),
        encoder_status: Mutex::new(EncoderStats {
            utilization: 100,
            devices: Vec::new(),
        }),
        encoder: Mutex::new(None),
        codecs: Mutex::new(Vec::new()),
    };
}
