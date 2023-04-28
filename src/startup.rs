use std::net::TcpListener;

use crate::routes;

use axum::{
    routing::{get, post},
    Router,
};
use sqlx::PgPool;
use tower_http::trace::TraceLayer;

pub async fn run(listener: TcpListener, db_pool: PgPool) {
    // build our application
    let app = Router::new()
        .route("/health_check", get(routes::health_check))
        .route("/subscriptions", post(routes::subscribe))
        .with_state(db_pool)
        .layer(TraceLayer::new_for_http());

    // run it with hyper on localhost:8000
    axum::Server::from_tcp(listener)
        .unwrap()
        .serve(app.into_make_service())
        .await
        .unwrap()
}
