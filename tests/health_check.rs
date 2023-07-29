mod common;

#[tokio::test]
async fn health_check_test() {
    let address = common::spawn_app();

    for _ in 0..100 {
        let client = reqwest::Client::new();

        let response = client
            .get(&format!("{address}/health_check"))
            .send()
            .await
            .expect("Failed to execute request");

        assert!(response.status().is_success());
        assert_eq!(Some(0), response.content_length());
    }
}
