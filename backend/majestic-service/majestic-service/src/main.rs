use aws_config::BehaviorVersion;
use axum::http::Method;
use axum::http::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;
use tracing::info;
use majestic_service::{create_tracing_subscriber, run_app};
use file;
use majestic_service::config::{APPLICATION_PORT, CORS_ALLOWED_ORIGINS};

/// The entry point of the application. It handles initialization of configs, variables,
/// server properties, routing and context.
#[tokio::main]
async fn main() {
    // initializing logging
    create_tracing_subscriber();
    info!("starting up application");
    
    // loading configs
    info!("loading configs");
    // retrieving configs from env
    let aws_config = aws_config::defaults(BehaviorVersion::latest())
        .load()
        .await;
    // loading configs for file crate
    file::init(aws_config.clone()).await;

    // initializing server properties
    let listener = TcpListener::bind(format!("0.0.0.0:{}", APPLICATION_PORT.as_str())).await.unwrap();
    let cors = CorsLayer::new()
        .allow_origin(CORS_ALLOWED_ORIGINS.clone())
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_credentials(true)
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

    // initializing routing
    let router =
        file::api::create_router();

    info!("running application");
    // running application
    run_app(listener, cors, Some(router)).await.await.unwrap();
    info!("listening on port: {}", APPLICATION_PORT.as_str());
}
