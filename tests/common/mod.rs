use std::net::TcpListener;

pub(crate) fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0")
        .expect("Failed to bind a tcp listener to a random port in spawn_app");

    let port = listener.local_addr().unwrap().port();

    let server = newsletter::app_server(listener);

    let _ = tokio::spawn(server);

    format!("http://127.0.0.1:{port}")
}
