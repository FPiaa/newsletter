use std::net::TcpListener;

use newsletter::app_server;
use newsletter::configuration;

#[tokio::main]
async fn main() -> hyper::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let configuration =
        configuration::get_configuration().expect("Unable to read configuration file");

    let address = format!("127.0.0.1:{}", configuration.database.port);

    let listener = TcpListener::bind(address).unwrap();
    app_server(listener).await
}
