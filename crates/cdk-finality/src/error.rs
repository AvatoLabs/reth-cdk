//! Error types for finality operations

use thiserror::Error;

/// Errors that can occur in finality operations
#[derive(Error, Debug)]
pub enum FinalityError {
    #[error("L1 RPC error: {0}")]
    L1RpcError(String),

    #[error("Contract call error: {0}")]
    ContractCallError(String),

    #[error("Network error: {0}")]
    NetworkError(String),

    #[error("Serialization error: {0}")]
    SerializationError(String),

    #[error("Oracle error: {0}")]
    OracleError(String),

    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("Timeout error: {0}")]
    TimeoutError(String),

    #[error("Invalid finality data: {0}")]
    InvalidFinalityData(String),

    #[error("Bridge contract error: {0}")]
    BridgeContractError(String),

    #[error("Rollback error: {0}")]
    RollbackError(String),

    #[error("Health check failed: {0}")]
    HealthCheckError(String),

    #[error("Internal error: {0}")]
    InternalError(String),
}

/// Result type for finality operations
pub type FinalityResult<T> = Result<T, FinalityError>;
