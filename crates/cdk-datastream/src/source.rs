//! Core traits for batch data sources

use cdk_types::Batch;
use crate::{Checkpoint, DatastreamError};
use async_trait::async_trait;
use std::fmt::Debug;
use futures::Stream;

/// Stream of batches
pub type BatchStream = Box<dyn Stream<Item = Result<Batch, DatastreamError>> + Send + Unpin>;

/// A source that can provide batches of data
#[async_trait]
pub trait BatchSource: Send + Sync + Debug {
    /// Get the next batch from the source
    async fn next(&mut self) -> Result<Option<Batch>, DatastreamError>;

    /// Get the current checkpoint for resumable ingestion
    async fn checkpoint(&self) -> Result<Checkpoint, DatastreamError>;

    /// Set a checkpoint to resume from
    async fn set_checkpoint(&mut self, checkpoint: Checkpoint) -> Result<(), DatastreamError>;

    /// Check if the source is healthy and ready to provide data
    async fn health_check(&self) -> Result<(), DatastreamError>;

    /// Get metadata about the source
    async fn metadata(&self) -> Result<SourceMetadata, DatastreamError>;

    /// Fetch a stream of batches starting from a specific batch number
    async fn fetch_batch_stream(&self, start_batch_number: Option<u64>) -> Result<BatchStream, DatastreamError>;
}

/// Metadata about a data source
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct SourceMetadata {
    /// Human-readable name of the source
    pub name: String,
    /// Version of the source protocol
    pub version: String,
    /// URL or identifier of the source
    pub url: String,
    /// Whether the source supports checkpoints
    pub supports_checkpoints: bool,
    /// Maximum batch size supported
    pub max_batch_size: Option<u32>,
    /// Whether the source is currently available
    pub available: bool,
}

impl SourceMetadata {
    /// Create new source metadata
    pub fn new(
        name: String,
        version: String,
        url: String,
        supports_checkpoints: bool,
    ) -> Self {
        Self {
            name,
            version,
            url,
            supports_checkpoints,
            max_batch_size: None,
            available: true,
        }
    }

    /// Set maximum batch size
    pub fn with_max_batch_size(mut self, max_size: u32) -> Self {
        self.max_batch_size = Some(max_size);
        self
    }

    /// Set availability status
    pub fn with_availability(mut self, available: bool) -> Self {
        self.available = available;
        self
    }
}
