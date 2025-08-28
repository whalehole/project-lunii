use axum::Router;
use axum::routing::get;
use crate::file::handlers::get_ai_entity_3d_model::get_ai_entity_3d_model;

mod handlers;

fn api_routes() -> Router<> {
    Router::new()
        .route("/entity", get(get_ai_entity_3d_model))
        .with_state()
}