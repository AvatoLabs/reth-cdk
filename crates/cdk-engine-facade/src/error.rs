//! Error types for engine facade

use thiserror::Error;

/// Errors that can occur in engine facade operations
#[derive(Error, Debug)]
pub enum EngineFacadeError {
    #[error("Block import failed: {0}")]
    BlockImportFailed(String),

    #[error("Finality marking failed: {0}")]
    FinalityMarkingFailed(String),

    #[error("Rollback failed: {0}")]
    RollbackFailed(String),

    #[error("Engine not initialized: {0}")]
    EngineNotInitialized(String),

    #[error("Invalid block data: {0}")]
    InvalidBlockData(String),

    #[error("Database error: {0}")]
    DatabaseError(String),

    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("Internal error: {0}")]
    InternalError(String),
}

/// Result type for engine facade operations
pub type EngineFacadeResult<T> = Result<T, EngineFacadeError>;
