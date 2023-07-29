use axum::http::StatusCode;
mod common;

#[tokio::test]
async fn subscribe_returns_200_when_valid_form_data() {
    let app_address = common::spawn_app();
    let client = reqwest::Client::new();

    let body = "name=1e%20guin&email=ursula_le_gin%40gmail.com";

    let response = client
        .post(&format!("{app_address}/subscription"))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request in subscribe returns 200");

    assert_eq!(StatusCode::OK, response.status().as_u16());
}

#[tokio::test]
async fn subscribe_returns_400_when_data_is_missing() {
    let app_address = common::spawn_app();
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email"),
    ];

    for (invalid_body, error_message) in test_cases {
        let response = client
            .post(&format!("{app_address}/subscription"))
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
