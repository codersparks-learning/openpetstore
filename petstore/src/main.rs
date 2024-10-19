use std::env;
use tracing::{debug, event, info, Level};
use petstoreserver::server::start_server;
use tracing_subscriber;

#[tokio::main]
async fn main() {

    tracing_subscriber::fmt::init();

    let open_api_env_var = env::var("PETSTORE_OPEN_API_FILE").unwrap_or_default();
    debug!("OpenApi file path from env var: {open_api_env_var}");

    info!("Starting server on port: 8080");

    start_server("0.0.0.0:8080", open_api_env_var.as_str()).await
}
