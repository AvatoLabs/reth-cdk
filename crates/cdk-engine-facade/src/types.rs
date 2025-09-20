//! Types for engine facade

use alloy_primitives::{Bytes, FixedBytes, U256};
use cdk_types::{Batch, BlockInBatch, FinalityTag};
use serde::{Deserialize, Serialize};

/// A block ready for import into the engine
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ImportableBlock {
    /// Block number
    pub number: U256,
    /// Block hash
    pub hash: FixedBytes<32>,
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
    /// Block data (RLP encoded)
    pub data: Bytes,
    /// Batch information
    pub batch_info: Option<BatchInfo>,
}

/// Information about the batch this block belongs to
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BatchInfo {
    /// Batch ID
    pub batch_id: U256,
    /// L1 origin block number
    pub l1_origin: U256,
    /// L1 origin block hash
    pub l1_origin_hash: FixedBytes<32>,
    /// Index within the batch
    pub batch_index: u32,
}

/// Result of block import operation
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ImportResult {
    /// Number of blocks imported
    pub blocks_imported: usize,
    /// Highest block number imported
    pub highest_block: U256,
    /// Whether any blocks were skipped
    pub blocks_skipped: bool,
}

/// Finality operation result
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FinalityResult {
    /// Block number marked as final
    pub final_block: U256,
    /// Number of blocks affected
    pub blocks_affected: usize,
}

/// Rollback operation result
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RollbackResult {
    /// Block number rolled back to
    pub rollback_block: U256,
    /// Number of blocks rolled back
    pub blocks_rolled_back: usize,
}

impl ImportableBlock {
    /// Create a new importable block
    pub fn new(
        number: U256,
        hash: FixedBytes<32>,
        parent_hash: FixedBytes<32>,
        state_root: FixedBytes<32>,
        tx_root: FixedBytes<32>,
        receipt_root: FixedBytes<32>,
        timestamp: u64,
        data: Bytes,
        batch_info: Option<BatchInfo>,
    ) -> Self {
        Self {
            number,
            hash,
            parent_hash,
            state_root,
            tx_root,
            receipt_root,
            timestamp,
            data,
            batch_info,
        }
    }

    /// Convert from a batch block
    pub fn from_batch_block(
        block: &BlockInBatch,
        batch: &Batch,
        data: Bytes,
    ) -> Self {
        let batch_info = BatchInfo {
            batch_id: batch.id.number,
            l1_origin: batch.l1_origin,
            l1_origin_hash: batch.l1_origin_hash,
            batch_index: block.batch_index,
        };

        Self::new(
            block.number,
            block.hash,
            block.parent_hash,
            block.state_root,
            block.tx_root,
            block.receipt_root,
            block.timestamp,
            data,
            Some(batch_info),
        )
    }
}

impl BatchInfo {
    /// Create new batch info
    pub fn new(
        batch_id: U256,
        l1_origin: U256,
        l1_origin_hash: FixedBytes<32>,
        batch_index: u32,
    ) -> Self {
        Self {
            batch_id,
            l1_origin,
            l1_origin_hash,
            batch_index,
        }
    }
}
