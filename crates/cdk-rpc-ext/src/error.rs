//! Error types for CDK RPC operations

use thiserror::Error;

/// Result type for CDK RPC operations
pub type CdkRpcResult<T> = Result<T, CdkRpcError>;

/// Errors that can occur during CDK RPC operations
#[derive(Error, Debug)]
pub enum CdkRpcError {
    /// Batch not found
    #[error("Batch {0} not found")]
    BatchNotFound(String),

    /// Epoch not found for the given block number
    #[error("Epoch for block {0} not found")]
    EpochNotFound(String),

    /// Finality oracle error
    #[error("Finality oracle error: {0}")]
    FinalityOracleError(String),

    /// Data source error
    #[error("Data source error: {0}")]
    DataSourceError(String),

    /// Invalid parameter
    #[error("Invalid parameter: {0}")]
    InvalidParameter(String),

    /// Internal server error
    #[error("Internal server error: {0}")]
    InternalError(String),

    /// Service unavailable
    #[error("Service unavailable: {0}")]
    ServiceUnavailable(String),
}

impl From<cdk_datastream::DatastreamError> for CdkRpcError {
    fn from(err: cdk_datastream::DatastreamError) -> Self {
        CdkRpcError::DataSourceError(err.to_string())
    }
}

impl From<cdk_ingest::IngestError> for CdkRpcError {
    fn from(err: cdk_ingest::IngestError) -> Self {
        CdkRpcError::InternalError(err.to_string())
    }
}

impl From<cdk_finality::FinalityError> for CdkRpcError {
    fn from(err: cdk_finality::FinalityError) -> Self {
        CdkRpcError::FinalityOracleError(err.to_string())
    }
}
