use axum::Router;
use axum::routing::get;
use crate::api::commands::get_ai_entity_3d_model_handler::get_ai_entity_3d_model;

mod commands;
mod dto;

pub fn create_router() -> Router {
    Router::new()
        .route("/file/entity", get(get_ai_entity_3d_model))
}