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

/// Constant containing CORS allowed origins.
pub const CORS_ALLOWED_ORIGINS: Lazy<Vec<HeaderValue>> = Lazy::new(|| {
    env::var("CORS_ALLOWED_ORIGINS")
        .expect(generate_error_message("CORS_ALLOWED_ORIGINS").as_str())
        .split(',')
        .map(|origin| origin.parse::<HeaderValue>().unwrap())
        .collect()
});