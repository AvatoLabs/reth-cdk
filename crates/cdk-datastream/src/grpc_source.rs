//! gRPC data stream source for CDK batch ingestion

use crate::{
    error::{DataStreamError, DataStreamResult},
    source::{BatchSource, BatchStream},
};
use async_trait::async_trait;
use cdk_types::Batch;
use tonic::transport::Channel;
use tracing::{info};

/// Configuration for the gRPC batch source
#[derive(Debug, Clone)]
pub struct GrpcSourceConfig {
    /// The URL of the gRPC endpoint
    pub url: String,
}

/// gRPC implementation of `BatchSource`
#[derive(Debug)]
pub struct GrpcSource {
    config: GrpcSourceConfig,
}

impl GrpcSource {
    /// Create a new GrpcSource
    pub async fn new(config: GrpcSourceConfig) -> DataStreamResult<Self> {
        info!(target: "cdk::datastream::grpc", url = %config.url, "Connecting to gRPC source");
        let _channel = Channel::from_shared(config.url.clone())
            .map_err(|e| DataStreamError::ConnectionError(format!("Invalid gRPC URL: {}", e)))?
            .connect()
            .await
            .map_err(|e| DataStreamError::ConnectionError(format!("Failed to connect to gRPC: {}", e)))?;
        info!(target: "cdk::datastream::grpc", url = %config.url, "gRPC connection established");
        Ok(Self { config })
    }
}

#[async_trait]
impl BatchSource for GrpcSource {
    async fn fetch_batch_stream(&self, _start_batch_number: Option<u64>) -> DataStreamResult<BatchStream> {
        info!(target: "cdk::datastream::grpc", start_batch_number = ?_start_batch_number, "Subscribing to gRPC batch stream");
        
        // For now, return an empty stream since we don't have the actual gRPC proto definitions
        let batch_stream = async_stream::stream! {
            // Empty stream for now - yield nothing
            if false {
                yield Ok(Batch::new(
                    cdk_types::BatchId::new(alloy_primitives::U256::ZERO, alloy_primitives::FixedBytes::ZERO),
                    alloy_primitives::U256::ZERO,
                    alloy_primitives::FixedBytes::ZERO,
                    vec![],
                    cdk_types::ProofMetadata::default(),
                    0,
                ));
            }
        };
        Ok(Box::new(Box::pin(batch_stream)))
    }

    async fn next(&mut self) -> Result<Option<Batch>, crate::DatastreamError> {
        Ok(None)
    }

    async fn checkpoint(&self) -> Result<crate::Checkpoint, crate::DatastreamError> {
        Ok(crate::Checkpoint::default())
    }

    async fn set_checkpoint(&mut self, _checkpoint: crate::Checkpoint) -> Result<(), crate::DatastreamError> {
        Ok(())
    }

    async fn health_check(&self) -> Result<(), crate::DatastreamError> {
        // Try to connect to check health
        let _channel = Channel::from_shared(self.config.url.clone())
            .map_err(|e| crate::DatastreamError::ConnectionError(format!("Invalid gRPC URL: {}", e)))?
            .connect()
            .await
            .map_err(|e| crate::DatastreamError::ConnectionError(format!("Failed to connect to gRPC: {}", e)))?;
        Ok(())
    }

    async fn metadata(&self) -> Result<crate::SourceMetadata, crate::DatastreamError> {
        Ok(crate::SourceMetadata::new(
            "gRPC Source".to_string(),
            "1.0".to_string(),
            self.config.url.clone(),
            true,
        ))
    }
}