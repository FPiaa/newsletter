mod common;
use common::TestApp;

#[tokio::test]
async fn health_check_test() {
    let TestApp {
        address,
        db_pool: _,
    } = common::spawn_app().await;

    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{address}/health_check"))
        .send()
        .await
        .expect("Failed to execute request");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
