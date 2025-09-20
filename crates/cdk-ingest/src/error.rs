//! Error types for ingestion operations

use thiserror::Error;

/// Errors that can occur in ingestion operations
#[derive(Error, Debug)]
pub enum IngestError {
    #[error("Block assembly error: {0}")]
    AssemblyError(String),

    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("Mapping error: {0}")]
    MappingError(String),

    #[error("Batch processing error: {0}")]
    BatchProcessingError(String),

    #[error("Block conversion error: {0}")]
    BlockConversionError(String),

    #[error("Transaction processing error: {0}")]
    TransactionProcessingError(String),

    #[error("State root mismatch: expected {expected}, got {actual}")]
    StateRootMismatch {
        expected: alloy_primitives::FixedBytes<32>,
        actual: alloy_primitives::FixedBytes<32>,
    },

    #[error("Invalid batch data: {0}")]
    InvalidBatchData(String),

    #[error("Invalid block data: {0}")]
    InvalidBlockData(String),

    #[error("Storage error: {0}")]
    StorageError(String),

    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("Internal error: {0}")]
    InternalError(String),
}

/// Result type for ingestion operations
pub type IngestResult<T> = Result<T, IngestError>;
