use std::fs;
use std::sync::Arc;
use axum::routing::get;
use axum_swagger_ui::swagger_ui;
use tokio::net::TcpListener;
use tokio::signal;
use tracing::{info, trace, warn};
use crate::api;

pub async fn start_server(addr: &str, openapi_path: &str) {

    // Init Axum router
    let mut app = petstoreapi::server::new(Arc::new(api::ServerImpl {}));

    let metadata = fs::metadata(openapi_path);

    match metadata {
        Err(_) => warn!("cannot find OpenAPI file (or not specified), therefore not adding swagger-ui route"),
        Ok(metadata) => {
            info!( "OpenAPI path is file: {}", metadata.is_file());
            let doc_url = "swagger/openapi.yaml";

            if let Ok(openapi_file) = fs::read_to_string(openapi_path) {
                trace!("file contents:");
                trace!("{}", openapi_file);
                app = app.route("/swagger-ui", get(|| async { swagger_ui(doc_url) }))
                    .route(format!("/{doc_url}").as_str(), get(|| async { openapi_file }));
            } else {
                warn!("cannot read OpenApi file");
            }
        }
    }

    // Add layers to the router
    //let app = app.layer(...);

    // Run the server with graceful shutdown
    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => { println!("Ctrl+c pressed");},
        _ = terminate => {},
    }
}