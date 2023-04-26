use std::net::TcpListener;

#[tokio::test]
async fn health_check_works() {
    let address = spawn_app().await;
    let endpoint = format!("http://{address}/health_check");
    
    let response = reqwest::Client::new()
        .get(endpoint)
        .send()
        .await
        .expect("request to {endpoint}");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

async fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("to bind a random port");
    let port = listener.local_addr().unwrap().port();
    tokio::spawn(zero2prod::run(listener));
    format!("127.0.0.1:{port}")
}
