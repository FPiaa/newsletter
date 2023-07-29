use std::net::TcpListener;

use newsletter::configuration;
use newsletter::startup::create_app_server;
use sqlx::PgPool;

#[tokio::main]
async fn main() -> hyper::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let configuration =
        configuration::get_configuration().expect("Unable to read configuration file");

    let address = format!("127.0.0.1:{}", configuration.database.port);

    let listener = TcpListener::bind(address).unwrap();
    let db_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Unable to connect with database");
    create_app_server(listener, db_pool.clone()).await
}
