use axum::{
    routing::{delete, get, patch, post},
    Router,
};
use clap::Parser;
use dotenv::dotenv;
use std::sync::Arc;
use tower_http::cors::CorsLayer;

mod args;
mod monitor;
mod router;
mod state;
mod stream;
mod utils;
mod worker;

fn setup() {
    // Apply dotenv
    dotenv().ok();

    // Default log info
    env_logger::init();
}

#[tokio::main]
async fn main() {
    // Setup
    let args = args::Args::parse();
    setup();

    log::info!("Starting gasket build: {}", utils::get_build_info());

    // Shared app state
    let shared_state = Arc::new(state::App::new());

    // Stream monitor thread
    tokio::spawn(monitor::start(shared_state.clone()));

    // Web server
    let app = Router::new()
        // Index
        .route("/", get(router::index))
        .route("/", delete(router::reset_state))
        // Stream
        .route("/stream", get(router::get_all_streams))
        .route("/stream", post(router::create_stream))
        .route("/stream/:uuid", get(router::get_stream))
        .route("/stream/:uuid", patch(router::patch_stream))
        .route("/stream/:uuid", delete(router::delete_stream))
        // Worker
        .route("/worker", get(router::get_all_workers))
        .route("/worker", post(router::create_worker))
        .route("/worker/:uuid", get(router::get_worker))
        .route("/worker/:uuid", patch(router::patch_worker))
        .route("/worker/:uuid", delete(router::delete_worker))
        // Health
        .route("/livez", get(router::livez))
        .route("/readyz", get(router::readyz))
        .layer(CorsLayer::permissive())
        .with_state(shared_state);

    log::info!("Listening on: {}", args.host);
    let listener = tokio::net::TcpListener::bind(&args.host).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
