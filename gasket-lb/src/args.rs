use clap::Parser;

#[derive(Parser, Clone)]
#[command(author, version, about, long_about = None)]
pub(crate) struct Args {
    // Web server address
    #[arg(long, env, default_value = "0.0.0.0:8888")]
    pub(crate) host: String,

    // State file
    #[arg(long, env, default_value = "state.json")]
    pub(crate) state_file: String,

    // Worker discovery (Formatted as "hostname0@publicIp0;hostname1@publicIp1")
    #[arg(long, env)]
    pub(crate) worker_discovery: Option<String>,
}
