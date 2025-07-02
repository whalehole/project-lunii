use std::sync::Arc;
use aws_config::{BehaviorVersion, Region};
use aws_config::meta::region::RegionProviderChain;
use axum::http::Method;
use axum::http::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use axum::Router;
use axum::routing::get;
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;
use tracing::info;
use majestic_service::{create_tracing_subscriber, health_check_handler, AppState};
use file::application::file_service::FileService;
use file::FileState;
use file::api::commands::get_ai_entity_3d_model_handler::get_ai_entity_3d_model;
use majestic_service::config::{APPLICATION_PORT, AWS_ENDPOINT_URL, AWS_REGION, CORS_ALLOWED_ORIGINS};

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
    let region_provider = RegionProviderChain::first_try(Region::new(AWS_REGION.clone()));
    let aws_config = aws_config::defaults(BehaviorVersion::latest())
        .region(region_provider)
        .endpoint_url(AWS_ENDPOINT_URL.as_str())
        .load()
        .await;
    // loading configs for file crate
    file::init(aws_config.clone()).await;

    // initializing services and shared dependencies
    info!("initializing services, shared dependencies, and states");
    let app_state = AppState {};
    let aws_s3_config = aws_sdk_s3::config::Builder::from(&aws_config).force_path_style(true).build();
    let aws_s3_client = Arc::new(aws_sdk_s3::Client::from_conf(aws_s3_config));

    info!("initializing file");
    let file_service = Arc::new(FileService {});
    let file_state = FileState {
        file_service: file_service.clone(),
        aws_s3_client: aws_s3_client.clone()
    };

    // initializing server properties
    let listener = TcpListener::bind(format!("0.0.0.0:{}", APPLICATION_PORT.as_str())).await.unwrap();
    let cors = CorsLayer::new()
        .allow_origin(CORS_ALLOWED_ORIGINS.clone())
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_credentials(true)
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

    // initializing routing
    let file_router = Router::new()
        .route("/entity", get(get_ai_entity_3d_model))
        .with_state(file_state);

    let router = Router::new()
        .route("/api/healthcheck", get(health_check_handler))
        .nest("/file", file_router)
        .layer(cors)
        .with_state(app_state);

    info!("running application");
    info!("listening on port: {}", APPLICATION_PORT.as_str());
    axum::serve(listener, router).await.unwrap();
}
