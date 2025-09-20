//! Checkpoint management for resumable batch ingestion

use alloy_primitives::{FixedBytes, U256};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::DatastreamError;

/// A checkpoint represents the state of batch ingestion
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Checkpoint {
    /// The last successfully processed batch ID
    pub last_batch_id: U256,
    /// The last successfully processed batch hash
    pub last_batch_hash: FixedBytes<32>,
    /// The L1 block number where the last batch was submitted
    pub last_l1_block: U256,
    /// Timestamp when the checkpoint was created
    pub timestamp: u64,
    /// Additional metadata for the checkpoint
    pub metadata: HashMap<String, String>,
}

impl Checkpoint {
    /// Create a new checkpoint
    pub fn new(
        last_batch_id: U256,
        last_batch_hash: FixedBytes<32>,
        last_l1_block: U256,
        timestamp: u64,
    ) -> Self {
        Self {
            last_batch_id,
            last_batch_hash,
            last_l1_block,
            timestamp,
            metadata: HashMap::new(),
        }
    }

    /// Add metadata to the checkpoint
    pub fn with_metadata(mut self, key: String, value: String) -> Self {
        self.metadata.insert(key, value);
        self
    }

    /// Get metadata value
    pub fn get_metadata(&self, key: &str) -> Option<&String> {
        self.metadata.get(key)
    }

    /// Check if this checkpoint is valid
    pub fn is_valid(&self) -> bool {
        !self.last_batch_hash.is_zero() && self.timestamp > 0
    }

    /// Create a checkpoint from a batch
    pub fn from_batch(batch: &cdk_types::Batch, timestamp: u64) -> Self {
        Self::new(
            batch.id.number,
            batch.id.hash,
            batch.l1_origin,
            timestamp,
        )
    }
}

impl Default for Checkpoint {
    fn default() -> Self {
        Self::new(
            U256::ZERO,
            FixedBytes::from([0u8; 32]),
            U256::ZERO,
            0,
        )
    }
}

/// Checkpoint storage trait for persisting checkpoints
#[async_trait::async_trait]
pub trait CheckpointStorage: Send + Sync {
    /// Save a checkpoint
    async fn save_checkpoint(&self, checkpoint: Checkpoint) -> Result<(), DatastreamError>;

    /// Load the latest checkpoint
    async fn load_checkpoint(&self) -> Result<Option<Checkpoint>, DatastreamError>;

    /// Delete a checkpoint
    async fn delete_checkpoint(&self) -> Result<(), DatastreamError>;
}

/// In-memory checkpoint storage for testing
#[derive(Debug, Default)]
pub struct MemoryCheckpointStorage {
    checkpoint: std::sync::Arc<std::sync::Mutex<Option<Checkpoint>>>,
}

#[async_trait::async_trait]
impl CheckpointStorage for MemoryCheckpointStorage {
    async fn save_checkpoint(&self, checkpoint: Checkpoint) -> Result<(), DatastreamError> {
        // In a real implementation, this would be stored persistently
        // For now, we just store it in memory
        let mut storage = self.checkpoint.lock().unwrap();
        *storage = Some(checkpoint);
        Ok(())
    }

    async fn load_checkpoint(&self) -> Result<Option<Checkpoint>, DatastreamError> {
        let storage = self.checkpoint.lock().unwrap();
        Ok(storage.clone())
    }

    async fn delete_checkpoint(&self) -> Result<(), DatastreamError> {
        let mut storage = self.checkpoint.lock().unwrap();
        *storage = None;
        Ok(())
    }
}

impl Clone for MemoryCheckpointStorage {
    fn clone(&self) -> Self {
        Self {
            checkpoint: self.checkpoint.clone(),
        }
    }
}
