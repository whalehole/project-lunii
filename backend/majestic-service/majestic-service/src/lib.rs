use axum::http::StatusCode;
use axum::response::IntoResponse;
use tracing::info;
use tracing_subscriber::EnvFilter;

pub mod config;

#[derive(Clone)]
pub struct AppState {}

/// Handles a health check command.
pub async fn health_check_handler() -> (StatusCode, impl IntoResponse) {
    info!("health check");
    (StatusCode::OK, "")
}

/// Creates a logging provider by using Tracing [<https://tokio.rs/tokio/topics/tracing>].
pub fn create_tracing_subscriber() {
    info!("creating tracing subscriber");
    let subscriber = tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .compact()
        .with_file(true)
        .with_line_number(true)
        .with_thread_ids(true)
        .with_target(false)
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .expect("setting default subscriber failed");
}