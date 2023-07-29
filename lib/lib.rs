use std::net::TcpListener;

use axum::{
    routing::{get, post},
    Form, Router,
};
use serde::Deserialize;

async fn health_check() {}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct SubscriptionInput {
    name: String,
    email: String,
}

async fn handle_subscription(Form(input): Form<SubscriptionInput>) {
    dbg!(&input);
}

pub async fn app_server(listener: TcpListener) -> Result<(), hyper::Error> {
    let app = Router::new()
        .route("/health_check", get(health_check))
        .route("/subscription", post(handle_subscription));

    axum::Server::from_tcp(listener)
        .unwrap()
        .serve(app.into_make_service())
        .await
}
