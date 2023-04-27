use axum::{
    http::StatusCode,
    routing::{get, post},
    Form, Router,
};
use std::net::TcpListener;

pub async fn run(listener: TcpListener) {
    // build our application
    let app = Router::new()
        .route("/health_check", get(health_check))
        .route("/subscriptions", post(subscribe));

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

#[derive(serde::Deserialize)]
struct FormData {
    email: String,
    name: String,
}

async fn subscribe(_form: Form<FormData>) -> StatusCode {
    StatusCode::OK
}
