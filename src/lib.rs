use axum::{http::StatusCode, routing::get, Router};
use std::net::TcpListener;

pub async fn run(listener: TcpListener) {
    // build our application
    let app = Router::new().route("/health_check", get(health_check));

    // run it with hyper on localhost:3000
    axum::Server::from_tcp(listener)
        .unwrap()
        .serve(app.into_make_service())
        .await
        .unwrap()
}

async fn health_check() -> StatusCode {
    StatusCode::OK
}
