use std::time::Duration;
use aws_sdk_s3::presigning::PresigningConfig;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use tracing::debug;
use crate::FileState;

/// Handles getting presigned url for the AI entity 3D model file
pub async fn get_ai_entity_3d_model(
    State(state): State<FileState>
) -> impl IntoResponse {
    debug!("getting AI entity 3D model");
    // JUST TESTING
    let presign_config = PresigningConfig::expires_in(Duration::from_secs(3600)).unwrap();
    let req = state.aws_s3_client
        .get_object()
        .bucket("elfera-assets")
        .key("3d_models/carlotta.glb")
        .presigned(presign_config)
        .await.unwrap();

    (StatusCode::OK, req.uri().to_string())
}