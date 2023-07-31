use axum::extract::{Form, State};
use chrono::Utc;
use hyper::StatusCode;
use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::{NewSubscriber, SubscriberName};

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

// TODO: make handle subscription return a Result<StatusCode, ProcessingError(?)>
pub(crate) async fn handle_subscription(
    State(db): State<PgPool>,
    Form(input): Form<SubscriptionInput>,
) -> StatusCode {
    let name = match SubscriberName::parse(input.name) {
        Ok(name) => name,
        Err(_) => return StatusCode::UNPROCESSABLE_ENTITY,
    };

    let new_subscriber = NewSubscriber {
        email: input.email,
        name,
    };

    match insert_subscriber(&new_subscriber, &db).await {
        Ok(_) => StatusCode::CREATED,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

#[tracing::instrument(skip(subscriber, pool))]
pub(crate) async fn insert_subscriber(
    subscriber: &NewSubscriber,
    pool: &PgPool,
) -> Result<(), sqlx::Error> {
    let _ = sqlx::query!(
        r#"
            INSERT INTO subscriptions (email, name, subscribed_at, id) VALUES ($1, $2, $3, $4)
        "#,
        subscriber.email,
        subscriber.name.as_ref(),
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
