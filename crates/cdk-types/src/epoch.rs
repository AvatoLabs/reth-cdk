//! Epoch types for CDK integration
//!
//! An `Epoch` represents a time period in the CDK system with defined
//! start and end block boundaries. Epochs are used for organizing
//! batches and tracking system state over time.

use alloy_primitives::{FixedBytes, U256};
use serde::{Deserialize, Serialize};
use std::hash::{Hash, Hasher};

/// An epoch representing a time period with block boundaries
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Epoch {
    /// Unique identifier for this epoch
    pub id: EpochId,
    /// Starting block number of this epoch
    pub start_block: U256,
    /// Ending block number of this epoch
    pub end_block: U256,
    /// Starting batch number of this epoch
    pub start_batch: U256,
    /// Ending batch number of this epoch
    pub end_batch: U256,
    /// Epoch start timestamp
    pub start_timestamp: u64,
    /// Epoch end timestamp
    pub end_timestamp: u64,
}

/// Unique identifier for an epoch
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EpochId {
    /// Sequential epoch number
    pub number: U256,
    /// Hash of the epoch metadata
    pub hash: FixedBytes<32>,
}

impl Epoch {
    /// Create a new epoch
    pub fn new(
        id: EpochId,
        start_block: U256,
        end_block: U256,
        start_batch: U256,
        end_batch: U256,
        start_timestamp: u64,
        end_timestamp: u64,
    ) -> Self {
        Self {
            id,
            start_block,
            end_block,
            start_batch,
            end_batch,
            start_timestamp,
            end_timestamp,
        }
    }

    /// Get the number of blocks in this epoch
    pub fn block_count(&self) -> U256 {
        self.end_block.saturating_sub(self.start_block) + U256::from(1)
    }

    /// Get the number of batches in this epoch
    pub fn batch_count(&self) -> U256 {
        self.end_batch.saturating_sub(self.start_batch) + U256::from(1)
    }

    /// Get the duration of this epoch in seconds
    pub fn duration_seconds(&self) -> u64 {
        self.end_timestamp.saturating_sub(self.start_timestamp)
    }

    /// Check if a block number is within this epoch
    pub fn contains_block(&self, block_number: U256) -> bool {
        block_number >= self.start_block && block_number <= self.end_block
    }

    /// Check if a batch number is within this epoch
    pub fn contains_batch(&self, batch_number: U256) -> bool {
        batch_number >= self.start_batch && batch_number <= self.end_batch
    }

    /// Check if epoch is empty (no blocks)
    pub fn is_empty(&self) -> bool {
        self.start_block > self.end_block
    }
}

impl EpochId {
    /// Create a new epoch ID
    pub fn new(number: U256, hash: FixedBytes<32>) -> Self {
        Self { number, hash }
    }
}

impl Hash for EpochId {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.number.hash(state);
        self.hash.hash(state);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy_primitives::{FixedBytes, U256};

    #[test]
    fn test_epoch_creation() {
        let epoch_id = EpochId::new(U256::from(1), FixedBytes::from([1u8; 32]));
        let epoch = Epoch::new(
            epoch_id,
            U256::from(100),
            U256::from(200),
            U256::from(10),
            U256::from(20),
            1234567890,
            1234567890 + 3600,
        );

        assert_eq!(epoch.block_count(), U256::from(101));
        assert_eq!(epoch.batch_count(), U256::from(11));
        assert_eq!(epoch.duration_seconds(), 3600);
        assert!(epoch.contains_block(U256::from(150)));
        assert!(!epoch.contains_block(U256::from(250)));
    }
}
