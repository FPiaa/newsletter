use axum::extract::{Form, State};
use chrono::Utc;
use hyper::StatusCode;
use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub(crate) struct SubscriptionInput {
    name: String,
    email: String,
}

#[tracing::instrument(
    name = "Adding new user",
    skip(db, input),
    fields(
        subscriber_email = %input.email,
        subscriber_name = %input.name
    )
)]
pub(crate) async fn handle_subscription(
    State(db): State<PgPool>,
    Form(input): Form<SubscriptionInput>,
) -> StatusCode {
    match insert_subscriber(&input, &db).await {
        Ok(_) => StatusCode::CREATED,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

#[tracing::instrument(skip(input, pool))]
pub(crate) async fn insert_subscriber(
    input: &SubscriptionInput,
    pool: &PgPool,
) -> Result<(), sqlx::Error> {
    let _ = sqlx::query!(
        r#"
            INSERT INTO subscriptions (email, name, subscribed_at, id) VALUES ($1, $2, $3, $4)
        "#,
        input.email,
        input.name,
        Utc::now(),
        Uuid::new_v4(),
    )
    .execute(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query {e:?}");
        e
    })?;

    Ok(())
}
