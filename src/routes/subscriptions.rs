use axum::extract::{Form, State};
use chrono::Utc;
use hyper::StatusCode;
use serde::Deserialize;
use sqlx::PgPool;
use tracing_futures::Instrument;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub(crate) struct SubscriptionInput {
    name: String,
    email: String,
}

pub(crate) async fn handle_subscription(
    State(db): State<PgPool>,
    Form(input): Form<SubscriptionInput>,
) -> StatusCode {
    let request_id = Uuid::new_v4();

    let request_span = tracing::info_span!(
        "Adding a new subscriber. ",
        %request_id,
        subscriber_email = %input.email,
        subscriber_name = %input.name
    );

    let _request_span_guard = request_span.enter();

    let query_span = tracing::info_span!("Saving new subscriber to the database.");

    let query_return = sqlx::query!(
        r#"
            INSERT INTO subscriptions (email, name, subscribed_at, id) VALUES ($1, $2, $3, $4)
        "#,
        input.email,
        input.name,
        Utc::now(),
        Uuid::new_v4(),
    )
    .execute(&db)
    .instrument(query_span)
    .await;

    match query_return {
        Ok(_) => {
            tracing::info!(
                "Request ID {request_id} -> User {} {} was create sucessfully",
                input.name,
                input.email
            );
            StatusCode::CREATED
        }
        Err(e) => {
            tracing::error!(
                "Request ID {request_id} -> An error {e:?} happened while registering user {} {}",
                input.name,
                input.email
            );
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}
