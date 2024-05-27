use clap::Parser;

#[derive(Parser, Clone)]
#[command(author, version, about, long_about = None)]
pub(crate) struct Args {
    ///
    ///
    /// Debug
    ///

    // List available encoders
    #[arg(short, long)]
    pub(crate) encoders: bool,

    /// Print version
    #[arg(long)]
    pub(crate) build: bool,

    ///
    ///
    /// Normal run
    ///

    // Data directory
    #[arg(short, long, env, default_value = "./test_loop.mp4")]
    pub(crate) input: String,

    // Web server address
    #[arg(long, env, default_value = "0.0.0.0:8080")]
    pub(crate) address: String,

    // Select output
    #[arg(short, long, env, default_value = "udp://0.0.0.0:30303")]
    pub(crate) output: String,

    // ffmpeg path
    #[arg(short, long, env, default_value = "ffmpeg")]
    pub(crate) ffmpeg: String,

    // Force CPU encoding
    #[arg(long, env)]
    pub(crate) cpu_only: bool,
}
