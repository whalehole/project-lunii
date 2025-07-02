use std::sync::Arc;
use aws_config::SdkConfig;
use aws_sdk_s3::Client;
use tracing::info;
use crate::application::file_service::FileService;
use crate::config::AWS_SDK_CONFIG;

mod config;
pub mod api;
pub mod application;
mod domain;

/// Loads AWS [`SdkConfig`] from environment variables. Keys it takes from are strictly defined, as
/// the documentation [<https://docs.aws.amazon.com/sdk-for-rust/latest/dg/credentials.html>] dictates.
pub async fn init(aws_config: SdkConfig) {
    info!("loading AWS config");
    AWS_SDK_CONFIG.set(aws_config).unwrap();
}

/// State struct to be used by handlers and services
#[derive(Clone)]
pub struct FileState {
    pub file_service: Arc<FileService>,
    pub aws_s3_client: Arc<Client>
}
