use crate::args;
use crate::state::{Codec, Encoder, EncoderStats, StreamLogLevel, StreamLogMessage, StreamOptions};
use crate::utils::{self, get_ffmpeg_path};
use clap::Parser;
use log;
use std::env;
use std::process::{Command, Stdio};
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command as TokioCommand;
use uuid::Uuid;

pub(crate) fn check_ffmpeg_installed() -> String {
    let custom_path = get_ffmpeg_path();
    if custom_path != "" {
        return custom_path;
    }
    let mut command = Command::new("which");
    command.arg("ffmpeg");
    return utils::run_command_capture(command);
}

pub(crate) fn list_hardware_encoders() -> bool {
    let mut command = Command::new(get_ffmpeg_path());
    command.arg("-hide_banner").arg("-encoders");

    return utils::run_command(command);
}

pub(crate) fn get_encoder(filtered: Vec<&str>, encoder_name: &str) -> String {
    let os_filtered = filtered
        .iter()
        .filter(|line| line.contains(encoder_name))
        .collect::<Vec<&&str>>();
    if os_filtered.len() == 0 {
        log::info!("Could not find {} encoder", encoder_name);
        return "".to_string();
    }

    let encoder: String = os_filtered[0].split(" ").collect::<Vec<&str>>()[1].to_string();
    log::info!("Using {} encoder: {}", encoder_name, encoder);
    return encoder;
}

// List hardware encoders, depending on OS choose the hw encoder as string
pub(crate) fn choose_encoder(codec: String) -> String {
    let args = args::Args::parse();

    if args.cpu_only {
        let c = codec.clone();
        match c.as_str() {
            "h264" => return "libx264".to_string(),
            "hevc" => return "libx265".to_string(),
            "av1" => return "libaom-av1".to_string(),
            _ => {
                return "".to_string();
            }
        }
    }

    let mut command = Command::new(get_ffmpeg_path());
    command.arg("-hide_banner").arg("-encoders");

    let result = utils::run_command_capture(command);

    // split result per lines
    let lines: Vec<&str> = result.split("\n").collect();

    // filter lines that contain the codec
    let filtered = lines
        .iter()
        .filter(|line| line.contains(&codec))
        .map(|line| line.trim())
        .collect::<Vec<&str>>();

    // if no codec is found, return empty string
    if filtered.len() == 0 {
        return "".to_string();
    }

    let os = env::consts::OS;
    log::info!("Using OS {}", os); // Prints the current OS.

    // match on OS
    match os {
        "macos" => {
            log::info!("Checking presence of videotoolbox for codec: {}", codec);
            let encoder = get_encoder(filtered, "videotoolbox");
            return encoder;
        }
        "linux" => {
            let encoder = get_encoder(filtered.clone(), "nvenc");
            if encoder != "" {
                log::info!("Found nvenc {}", codec);
                return encoder;
            }

            let encoder = get_encoder(filtered.clone(), "mpsoc_vcu");
            if encoder != "" {
                log::info!("Found u30 {}", codec);
                return encoder;
            }

            let encoder = get_encoder(filtered.clone(), "ni_quadra");
            if encoder != "" {
                log::info!("Found netint {}", codec);
                return encoder;
            }

            log::error!("No hardware encoder found for codec: {}", codec);
            return "".to_string();
        }
        _ => {
            return "".to_string();
        }
    }
}

pub(crate) fn list_hardware_support() -> (Option<Encoder>, Vec<Codec>) {
    // for each codec, check, run choose_encoder(codec) and see if we get something back
    let codecs = Codec::all();
    let mut supported: Vec<Codec> = Vec::new();
    let mut hw_encoder: Option<Encoder> = None;

    for codec in codecs.iter() {
        let encoder = choose_encoder(codec.ffmpeg_name().to_string());

        if encoder == "" {
            continue;
        }

        for hw_encoder_enum in Encoder::all().iter() {
            if encoder.contains(hw_encoder_enum.ffmpeg_name()) {
                hw_encoder = Some(hw_encoder_enum.clone());
            }
        }

        supported.push(codec.clone());
    }
    return (hw_encoder, supported);
}

pub(crate) async fn stream(
    encoder_status: EncoderStats,
    uuid: Uuid,
    name: String,
    input: String,
    output: String,
    codec: String,
    options: Option<StreamOptions>,
    tx: tokio::sync::broadcast::Sender<StreamLogMessage>,
) -> Option<u32> {
    // unwrap options, set default values if None
    let pixel_format = options
        .as_ref()
        .and_then(|o| o.pixel_format.clone());
    let bitrate = options
        .as_ref()
        .and_then(|o| o.bitrate.clone())
        .unwrap_or("500k".to_string());
    let framerate = options.as_ref().and_then(|o| o.framerate.clone());
    let gop_size = options
        .as_ref()
        .and_then(|o| o.gop_size.clone())
        .unwrap_or("60".to_string());
    let debug_text = options
        .as_ref()
        .and_then(|o| o.debug_text.clone())
        .unwrap_or(true);
    let output_format = options
        .as_ref()
        .and_then(|o| o.output_format.clone())
        .unwrap_or("mpegts".to_string());

    let encoder = choose_encoder(codec);

    let mut hwdev = "";
    if encoder.contains("mpsoc_vcu") {
        hwdev = "u30";
    }
    if encoder.contains("ni_quadra") {
        hwdev = "ni";
    }

    let mut command = TokioCommand::new(get_ffmpeg_path());

    // Less verbose output
    command.arg("-hide_banner");

    // Pick device with lowest usage
    // TODO also set codec for input video
    // ffmpeg -xlnx_hwdev 1 -c:v mpsoc_vcu_h264 -stream_loop -1 -i test_loop.mp4 -f mp4 -c:v mpsoc_vcu_hevc -y /dev/null
    if hwdev == "u30" {
        command.arg("-xlnx_hwdev").arg(
            encoder_status
                .devices
                .iter()
                .enumerate()
                .min_by_key(|&(_, &value)| value)
                .expect("No devices in stats list")
                .0
                .to_string(),
        );
    }

    // Loop input
    command.arg("-stream_loop").arg("-1");

    // Set incoming framerate
    command.arg("-re");

    if framerate.is_some() {
        command.arg("-r").arg(framerate.clone().unwrap());
    }

    if hwdev == "ni" {
        command
            .arg("-c:v")
            .arg("h264_ni_quadra_dec")
            .arg("-xcoder-params")
            .arg("out=hw");
    }

    command
        // Input stream
        .arg("-i")
        .arg(input);

    command.arg("-b:v").arg(bitrate);

    if hwdev == "ni" {
        command.arg("-noautoscale");
    }

    // if encoder is not empty, use it
    if encoder != "" {
        command.arg("-c:v").arg(encoder);
    }

    // Chroma subsampling
    if pixel_format.is_some(){
        command.arg("-pix_fmt").arg(pixel_format.unwrap());
    }

    if framerate.is_some() {
        command.arg("-r").arg(framerate.clone().unwrap());
    }

    // Gop size (keyframe interval)
    command
        .arg("-g")
        .arg(gop_size)
        // Output format
        .arg("-f")
        .arg(output_format.clone());

    if output_format == "mpegts" {
        command
            .arg("-metadata")
            .arg("service_provider=gasket-".to_string() + &utils::get_build_info())
            .arg("-metadata")
            .arg("service_name=".to_string() + &name.to_string());
    }

    // Video filter for debug text
    if debug_text {
        if hwdev == "ni" {
            command.arg("-filter_complex")
            .arg(["ni_quadra_drawtext=text='%{localtime\\:%Y-%m-%d %H\\\\\\:%M\\\\\\:%S}':fontcolor=yellow:fontsize=100:x=10:y=10:box=1:boxcolor=black@0.5:boxborderw=5,ni_quadra_drawtext=text='gasket ", 
                &utils::get_build_info().replace(":", "\\:"),
                "':fontcolor=white:fontsize=50:x=W-tw-10:y=H-th-10:box=1:boxcolor=black@0.5:boxborderw=5"].concat());
        } else {
            command.arg("-vf")
            .arg(["drawtext=text='%{localtime\\:%Y-%m-%d %H\\\\\\:%M\\\\\\:%S}':fontcolor=yellow:fontsize=100:x=10:y=10:box=1:boxcolor=black@0.5:boxborderw=5,drawtext=text='gasket ", 
                &utils::get_build_info().replace(":", "\\:"),
                "':fontcolor=white:fontsize=50:x=W-tw-10:y=H-th-10:box=1:boxcolor=black@0.5:boxborderw=5"].concat());
        }
    }

    // Output to destination
    command.arg(output);

    // get command as string, for logging
    log::info!("Running command: {:?}", command);

    command.stdout(Stdio::piped());
    command.stderr(Stdio::piped());

    let mut child = command.spawn().expect("failed to spawn ffmpeg process");
    let stdout = child.stdout.take().expect("failed to open stdout");
    let stderr = child.stderr.take().expect("failed to open stderr");

    let mut reader_out = BufReader::new(stdout).lines();
    let mut reader_err = BufReader::new(stderr).lines();

    let tx_exit = tx.clone();
    let pid = child.id();

    log::info!("Stream {} started with PID: {:?}", uuid, pid);

    tokio::spawn(async move {
        let status = child.wait().await.expect("failed to wait on child");
        log::info!("Stream {} exited with status: {}", uuid, status);
        tx_exit
            .send(StreamLogMessage {
                stream_id: uuid,
                message: format!("Stream {} exited with status: {}", uuid, status),
                error: StreamLogLevel::Stdout,
            })
            .unwrap();
    });

    // Send stdout and stderr to the tx broadcaster
    let tx_out = tx.clone();
    tokio::spawn(async move {
        while let Some(line) = reader_out.next_line().await.unwrap() {
            tx_out
                .send(StreamLogMessage {
                    stream_id: uuid,
                    message: line,
                    error: StreamLogLevel::Stdout,
                })
                .unwrap();
        }
    });

    let tx_err = tx.clone();
    tokio::spawn(async move {
        while let Some(line) = reader_err.next_line().await.unwrap() {
            tx_err
                .send(StreamLogMessage {
                    stream_id: uuid,
                    message: line,
                    error: StreamLogLevel::Stderr,
                })
                .unwrap();
        }
    });

    // Return handles to stdout and stderr
    return pid;
}
