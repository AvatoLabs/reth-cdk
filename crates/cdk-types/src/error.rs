//! Error types for CDK integration

use thiserror::Error;

/// Errors that can occur in CDK operations
#[derive(Error, Debug)]
pub enum CdkError {
    #[error("Invalid batch: {0}")]
    InvalidBatch(String),

    #[error("Invalid epoch: {0}")]
    InvalidEpoch(String),

    #[error("Invalid finality status: {0}")]
    InvalidFinality(String),

    #[error("Data availability verification failed: {0}")]
    DataAvailabilityFailed(String),

    #[error("L1 contract interaction failed: {0}")]
    L1ContractError(String),

    #[error("Serialization error: {0}")]
    SerializationError(String),

    #[error("Network error: {0}")]
    NetworkError(String),

    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("Internal error: {0}")]
    InternalError(String),
}

/// Result type for CDK operations
pub type CdkResult<T> = Result<T, CdkError>;
