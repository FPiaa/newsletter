use newsletter::{
    configuration::{get_configuration, DatabaseSettings},
    startup,
    telemetry::{get_subscriber, init_subscriber},
};
use sqlx::{Executor, PgPool};

use std::net::TcpListener;
use std::sync::OnceLock;

static TRACING: OnceLock<()> = OnceLock::new();

pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
}

pub(crate) async fn spawn_app() -> TestApp {
    let _ = TRACING.get_or_init(|| {
        let subscriber = get_subscriber("test".into(), "debug".into());
        init_subscriber(subscriber);
    });

    let listener = TcpListener::bind("127.0.0.1:0")
        .expect("Failed to bind a tcp listener to a random port in spawn_app");

    let port = listener.local_addr().unwrap().port();
    let mut configuration = get_configuration().expect("Unable to read configuration file");

    configuration.database.database_name = uuid::Uuid::new_v4().to_string();
    let pool = configure_database(&configuration.database).await;
    let server = startup::create_app_server(listener, pool.clone());

    let _ = tokio::spawn(server);

    TestApp {
        address: format!("http://127.0.0.1:{port}"),
        db_pool: pool,
    }
}

pub async fn configure_database(config: &DatabaseSettings) -> PgPool {
    let connection = PgPool::connect(&config.connection_string_without_db())
        .await
        .expect("Failed to connect with DB in configure_database");

    connection
        .execute(format!(r#"CREATE DATABASE "{}";"#, config.database_name).as_str())
        .await
        .expect("Failed to create database");

    let connection_pool = PgPool::connect(&config.connection_string())
        .await
        .expect("Failed to conned to the newly created database");

    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to run migrations on the newly created database");

    connection_pool
}
