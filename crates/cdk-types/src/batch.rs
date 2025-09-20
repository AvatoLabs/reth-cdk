//! Batch types for CDK integration
//!
//! A `Batch` represents a collection of blocks that were submitted to L1
//! as a single unit. Each batch contains metadata about its L1 origin
//! and proof information for data availability verification.

use alloy_primitives::{Bytes, FixedBytes, U256};
use serde::{Deserialize, Serialize};

/// A batch of blocks submitted to L1
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Batch {
    /// Unique identifier for this batch
    pub id: BatchId,
    /// L1 block number where this batch was submitted
    pub l1_origin: U256,
    /// L1 block hash where this batch was submitted
    pub l1_origin_hash: FixedBytes<32>,
    /// Blocks contained in this batch
    pub blocks: Vec<BlockInBatch>,
    /// Proof metadata for data availability verification
    pub proof_meta: ProofMetadata,
    /// Timestamp when batch was created
    pub timestamp: u64,
}

/// Unique identifier for a batch
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BatchId {
    /// Sequential batch number
    pub number: U256,
    /// Hash of the batch contents
    pub hash: FixedBytes<32>,
}

/// A block within a batch
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BlockInBatch {
    /// Block number within the batch
    pub batch_index: u32,
    /// Block hash
    pub hash: FixedBytes<32>,
    /// Block number in the L2 chain
    pub number: U256,
    /// Parent block hash
    pub parent_hash: FixedBytes<32>,
    /// State root
    pub state_root: FixedBytes<32>,
    /// Transaction root
    pub tx_root: FixedBytes<32>,
    /// Receipt root
    pub receipt_root: FixedBytes<32>,
    /// Block timestamp
    pub timestamp: u64,
}

/// Proof metadata for data availability verification
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProofMetadata {
    /// Data availability proof
    pub data_proof: Bytes,
    /// Celestia namespace ID
    pub namespace_id: FixedBytes<8>,
    /// Celestia commitment
    pub commitment: FixedBytes<32>,
    /// Proof of inclusion in Celestia
    pub inclusion_proof: Bytes,
}

impl Default for ProofMetadata {
    fn default() -> Self {
        Self {
            data_proof: Bytes::new(),
            namespace_id: FixedBytes::from([0u8; 8]),
            commitment: FixedBytes::from([0u8; 32]),
            inclusion_proof: Bytes::new(),
        }
    }
}

impl Batch {
    /// Create a new batch
    pub fn new(
        id: BatchId,
        l1_origin: U256,
        l1_origin_hash: FixedBytes<32>,
        blocks: Vec<BlockInBatch>,
        proof_meta: ProofMetadata,
        timestamp: u64,
    ) -> Self {
        Self {
            id,
            l1_origin,
            l1_origin_hash,
            blocks,
            proof_meta,
            timestamp,
        }
    }

    /// Get the number of blocks in this batch
    pub fn block_count(&self) -> usize {
        self.blocks.len()
    }

    /// Check if batch is empty
    pub fn is_empty(&self) -> bool {
        self.blocks.is_empty()
    }

    /// Get block by batch index
    pub fn get_block(&self, batch_index: u32) -> Option<&BlockInBatch> {
        self.blocks.iter().find(|b| b.batch_index == batch_index)
    }

    /// Get all block hashes in this batch
    pub fn block_hashes(&self) -> Vec<FixedBytes<32>> {
        self.blocks.iter().map(|b| b.hash).collect()
    }
}

impl BatchId {
    /// Create a new batch ID
    pub fn new(number: U256, hash: FixedBytes<32>) -> Self {
        Self { number, hash }
    }
}

impl BlockInBatch {
    /// Create a new block in batch
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        batch_index: u32,
        hash: FixedBytes<32>,
        number: U256,
        parent_hash: FixedBytes<32>,
        state_root: FixedBytes<32>,
        tx_root: FixedBytes<32>,
        receipt_root: FixedBytes<32>,
        timestamp: u64,
    ) -> Self {
        Self {
            batch_index,
            hash,
            number,
            parent_hash,
            state_root,
            tx_root,
            receipt_root,
            timestamp,
        }
    }
}

impl ProofMetadata {
    /// Create new proof metadata
    pub fn new(
        data_proof: Bytes,
        namespace_id: FixedBytes<8>,
        commitment: FixedBytes<32>,
        inclusion_proof: Bytes,
    ) -> Self {
        Self {
            data_proof,
            namespace_id,
            commitment,
            inclusion_proof,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy_primitives::{Bytes, FixedBytes, U256};

    #[test]
    fn test_batch_creation() {
        let batch_id = BatchId::new(U256::from(1), FixedBytes::from([1u8; 32]));
        let l1_origin_hash = FixedBytes::from([2u8; 32]);
        let proof_meta = ProofMetadata::new(
            Bytes::from(vec![1, 2, 3]),
            FixedBytes::from([3u8; 8]),
            FixedBytes::from([4u8; 32]),
            Bytes::from(vec![4, 5, 6]),
        );

        let batch = Batch::new(
            batch_id,
            U256::from(100),
            l1_origin_hash,
            vec![],
            proof_meta,
            1234567890,
        );

        assert_eq!(batch.l1_origin, U256::from(100));
        assert_eq!(batch.timestamp, 1234567890);
        assert!(batch.is_empty());
    }

    #[test]
    fn test_block_in_batch() {
        let block = BlockInBatch::new(
            0,
            FixedBytes::from([1u8; 32]),
            U256::from(1000),
            FixedBytes::from([2u8; 32]),
            FixedBytes::from([3u8; 32]),
            FixedBytes::from([4u8; 32]),
            FixedBytes::from([5u8; 32]),
            1234567890,
        );

        assert_eq!(block.batch_index, 0);
        assert_eq!(block.number, U256::from(1000));
    }
}
