use std::sync::Arc;

use crate::server::AppState;
use axum::response::IntoResponse;

pub(crate) async fn get_status(_state: Arc<AppState>) -> impl IntoResponse {
    String::from("Hello World!")
}

pub(crate) async fn get_status_by_id(_state: Arc<AppState>) -> impl IntoResponse {
    String::from("Hello World!")
}
