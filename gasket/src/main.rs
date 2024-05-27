use axum::{
    routing::{delete, get, post},
    Router,
};
use clap::Parser;
use dotenv::dotenv;
use std::sync::Arc;
use tokio;
use tower_http::cors::CorsLayer;

mod args;
mod encoder;
mod monitor;
mod router;
mod state;
mod transcode;
mod utils;

fn setup() {
    // Apply dotenv
    dotenv().ok();

    // Default log info
    env_logger::init();

    // Check debug args
    let args = args::Args::parse();
    if args.encoders {
        transcode::list_hardware_encoders();
        std::process::exit(0);
    }

    let build_version = utils::get_build_info();

    if args.build {
        println!("{build_version}");
        std::process::exit(0);
    }
    log::info!("Using gasket build: {build_version}");

    let ffmpeg_path = transcode::check_ffmpeg_installed();
    match ffmpeg_path.trim() {
        "" => {
            log::error!("Cannot find ffmpeg, is it in the path?");
            std::process::exit(1);
        }
        _ => log::info!("Using ffmpeg at {}", ffmpeg_path.trim()),
    }
}

#[tokio::main]
async fn main() {
    // Setup
    let args = args::Args::parse();
    setup();

    // Shared app state
    let shared_state = Arc::new(state::new_app());

    // Stream monitor thread
    tokio::spawn(monitor::start(shared_state.clone()));

    // Web server
    let app = Router::new()
        .route("/", get(router::index))
        .route("/stream", get(router::get_streams))
        .route("/stream", post(router::create_stream))
        .route("/stream/:uuid", delete(router::delete_stream))
        .route("/encoder", get(router::get_encoder_status))
        .route("/capabilities", get(router::get_capabilities))
        .route("/livez", get(router::livez))
        .route("/readyz", get(router::readyz))
        .layer(CorsLayer::permissive())
        .with_state(shared_state);

    log::info!("Listening on: {}", args.address);
    let listener = tokio::net::TcpListener::bind(&args.address).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
