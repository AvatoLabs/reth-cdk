//! Error types for CDK observability

use thiserror::Error;

/// Errors that can occur in observability operations
#[derive(Error, Debug)]
pub enum ObservabilityError {
    #[error("Metrics error: {0}")]
    MetricsError(String),

    #[error("Tracing error: {0}")]
    TracingError(String),

    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("Performance monitoring error: {0}")]
    PerformanceError(String),

    #[error("Cache error: {0}")]
    CacheError(String),

    #[error("Internal error: {0}")]
    InternalError(String),
}

/// Result type for observability operations
pub type ObservabilityResult<T> = Result<T, ObservabilityError>;

impl From<prometheus::Error> for ObservabilityError {
    fn from(err: prometheus::Error) -> Self {
        ObservabilityError::MetricsError(err.to_string())
    }
}

impl From<std::io::Error> for ObservabilityError {
    fn from(err: std::io::Error) -> Self {
        ObservabilityError::InternalError(err.to_string())
    }
}
