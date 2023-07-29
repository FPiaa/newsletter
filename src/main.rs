use std::net::TcpListener;

use newsletter::app_server;

#[tokio::main]
async fn main() -> hyper::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let listener = TcpListener::bind("127.0.0.1:8000").unwrap();
    app_server(listener).await
}
