use std::net::TcpListener;

use tracing::subscriber::set_global_default;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};

use newsletter::configuration;
use newsletter::startup::create_app_server;
use sqlx::PgPool;
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};

use tracing_log::LogTracer;
#[tokio::main]
async fn main() -> hyper::Result<()> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "newsletter=debug,tower_http=debug")
    }

    LogTracer::init().expect("Failed to start log tracer");

    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    let formatting_layer = BunyanFormattingLayer::new("newsletter".into(), std::io::stdout);

    let subscriber = Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer);

    set_global_default(subscriber).expect("Failed to get a susbcriber");

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
