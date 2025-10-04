use crate::models::HealthResponse;
use axum::{http::StatusCode, response::IntoResponse, Json};
use chrono::Utc;

/// GET /health
pub async fn health_check() -> impl IntoResponse {
    (
        StatusCode::OK,
        Json(HealthResponse {
            status: "healthy".to_string(),
            timestamp: Utc::now(),
            version: env!("CARGO_PKG_VERSION").to_string(),
        }),
    )
}
