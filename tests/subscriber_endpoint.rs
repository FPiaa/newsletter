use axum::http::StatusCode;

mod common;
use common::TestApp;

#[tokio::test]
async fn subscribe_returns_201_when_valid_form_data() {
    let TestApp { address, db_pool } = common::spawn_app().await;

    let client = reqwest::Client::new();

    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";

    let response = client
        .post(&format!("{address}/subscription"))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request in subscribe returns 200");

    let saved_user = sqlx::query!("SELECT email, name FROM subscriptions",)
        .fetch_one(&db_pool)
        .await
        .expect("Failed to fetch saved subscription");

    assert_eq!(StatusCode::CREATED, response.status().as_u16());
    assert_eq!(saved_user.email, "ursula_le_guin@gmail.com");
    assert_eq!(saved_user.name, "le guin");
}

#[tokio::test]
async fn subscribe_returns_422_when_data_is_missing() {
    let TestApp {
        address,
        db_pool: _,
    } = common::spawn_app().await;

    let client = reqwest::Client::new();

    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email"),
    ];

    for (invalid_body, error_message) in test_cases {
        let response = client
            .post(&format!("{address}/subscription"))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request in subscribe returns 200");

        assert_eq!(
            StatusCode::UNPROCESSABLE_ENTITY,
            response.status().as_u16(),
            "The API didn't return 422 Unprocessable Entity when the payload was {error_message}"
        );
    }
}
