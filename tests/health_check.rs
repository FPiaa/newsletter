use std::{fmt::format, net::TcpListener};

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0")
        .expect("Failed to bind a tcp listener to a random port in spawn_app");

    let port = listener.local_addr().unwrap().port();

    let server = newsletter::app_server(listener);

    let _ = tokio::spawn(server);

    format!("http://127.0.0.1:{port}")
}

#[tokio::test]
async fn health_check_test() {
    let address = spawn_app();

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
