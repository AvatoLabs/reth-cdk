//! Core traits for block assembly

use cdk_types::Batch;
use crate::IngestError;
use async_trait::async_trait;
use std::fmt::Debug;

/// Input data for block assembly
#[derive(Debug, Clone, PartialEq)]
pub struct BlockInputs {
    /// Block number
    pub number: u64,
    /// Block hash
    pub hash: alloy_primitives::FixedBytes<32>,
    /// Parent hash
    pub parent_hash: alloy_primitives::FixedBytes<32>,
    /// State root
    pub state_root: alloy_primitives::FixedBytes<32>,
    /// Receipts root
    pub receipts_root: alloy_primitives::FixedBytes<32>,
    /// Transactions root
    pub transactions_root: alloy_primitives::FixedBytes<32>,
    /// Block timestamp
    pub timestamp: u64,
    /// Gas limit
    pub gas_limit: u64,
    /// Gas used
    pub gas_used: u64,
    /// Base fee per gas (EIP-1559)
    pub base_fee_per_gas: Option<u64>,
    /// Extra data
    pub extra_data: alloy_primitives::Bytes,
    /// Transactions
    pub transactions: Vec<TransactionInput>,
}

/// Transaction input data
#[derive(Debug, Clone, PartialEq)]
pub struct TransactionInput {
    /// Transaction hash
    pub hash: alloy_primitives::FixedBytes<32>,
    /// Transaction type
    pub tx_type: u8,
    /// Gas limit
    pub gas_limit: u64,
    /// Gas price
    pub gas_price: Option<u64>,
    /// Max fee per gas (EIP-1559)
    pub max_fee_per_gas: Option<u64>,
    /// Max priority fee per gas (EIP-1559)
    pub max_priority_fee_per_gas: Option<u64>,
    /// Nonce
    pub nonce: u64,
    /// Value
    pub value: alloy_primitives::U256,
    /// To address
    pub to: Option<alloy_primitives::Address>,
    /// Data
    pub data: alloy_primitives::Bytes,
    /// Access list (EIP-2930)
    pub access_list: Vec<AccessListItem>,
}

/// Access list item (EIP-2930)
#[derive(Debug, Clone, PartialEq)]
pub struct AccessListItem {
    /// Address
    pub address: alloy_primitives::Address,
    /// Storage keys
    pub storage_keys: Vec<alloy_primitives::FixedBytes<32>>,
}

/// A block assembler converts batches into standard blocks
#[async_trait]
pub trait BlockAssembler: Send + Sync + Debug {
    /// Assemble a batch into block inputs
    async fn assemble(&mut self, batch: &Batch) -> Result<Vec<BlockInputs>, IngestError>;

    /// Validate a batch before assembly
    async fn validate_batch(&self, batch: &Batch) -> Result<(), IngestError>;

    /// Get block mapping for a given block number
    async fn get_block_mapping(&self, block_number: u64) -> Result<Option<BlockMapping>, IngestError>;

    /// Get batch mapping for a given batch ID
    async fn get_batch_mapping(&self, batch_id: u64) -> Result<Option<BatchMapping>, IngestError>;

    /// Get epoch mapping for a given epoch ID
    async fn get_epoch_mapping(&self, epoch_id: u64) -> Result<Option<EpochMapping>, IngestError>;

    /// Update mappings after successful block import
    async fn update_mappings(&mut self, mappings: Vec<BlockMapping>) -> Result<(), IngestError>;

    /// Get assembly statistics
    async fn get_stats(&self) -> Result<AssemblyStats, IngestError>;
}

/// Block mapping information
#[derive(Debug, Clone, PartialEq)]
pub struct BlockMapping {
    /// Block number
    pub block_number: u64,
    /// Block hash
    pub block_hash: alloy_primitives::FixedBytes<32>,
    /// Batch ID this block belongs to
    pub batch_id: u64,
    /// Batch index within the batch
    pub batch_index: u32,
    /// Epoch ID this block belongs to
    pub epoch_id: u64,
    /// Timestamp when mapping was created
    pub timestamp: u64,
}

/// Batch mapping information
#[derive(Debug, Clone, PartialEq)]
pub struct BatchMapping {
    /// Batch ID
    pub batch_id: u64,
    /// Batch hash
    pub batch_hash: alloy_primitives::FixedBytes<32>,
    /// Start block number
    pub start_block: u64,
    /// End block number
    pub end_block: u64,
    /// Block count
    pub block_count: u32,
    /// Epoch ID this batch belongs to
    pub epoch_id: u64,
    /// Timestamp when mapping was created
    pub timestamp: u64,
}

/// Epoch mapping information
#[derive(Debug, Clone, PartialEq)]
pub struct EpochMapping {
    /// Epoch ID
    pub epoch_id: u64,
    /// Epoch hash
    pub epoch_hash: alloy_primitives::FixedBytes<32>,
    /// Start block number
    pub start_block: u64,
    /// End block number
    pub end_block: u64,
    /// Block count
    pub block_count: u32,
    /// Batch count
    pub batch_count: u32,
    /// Timestamp when mapping was created
    pub timestamp: u64,
}

/// Assembly statistics
#[derive(Debug, Clone, PartialEq)]
pub struct AssemblyStats {
    /// Total blocks assembled
    pub total_blocks: u64,
    /// Total batches processed
    pub total_batches: u64,
    /// Total epochs processed
    pub total_epochs: u64,
    /// Average blocks per batch
    pub avg_blocks_per_batch: f64,
    /// Average batches per epoch
    pub avg_batches_per_epoch: f64,
    /// Last assembly timestamp
    pub last_assembly: u64,
}

impl Default for AssemblyStats {
    fn default() -> Self {
        Self {
            total_blocks: 0,
            total_batches: 0,
            total_epochs: 0,
            avg_blocks_per_batch: 0.0,
            avg_batches_per_epoch: 0.0,
            last_assembly: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy_primitives::{FixedBytes, U256};

    #[test]
    fn test_block_inputs_creation() {
        let block = BlockInputs {
            number: 100,
            hash: FixedBytes::from([1u8; 32]),
            parent_hash: FixedBytes::from([2u8; 32]),
            state_root: FixedBytes::from([3u8; 32]),
            receipts_root: FixedBytes::from([4u8; 32]),
            transactions_root: FixedBytes::from([5u8; 32]),
            timestamp: 1234567890,
            gas_limit: 30000000,
            gas_used: 15000000,
            base_fee_per_gas: Some(1000000000),
            extra_data: alloy_primitives::Bytes::new(),
            transactions: vec![],
        };

        assert_eq!(block.number, 100);
        assert_eq!(block.gas_limit, 30000000);
    }

    #[test]
    fn test_transaction_input_creation() {
        let tx = TransactionInput {
            hash: FixedBytes::from([1u8; 32]),
            tx_type: 2, // EIP-1559
            gas_limit: 21000,
            gas_price: None,
            max_fee_per_gas: Some(1000000000),
            max_priority_fee_per_gas: Some(100000000),
            nonce: 1,
            value: U256::from(1000000000000000000u64), // 1 ETH
            to: Some(alloy_primitives::Address::from([2u8; 20])),
            data: alloy_primitives::Bytes::new(),
            access_list: vec![],
        };

        assert_eq!(tx.tx_type, 2);
        assert_eq!(tx.gas_limit, 21000);
    }

    #[test]
    fn test_assembly_stats_default() {
        let stats = AssemblyStats::default();
        assert_eq!(stats.total_blocks, 0);
        assert_eq!(stats.total_batches, 0);
        assert_eq!(stats.avg_blocks_per_batch, 0.0);
    }
}
