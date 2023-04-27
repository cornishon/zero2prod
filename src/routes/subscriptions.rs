use axum::{http::StatusCode, Form};

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

pub async fn subscribe(_form: Form<FormData>) -> StatusCode {
    StatusCode::OK
}
