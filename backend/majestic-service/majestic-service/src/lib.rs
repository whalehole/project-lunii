use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Router;
use axum::routing::get;
use axum::serve::Serve;
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;
use tracing::info;
use tracing_subscriber::EnvFilter;

pub mod config;

/// Handles a health check command.
async fn health_check_handler() -> (StatusCode, impl IntoResponse) {
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

/// Creates and returns an Axum application server.
pub async fn run_app(listener: TcpListener, cors_layer: CorsLayer, router: Option<Router>) -> Serve<TcpListener, Router, Router> {
    // if there is router, merge with existing
    if let Some(router) = router {
        let app = Router::new()
            .route("/api/healthcheck", get(health_check_handler))
            .merge(router)
            .layer(cors_layer);

        axum::serve(listener, app)
    }
    else { // else create just existing
        let app = Router::new()
            .route("/api/healthcheck", get(health_check_handler))
            .layer(cors_layer);

        axum::serve(listener, app)
    }
}