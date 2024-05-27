#![allow(dead_code)]

use clap::Parser;
use std::process::Command;

use crate::args::Args;

pub(crate) fn get_build_info() -> String {
    let raw_ver = option_env!("BUILD_VERSION");
    if raw_ver.is_none() {
        return "test".to_string();
    }
    let raw_ver = raw_ver.unwrap();

    let long_sha = raw_ver.split("-").last().unwrap();
    let short_sha = long_sha.chars().take(7).collect::<String>();

    raw_ver.replace(long_sha, &short_sha)
}

pub(crate) fn run_command(mut command: Command) -> bool {
    let result = command.status();

    return match result {
        Ok(status) => status.success(),
        Err(e) => {
            log::error!(
                "Failed to execute command '{}': {}",
                format!("{:?}", command),
                e
            );
            false
        }
    };
}

pub(crate) fn run_command_silent(mut command: Command) -> bool {
    let result = command.output();

    return match result {
        Ok(_r) => true,
        Err(e) => {
            log::error!(
                "Failed to execute command '{}': {}",
                format!("{:?}", command),
                e
            );
            false
        }
    };
}

pub(crate) fn run_command_capture(mut command: Command) -> String {
    let result = command.output();

    return match result {
        Ok(output) => String::from_utf8_lossy(&output.stdout).to_string(),
        Err(e) => {
            log::error!(
                "Failed to execute command '{}': {}",
                format!("{:?}", command),
                e
            );
            String::from("Error")
        }
    };
}

pub(crate) fn to_numbers_only(input: String) -> String {
    return input.chars().filter(|c| c.is_numeric()).collect();
}

pub(crate) fn get_ffmpeg_path() -> String {
    let args = Args::parse();
    let ffmpeg_path = args.ffmpeg;
    return ffmpeg_path;
}
