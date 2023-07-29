use axum::extract::{Form, State};
use chrono::Utc;
use hyper::StatusCode;
use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub(crate) struct SubscriptionInput {
    name: String,
    email: String,
}

pub(crate) async fn handle_subscription(
    State(db): State<PgPool>,
    Form(input): Form<SubscriptionInput>,
) -> StatusCode {
    let query_return = sqlx::query!(
        r#"
            INSERT INTO subscriptions (id, email, name, subscribed_at) VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        input.email,
        input.name,
        Utc::now()
    )
    .execute(&db)
    .await;

    match query_return {
        Ok(_) => StatusCode::CREATED,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}
