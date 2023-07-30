use axum::{
    routing::{get, post},
    Router,
};
use sqlx::PgPool;
use std::net::TcpListener;

use crate::routes::{health_check::health_check, subscriptions::handle_subscription};

pub async fn create_app_server(
    listener: TcpListener,
    connection: PgPool,
) -> Result<(), hyper::Error> {
    let app = Router::new()
        .route("/health_check", get(health_check))
        .route("/subscription", post(handle_subscription))
        .with_state(connection);

    axum::Server::from_tcp(listener)
        .unwrap()
        .serve(app.into_make_service())
        .await
}
