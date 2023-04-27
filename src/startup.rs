use std::net::TcpListener;

use crate::routes;

use axum::{
    routing::{get, post},
    Router,
};

pub async fn run(listener: TcpListener) {
    // build our application
    let app = Router::new()
        .route("/health_check", get(routes::health_check))
        .route("/subscriptions", post(routes::subscribe));

    // run it with hyper on localhost:3000
    axum::Server::from_tcp(listener)
        .unwrap()
        .serve(app.into_make_service())
        .await
        .unwrap()
}
