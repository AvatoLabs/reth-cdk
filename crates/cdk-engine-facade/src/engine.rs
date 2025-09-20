//! Main engine facade

use crate::{block_import::*, error::EngineFacadeError, finality::*, types::*};
use cdk_types::{Batch, FinalityTag};
use alloy_primitives::U256;

/// Main engine facade that provides unified access to Reth engine operations
pub struct EngineFacade {
    block_importer: Box<dyn BlockImporter + Send + Sync>,
    finality_manager: Box<dyn FinalityManager + Send + Sync>,
}

impl EngineFacade {
    /// Create a new engine facade
    pub fn new(
        block_importer: Box<dyn BlockImporter + Send + Sync>,
        finality_manager: Box<dyn FinalityManager + Send + Sync>,
    ) -> Self {
        Self {
            block_importer,
            finality_manager,
        }
    }

    /// Create a default engine facade with default implementations
    pub fn default() -> Self {
        Self::new(
            Box::new(DefaultBlockImporter::new()),
            Box::new(DefaultFinalityManager::new()),
        )
    }

    /// Import blocks from a batch
    pub async fn import_batch(&self, batch: &Batch, blocks: Vec<ImportableBlock>) -> Result<ImportResult, EngineFacadeError> {
        self.block_importer.import_batch(batch, blocks).await
    }

    /// Import a single block
    pub async fn import_block(&self, block: ImportableBlock) -> Result<(), EngineFacadeError> {
        self.block_importer.import_block(block).await
    }

    /// Mark a block as final
    pub async fn mark_final(&self, block_number: U256) -> Result<FinalityResult, EngineFacadeError> {
        self.finality_manager.mark_final(block_number).await
    }

    /// Process a finality tag
    pub async fn process_finality_tag(&self, tag: &FinalityTag) -> Result<FinalityResult, EngineFacadeError> {
        self.finality_manager.process_finality_tag(tag).await
    }

    /// Rollback to a specific block
    pub async fn rollback_to(&self, block_number: U256) -> Result<RollbackResult, EngineFacadeError> {
        // TODO: Implement rollback logic
        // This would involve unwinding the chain state to the specified block
        Ok(RollbackResult {
            rollback_block: block_number,
            blocks_rolled_back: 0,
        })
    }

    /// Get current head block
    pub async fn get_head_block(&self) -> Result<U256, EngineFacadeError> {
        self.block_importer.get_head_block().await
    }

    /// Get current final block
    pub async fn get_final_block(&self) -> Result<U256, EngineFacadeError> {
        self.finality_manager.get_final_block().await
    }

    /// Check if a block exists
    pub async fn block_exists(&self, block_number: U256) -> Result<bool, EngineFacadeError> {
        self.block_importer.block_exists(block_number).await
    }

    /// Check if a block is final
    pub async fn is_final(&self, block_number: U256) -> Result<bool, EngineFacadeError> {
        self.finality_manager.is_final(block_number).await
    }
}

impl Default for EngineFacade {
    fn default() -> Self {
        Self::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy_primitives::{Bytes, FixedBytes, U256};

    #[tokio::test]
    async fn test_engine_facade_creation() {
        let facade = EngineFacade::default();
        let head = facade.get_head_block().await.unwrap();
        assert_eq!(head, U256::ZERO);
    }

    #[tokio::test]
    async fn test_block_import() {
        let facade = EngineFacade::default();
        let block = ImportableBlock::new(
            U256::from(1),
            FixedBytes::from([1u8; 32]),
            FixedBytes::from([0u8; 32]),
            FixedBytes::from([2u8; 32]),
            FixedBytes::from([3u8; 32]),
            FixedBytes::from([4u8; 32]),
            1234567890,
            Bytes::new(),
            None,
        );

        let result = facade.import_block(block).await;
        assert!(result.is_ok());
    }
}
