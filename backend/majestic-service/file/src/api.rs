use axum::Router;
use axum::routing::get;
use crate::api::commands::get_ai_entity_3d_model_handler::get_ai_entity_3d_model;
use crate::FileState;

pub mod commands;
mod dto;

pub fn create_router(file_state: FileState) -> Router {
    Router::new()
        .route("/entity", get(get_ai_entity_3d_model))
        .with_state(file_state)
}