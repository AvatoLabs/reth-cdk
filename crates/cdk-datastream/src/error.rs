//! Error types for datastream operations

use thiserror::Error;

/// Errors that can occur in datastream operations
#[derive(Error, Debug)]
pub enum DatastreamError {
    #[error("Network error: {0}")]
    NetworkError(String),

    #[error("HTTP error: {status} - {message}")]
    HttpError { status: u16, message: String },

    #[error("Serialization error: {0}")]
    SerializationError(String),

    #[error("Checkpoint error: {0}")]
    CheckpointError(String),

    #[error("Source unavailable: {0}")]
    SourceUnavailable(String),

    #[error("Invalid batch data: {0}")]
    InvalidBatchData(String),

    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("Timeout error: {0}")]
    TimeoutError(String),

    #[error("Internal error: {0}")]
    InternalError(String),

    #[error("Connection error: {0}")]
    ConnectionError(String),

    #[error("Communication error: {0}")]
    CommunicationError(String),

    #[error("Deserialization error: {0}")]
    DeserializationError(String),

    #[error("IO error: {0}")]
    IoError(String),
}

/// Result type for datastream operations
pub type DatastreamResult<T> = Result<T, DatastreamError>;

/// Alias for backward compatibility
pub type DataStreamError = DatastreamError;
pub type DataStreamResult<T> = DatastreamResult<T>;
