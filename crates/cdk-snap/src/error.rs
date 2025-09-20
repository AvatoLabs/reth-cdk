//! Error types for CDK snapshot operations

use thiserror::Error;

/// Result type for CDK snapshot operations
pub type SnapResult<T> = Result<T, SnapError>;

/// Errors that can occur during CDK snapshot operations
#[derive(Error, Debug)]
pub enum SnapError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Invalid snapshot format: {0}")]
    InvalidFormat(String),

    #[error("Snapshot version mismatch: expected {expected}, got {actual}")]
    VersionMismatch { expected: u32, actual: u32 },

    #[error("Checksum mismatch: expected {expected}, got {actual}")]
    ChecksumMismatch { expected: String, actual: String },

    #[error("Database error: {0}")]
    Database(String),

    #[error("Conversion error: {0}")]
    Conversion(String),

    #[error("Validation error: {0}")]
    Validation(String),
}
