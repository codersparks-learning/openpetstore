use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::signal;

mod api;

pub async fn start_server(addr: &str) {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // Init Axum router
    let app = petstoreapi::server::new(Arc::new(api::ServerImpl {}));

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


#[tokio::main]
async fn main() {
    println!("Hello, world!");

    start_server("0.0.0.0:8080").await
}
