use std::net::TcpListener;

#[tokio::test]
async fn health_check_works() {
    let address = spawn_app().await;

    let response = reqwest::Client::new()
        .get(format!("{address}/health_check"))
        .send()
        .await
        .expect("request to health_check");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

/// Spin up an instance of our application
/// and returns its address (i.e. http://localhost:XXXX)
async fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("to bind a random port");
    let port = listener.local_addr().unwrap().port();
    tokio::spawn(zero2prod::run(listener));
    format!("http://127.0.0.1:{port}")
}
