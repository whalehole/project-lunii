use once_cell::sync::Lazy;
use std::env;
use axum::http::HeaderValue;

/// Generates messages for errors for when an attempt to load an environment variable fails.
fn generate_error_message(name: &str) -> String {
    format!("Encountered an error with environment variable [{}]", name)
}

/// Constant containing application port.
pub const APPLICATION_PORT: Lazy<String> = Lazy::new(|| {
    format!("{}",
            env::var("APPLICATION_PORT")
                .expect(generate_error_message("APPLICATION_PORT").as_str())
    )
});

/// Constant containing AWS region
pub const AWS_REGION: Lazy<String> = Lazy::new(|| {
    format!("{}",
            env::var("AWS_REGION")
                .expect(generate_error_message("AWS_REGION").as_str())
    )
});

/// Constant containing AWS endpoint URL
pub const AWS_ENDPOINT_URL: Lazy<String> = Lazy::new(|| {
    format!("{}",
            env::var("AWS_ENDPOINT_URL")
                .unwrap_or_else(|_| String::new())
    )
});

/// Constant containing AWS S3 force-path-style choice
pub const AWS_S3_FORCE_PATH_STYLE: Lazy<String> = Lazy::new(|| {
    format!("{}",
            env::var("AWS_S3_FORCE_PATH_STYLE")
                .unwrap_or_else(|_| String::new())
    )
});

/// Constant containing CORS allowed origins.
pub const CORS_ALLOWED_ORIGINS: Lazy<Vec<HeaderValue>> = Lazy::new(|| {
    env::var("CORS_ALLOWED_ORIGINS")
        .expect(generate_error_message("CORS_ALLOWED_ORIGINS").as_str())
        .split(',')
        .map(|origin| origin.parse::<HeaderValue>().unwrap())
        .collect()
});