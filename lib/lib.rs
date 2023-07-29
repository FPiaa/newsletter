use std::net::TcpListener;

use axum::{routing::get, Router};

async fn health_check() {}

pub async fn app_server(listener: TcpListener) -> Result<(), hyper::Error> {
    let app = Router::new().route("/health_check", get(health_check));

    axum::Server::from_tcp(listener)
        .unwrap()
        .serve(app.into_make_service())
        .await
}
