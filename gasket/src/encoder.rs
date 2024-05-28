use crate::{state, utils};
use atoi::atoi;
use serde::Deserialize;
use serde_json::Value;
use std::process::Command;

#[derive(Debug, Deserialize, PartialEq)]
struct NvidiaSmiLog {
    #[serde(rename = "gpu")]
    gpu: Gpu,
}
#[derive(Debug, Deserialize, PartialEq)]
struct Gpu {
    #[serde(rename = "utilization")]
    utilization: Utilization,
    #[serde(rename = "encoder_stats")]
    encoder_stats: EncoderStats,
}

#[derive(Debug, Deserialize, PartialEq)]
struct Utilization {
    #[serde(rename = "encoder_util")]
    encoder_util: String,
}

#[derive(Debug, Deserialize, PartialEq)]
struct EncoderStats {
    #[serde(rename = "session_count")]
    session_count: String,
    #[serde(rename = "average_fps")]
    average_fps: String,
    #[serde(rename = "average_latency")]
    average_latency: String,
}

pub fn nvenc_stats() -> Result<state::EncoderStats, serde_xml_rs::Error> {
    let mut command = Command::new("nvidia-smi");
    command.arg("-q").arg("-x");

    let output = utils::run_command_capture(command);
    let log: NvidiaSmiLog = serde_xml_rs::from_str(output.as_str())?;
    let util: u32 =
        atoi(log.gpu.utilization.encoder_util.as_bytes()).expect("Failed to parse encoder_util");

    Ok(state::EncoderStats {
        utilization: util,
        devices: vec![util],
    })
}

#[derive(Debug, Deserialize, PartialEq)]
struct XrmListDevicesResponseData {
    #[serde(rename = "deviceNumber")]
    device_number: String,
}

#[derive(Debug, Deserialize, PartialEq)]
struct XrmAdmListDevicesResponse {
    #[serde(rename = "data")]
    data: XrmListDevicesResponseData,
}

#[derive(Debug, Deserialize, PartialEq)]
struct XradmListDevices {
    #[serde(rename = "response")]
    response: XrmAdmListDevicesResponse,
}

pub fn xilinx_stats() -> Result<state::EncoderStats, serde_xml_rs::Error> {
    let mut command = Command::new("xrmadm");
    command.arg("/opt/xilinx/xrm/test/list_cmd.json");

    let output = utils::run_command_capture(command);

    let list_response: XradmListDevices = serde_json::from_str(output.as_str())
        .expect("Error deserializing xrmadm list devices output");
    let num_devices = atoi::<usize>(list_response.response.data.device_number.as_bytes()).unwrap();

    let mut usage_vec = vec![0; num_devices];
    let mut total_vec = vec![0; num_devices];

    std::fs::create_dir_all("./gasket_xrmadm").expect("Failed to create directory");
    for dev_id in 0..num_devices {
        // For some reason xrmadm wants a file path and not a pipe or inline request
        let path = format!("./gasket_xrmadm/list_device{dev_id}.json");

        std::fs::write(
            &path,
            format!(
                "{{\"request\": {{\"name\": \"list\",\"requestId\": 1,\"device\": {dev_id}}}}}"
            ),
        )
        .expect("Could not write query file");

        let mut command = Command::new("xrmadm");
        command.arg(&path);
        let output = utils::run_command_capture(command);

        let parsed: Value = serde_json::from_str(output.as_str())
            .expect("Error deserializing xrmadm list device output");

        let devices = parsed["response"]["data"][format!("device_{dev_id}")]
            .as_object()
            .unwrap();

        for cu_id in 0..70 {
            if let Some(cu) = devices.get(&format!("cu_{}", cu_id)) {
                let kernel_alias = cu["kernelAlias  "].as_str().unwrap_or_default();
                match kernel_alias {
                    // "ENCODER_MPSOC" | "DECODER_MPSOC" => {
                    // Bottleneck is probably encoder not decoder
                    "ENCODER_MPSOC" => {
                        let load = cu["usedLoad     "].to_string();
                        let parts: Vec<&str> = load.split(" of ").collect();
                        usage_vec[dev_id] +=
                            atoi::<u32>(utils::to_numbers_only(parts[0].to_string()).as_bytes())
                                .expect("Failed to parse usedLoad usage");
                        total_vec[dev_id] +=
                            atoi::<u32>(utils::to_numbers_only(parts[1].to_string()).as_bytes())
                                .expect("Failed to parse usedLoad total");
                    }
                    _ => {}
                }
            }
        }
    }

    let mut avg: u32 = 0;
    // calculate usage percentage
    for i in 0..num_devices {
        usage_vec[i] = (usage_vec[i] * 100) / total_vec[i];
        avg += usage_vec[i];
    }

    Ok(state::EncoderStats {
        utilization: avg / num_devices as u32,
        devices: usage_vec,
    })
}

#[derive(Debug, Deserialize, PartialEq)]
struct NetintDevStats {
    #[serde(rename = "LOAD")]
    load: u32,
}

#[derive(Debug, Deserialize, PartialEq)]
struct NetintStats {
    #[serde(rename = "encoders")]
    encoders: Vec<NetintDevStats>,
}

pub fn netint_stats() -> Result<state::EncoderStats, serde_json::Error> {
    let mut command = Command::new("ni_rsrc_mon");
    command
        .arg("-S")
        .arg("-l")
        .arg("none")
        .arg("-o")
        .arg("json1");

    let output = utils::run_command_capture(command);

    let log: NetintStats = serde_json::from_str(output.as_str())
        .expect("Failed to parse JSON from netint ni_rsrc_mon");

    let mut util: Vec<u32> = Vec::new();
    let mut avg: u32 = 0;

    for encoder in log.encoders.iter() {
        util.push(encoder.load);
        avg += encoder.load;
    }

    Ok(state::EncoderStats {
        utilization: avg / log.encoders.len() as u32,
        devices: util,
    })
}
