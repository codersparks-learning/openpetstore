use petstoreserver::server::start_server;

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    start_server("0.0.0.0:8080").await
}
