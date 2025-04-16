use std::sync::Arc;

use axum::{extract::State, Json};
use reqwest::StatusCode;
use serde_json::{json, Value};

use crate::{cqrs::{CommandHandler, CreateCheckoutSessionCommand}, dtos::ApiError, state::AppState};

pub async fn index() -> &'static str {
    "Hello, World!"
}

pub async fn create_checkout_session(State(state): State<Arc<AppState>>, Json(create_checkout_session_command): Json<CreateCheckoutSessionCommand>) -> (StatusCode, Json<Value>) {
    match state.create_checkout_session_command_handler.handle(&create_checkout_session_command).await {
        Ok(response) => (StatusCode::CREATED, Json(json!(response))),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!(ApiError{error: e})))
    }
}