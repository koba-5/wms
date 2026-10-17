use crate::service;
use crate::util::error::Error;
use axum::Json;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{MethodRouter, get};

pub fn todo_controller() -> MethodRouter {
    get(get_handler)
}

async fn get_handler() -> Result<impl IntoResponse, Error> {
    let response = service::todo::get(1)?;
    Ok((StatusCode::OK, Json(response)))
}
