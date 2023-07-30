use axum::{
    routing::{get, post},
    Router,
};
use hyper::{Body, Request};
use sqlx::PgPool;
use std::net::TcpListener;
use tower_http::trace::TraceLayer;
use tower_request_id::{RequestId, RequestIdLayer};
use tracing::error_span;

use crate::routes::{health_check::health_check, subscriptions::handle_subscription};

pub async fn create_app_server(
    listener: TcpListener,
    connection: PgPool,
) -> Result<(), hyper::Error> {
    let app = Router::new()
        .route("/health_check", get(health_check))
        .route("/subscription", post(handle_subscription))
        .layer(
            TraceLayer::new_for_http().make_span_with(|request: &Request<Body>| {
                // We get the request id from the extensions
                let request_id = request
                    .extensions()
                    .get::<RequestId>()
                    .map(ToString::to_string)
                    .unwrap_or_else(|| "unknown".into());
                // And then we put it along with other information into the `request` span
                error_span!(
                    "request",
                    id = %request_id,
                    method = %request.method(),
                    uri = %request.uri(),
                )
            }),
        )
        .layer(RequestIdLayer)
        .with_state(connection);

    axum::Server::from_tcp(listener)
        .unwrap()
        .serve(app.into_make_service())
        .await
}
