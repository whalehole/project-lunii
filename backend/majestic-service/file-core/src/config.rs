use aws_config::SdkConfig;
use once_cell::sync::OnceCell;



/// Constant containing config for AWS SDKs.
pub const AWS_SDK_CONFIG: OnceCell<SdkConfig> = OnceCell::new();