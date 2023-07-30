use std::net::TcpListener;

use newsletter::configuration;
use newsletter::startup::create_app_server;
use newsletter::telemetry::{get_subscriber, init_subscriber};
use sqlx::PgPool;

#[tokio::main]
async fn main() -> hyper::Result<()> {
    let subscriber = get_subscriber("newsletter".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let configuration =
        configuration::get_configuration().expect("Unable to read configuration file");

    let address = format!("127.0.0.1:{}", configuration.application_port);

    let listener = TcpListener::bind(address).unwrap();
    let db_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Unable to connect with database");

    tracing::info!(
        "Listening server on port {}",
        listener.local_addr().unwrap()
    );
    create_app_server(listener, db_pool.clone()).await
}
