//! Batch and block validation

use cdk_types::{Batch, BlockInBatch};
use crate::{BlockInputs, IngestError, IngestResult};
use alloy_primitives::U256;
use tracing::{debug, warn};

/// Batch validator for ensuring data integrity
#[derive(Debug)]
pub struct BatchValidator {
    /// Maximum blocks per batch
    pub max_blocks_per_batch: u32,
    /// Maximum batch size in bytes
    pub max_batch_size_bytes: u64,
    /// Enable strict validation
    pub strict_mode: bool,
}

impl Default for BatchValidator {
    fn default() -> Self {
        Self {
            max_blocks_per_batch: 1000,
            max_batch_size_bytes: 10 * 1024 * 1024, // 10MB
            strict_mode: true,
        }
    }
}

impl BatchValidator {
    /// Create a new batch validator
    pub fn new(max_blocks_per_batch: u32, max_batch_size_bytes: u64, strict_mode: bool) -> Self {
        Self {
            max_blocks_per_batch,
            max_batch_size_bytes,
            strict_mode,
        }
    }

    /// Validate a batch
    pub async fn validate_batch(&self, batch: &Batch) -> IngestResult<()> {
        debug!("Validating batch {}", batch.id.number);

        // Check batch ID
        if batch.id.number == U256::ZERO {
            return Err(IngestError::InvalidBatchData("Batch ID cannot be zero".to_string()));
        }

        // Check L1 origin
        if batch.l1_origin == U256::ZERO {
            return Err(IngestError::InvalidBatchData("L1 origin cannot be zero".to_string()));
        }

        // Check block count
        if batch.blocks.len() > self.max_blocks_per_batch as usize {
            return Err(IngestError::InvalidBatchData(format!(
                "Too many blocks in batch: {} > {}",
                batch.blocks.len(),
                self.max_blocks_per_batch
            )));
        }

        // Check batch size
        let batch_size = self.estimate_batch_size(batch);
        if batch_size > self.max_batch_size_bytes {
            return Err(IngestError::InvalidBatchData(format!(
                "Batch too large: {} bytes > {} bytes",
                batch_size, self.max_batch_size_bytes
            )));
        }

        // Validate each block in the batch
        for (index, block) in batch.blocks.iter().enumerate() {
            self.validate_block_in_batch(block, index as u32).await?;
        }

        // Check block ordering
        if self.strict_mode {
            self.validate_block_ordering(&batch.blocks).await?;
        }

        debug!("Batch {} validation passed", batch.id.number);
        Ok(())
    }

    /// Validate a block within a batch
    async fn validate_block_in_batch(
        &self,
        block: &BlockInBatch,
        expected_index: u32,
    ) -> IngestResult<()> {
        // Check batch index
        if block.batch_index != expected_index {
            return Err(IngestError::InvalidBatchData(format!(
                "Block batch index mismatch: expected {}, got {}",
                expected_index, block.batch_index
            )));
        }

        // Check block number
        if block.number == U256::ZERO {
            return Err(IngestError::InvalidBatchData("Block number cannot be zero".to_string()));
        }

        // Check hashes
        if block.hash.is_zero() {
            return Err(IngestError::InvalidBatchData("Block hash cannot be zero".to_string()));
        }

        if block.parent_hash.is_zero() && block.number != U256::from(1) {
            return Err(IngestError::InvalidBatchData(
                "Non-genesis block must have non-zero parent hash".to_string(),
            ));
        }

        // Check roots
        if block.state_root.is_zero() {
            return Err(IngestError::InvalidBatchData("State root cannot be zero".to_string()));
        }

        if block.receipt_root.is_zero() {
            return Err(IngestError::InvalidBatchData("Receipts root cannot be zero".to_string()));
        }

        if block.tx_root.is_zero() {
            return Err(IngestError::InvalidBatchData("Transactions root cannot be zero".to_string()));
        }

        // Check timestamp
        if block.timestamp == 0 {
            return Err(IngestError::InvalidBatchData("Block timestamp cannot be zero".to_string()));
        }

        Ok(())
    }

    /// Validate block ordering within a batch
    async fn validate_block_ordering(&self, blocks: &[BlockInBatch]) -> IngestResult<()> {
        if blocks.is_empty() {
            return Ok(());
        }

        let mut prev_block_number = blocks[0].number;
        let mut prev_timestamp = blocks[0].timestamp;

        for block in blocks.iter().skip(1) {
            // Check block number ordering
            if block.number <= prev_block_number {
                return Err(IngestError::InvalidBatchData(format!(
                    "Block numbers not in order: {} <= {}",
                    block.number, prev_block_number
                )));
            }

            // Check timestamp ordering
            if block.timestamp < prev_timestamp {
                warn!(
                    "Block timestamp {} is before previous block timestamp {}",
                    block.timestamp, prev_timestamp
                );
                if self.strict_mode {
                    return Err(IngestError::InvalidBatchData(format!(
                        "Block timestamps not in order: {} < {}",
                        block.timestamp, prev_timestamp
                    )));
                }
            }

            prev_block_number = block.number;
            prev_timestamp = block.timestamp;
        }

        Ok(())
    }

    /// Estimate batch size in bytes
    fn estimate_batch_size(&self, batch: &Batch) -> u64 {
        let mut size = 0;

        // Batch metadata
        size += 32; // batch ID hash
        size += 32; // L1 origin hash
        size += 8;  // timestamp
        size += batch.proof_meta.data_proof.len() as u64;

        // Block data
        for _block in &batch.blocks {
            size += 32; // block hash
            size += 32; // parent hash
            size += 32; // state root
            size += 32; // receipts root
            size += 32; // transactions root
            size += 8;  // timestamp
            size += 8;  // number
            size += 4;  // batch index
        }

        size
    }

    /// Validate block inputs
    pub async fn validate_block_inputs(&self, block: &BlockInputs) -> IngestResult<()> {
        // Check block number
        if block.number == 0 {
            return Err(IngestError::InvalidBlockData("Block number cannot be zero".to_string()));
        }

        // Check hashes
        if block.hash.is_zero() {
            return Err(IngestError::InvalidBlockData("Block hash cannot be zero".to_string()));
        }

        if block.parent_hash.is_zero() && block.number != 1 {
            return Err(IngestError::InvalidBlockData(
                "Non-genesis block must have non-zero parent hash".to_string(),
            ));
        }

        // Check roots
        if block.state_root.is_zero() {
            return Err(IngestError::InvalidBlockData("State root cannot be zero".to_string()));
        }

        if block.receipts_root.is_zero() {
            return Err(IngestError::InvalidBlockData("Receipts root cannot be zero".to_string()));
        }

        if block.transactions_root.is_zero() {
            return Err(IngestError::InvalidBlockData("Transactions root cannot be zero".to_string()));
        }

        // Check gas
        if block.gas_limit == 0 {
            return Err(IngestError::InvalidBlockData("Gas limit cannot be zero".to_string()));
        }

        if block.gas_used > block.gas_limit {
            return Err(IngestError::InvalidBlockData(format!(
                "Gas used {} exceeds gas limit {}",
                block.gas_used, block.gas_limit
            )));
        }

        // Check timestamp
        if block.timestamp == 0 {
            return Err(IngestError::InvalidBlockData("Block timestamp cannot be zero".to_string()));
        }

        // Validate transactions
        for tx in &block.transactions {
            self.validate_transaction_input(tx).await?;
        }

        Ok(())
    }

    /// Validate transaction input
    async fn validate_transaction_input(&self, tx: &crate::TransactionInput) -> IngestResult<()> {
        // Check transaction hash
        if tx.hash.is_zero() {
            return Err(IngestError::InvalidBlockData("Transaction hash cannot be zero".to_string()));
        }

        // Check gas
        if tx.gas_limit == 0 {
            return Err(IngestError::InvalidBlockData("Transaction gas limit cannot be zero".to_string()));
        }

        // Check nonce
        if tx.nonce == 0 && tx.value > U256::ZERO {
            return Err(IngestError::InvalidBlockData(
                "Non-zero value transaction must have non-zero nonce".to_string(),
            ));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cdk_types::{Batch, BatchId, BlockInBatch, ProofMetadata};
    use alloy_primitives::U256;

    #[tokio::test]
    async fn test_batch_validator_default() {
        let validator = BatchValidator::default();
        assert_eq!(validator.max_blocks_per_batch, 1000);
        assert_eq!(validator.max_batch_size_bytes, 10 * 1024 * 1024);
        assert!(validator.strict_mode);
    }

    #[tokio::test]
    async fn test_batch_validation() {
        let validator = BatchValidator::default();
        
        let batch_id = BatchId::new(U256::from(1), FixedBytes::from([1u8; 32]));
        let proof_meta = ProofMetadata::new(
            alloy_primitives::Bytes::from(vec![1, 2, 3]),
            FixedBytes::from([3u8; 8]),
            FixedBytes::from([4u8; 32]),
            alloy_primitives::Bytes::from(vec![4, 5, 6]),
        );

        let batch = Batch::new(
            batch_id,
            U256::from(100),
            FixedBytes::from([2u8; 32]),
            vec![],
            proof_meta,
            1234567890,
        );

        // Empty batch should be valid
        validator.validate_batch(&batch).await.unwrap();
    }

    #[tokio::test]
    async fn test_batch_validation_invalid_id() {
        let validator = BatchValidator::default();
        
        let batch_id = BatchId::new(U256::ZERO, FixedBytes::from([1u8; 32]));
        let proof_meta = ProofMetadata::new(
            alloy_primitives::Bytes::from(vec![1, 2, 3]),
            FixedBytes::from([3u8; 8]),
            FixedBytes::from([4u8; 32]),
            alloy_primitives::Bytes::from(vec![4, 5, 6]),
        );

        let batch = Batch::new(
            batch_id,
            U256::from(100),
            FixedBytes::from([2u8; 32]),
            vec![],
            proof_meta,
            1234567890,
        );

        // Zero batch ID should be invalid
        let result = validator.validate_batch(&batch).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_block_inputs_validation() {
        let validator = BatchValidator::default();
        
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

        validator.validate_block_inputs(&block).await.unwrap();
    }

    #[tokio::test]
    async fn test_block_inputs_validation_invalid_gas() {
        let validator = BatchValidator::default();
        
        let block = BlockInputs {
            number: 100,
            hash: FixedBytes::from([1u8; 32]),
            parent_hash: FixedBytes::from([2u8; 32]),
            state_root: FixedBytes::from([3u8; 32]),
            receipts_root: FixedBytes::from([4u8; 32]),
            transactions_root: FixedBytes::from([5u8; 32]),
            timestamp: 1234567890,
            gas_limit: 10000000,
            gas_used: 15000000, // More than limit
            base_fee_per_gas: Some(1000000000),
            extra_data: alloy_primitives::Bytes::new(),
            transactions: vec![],
        };

        let result = validator.validate_block_inputs(&block).await;
        assert!(result.is_err());
    }
}
