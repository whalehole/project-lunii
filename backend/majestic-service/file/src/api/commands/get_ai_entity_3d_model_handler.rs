use axum::http::StatusCode;
use axum::response::IntoResponse;
use tracing::debug;

/// Handles getting presigned url for the AI entity 3D model file
pub async fn get_ai_entity_3d_model() -> impl IntoResponse {
    debug!("getting AI entity 3D model");

    (StatusCode::OK, ())
}