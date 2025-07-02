use aws_config::SdkConfig;
use tracing::info;
use crate::config::AWS_SDK_CONFIG;

mod config;
pub mod api;

/// Loads AWS [`SdkConfig`] from environment variables. Keys it takes from are strictly defined, as
/// the documentation [<https://docs.aws.amazon.com/sdk-for-rust/latest/dg/credentials.html>] dictates.
pub async fn init(aws_config: SdkConfig) {
    info!("loading AWS config");
    AWS_SDK_CONFIG.set(aws_config).unwrap();
}
