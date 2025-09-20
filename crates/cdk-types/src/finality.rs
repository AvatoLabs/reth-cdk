//! Finality types for CDK integration
//!
//! `FinalityTag` represents the finality status of batches as determined
//! by L1 contracts. This is crucial for determining when batches can be
//! considered final and when rollbacks should occur.

use alloy_primitives::{FixedBytes, U256};
use serde::{Deserialize, Serialize};

/// Finality status of a batch
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum FinalityStatus {
    /// Batch is pending finality
    Pending,
    /// Batch is finalized
    Finalized,
    /// Batch has been rolled back
    RolledBack,
}

/// A finality tag indicating the finality status of a batch
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FinalityTag {
    /// The batch ID this finality tag refers to
    pub batch_id: U256,
    /// L1 block number where finality was determined
    pub l1_block: U256,
    /// L1 block hash where finality was determined
    pub l1_block_hash: FixedBytes<32>,
    /// Finality status
    pub status: FinalityStatus,
    /// Timestamp when finality was determined
    pub timestamp: u64,
    /// Transaction hash that triggered this finality change
    pub tx_hash: Option<FixedBytes<32>>,
}

impl FinalityTag {
    /// Create a new finality tag
    pub fn new(
        batch_id: U256,
        l1_block: U256,
        l1_block_hash: FixedBytes<32>,
        status: FinalityStatus,
        timestamp: u64,
        tx_hash: Option<FixedBytes<32>>,
    ) -> Self {
        Self {
            batch_id,
            l1_block,
            l1_block_hash,
            status,
            timestamp,
            tx_hash,
        }
    }

    /// Check if this batch is finalized
    pub fn is_finalized(&self) -> bool {
        matches!(self.status, FinalityStatus::Finalized)
    }

    /// Check if this batch has been rolled back
    pub fn is_rolled_back(&self) -> bool {
        matches!(self.status, FinalityStatus::RolledBack)
    }

    /// Check if this batch is still pending
    pub fn is_pending(&self) -> bool {
        matches!(self.status, FinalityStatus::Pending)
    }
}

impl FinalityStatus {
    /// Get a human-readable string representation
    pub fn as_str(&self) -> &'static str {
        match self {
            FinalityStatus::Pending => "pending",
            FinalityStatus::Finalized => "finalized",
            FinalityStatus::RolledBack => "rolled_back",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy_primitives::{FixedBytes, U256};

    #[test]
    fn test_finality_tag() {
        let tag = FinalityTag::new(
            U256::from(1),
            U256::from(100),
            FixedBytes::from([1u8; 32]),
            FinalityStatus::Finalized,
            1234567890,
            Some(FixedBytes::from([2u8; 32])),
        );

        assert!(tag.is_finalized());
        assert!(!tag.is_rolled_back());
        assert!(!tag.is_pending());
    }

    #[test]
    fn test_finality_status_strings() {
        assert_eq!(FinalityStatus::Pending.as_str(), "pending");
        assert_eq!(FinalityStatus::Finalized.as_str(), "finalized");
        assert_eq!(FinalityStatus::RolledBack.as_str(), "rolled_back");
    }
}
