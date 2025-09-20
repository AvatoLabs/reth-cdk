//! Block, batch, and epoch mapping management

use crate::{BlockMapping, BatchMapping, EpochMapping, IngestResult, AssemblyStats};
use alloy_primitives::FixedBytes;
use std::collections::HashMap;
use tracing::debug;

/// Mapping storage trait for persisting block/batch/epoch mappings
#[async_trait::async_trait]
pub trait MappingStorage: Send + Sync {
    /// Save block mapping
    async fn save_block_mapping(&self, mapping: BlockMapping) -> IngestResult<()>;

    /// Load block mapping by block number
    async fn load_block_mapping(&self, block_number: u64) -> IngestResult<Option<BlockMapping>>;

    /// Save batch mapping
    async fn save_batch_mapping(&self, mapping: BatchMapping) -> IngestResult<()>;

    /// Load batch mapping by batch ID
    async fn load_batch_mapping(&self, batch_id: u64) -> IngestResult<Option<BatchMapping>>;

    /// Save epoch mapping
    async fn save_epoch_mapping(&self, mapping: EpochMapping) -> IngestResult<()>;

    /// Load epoch mapping by epoch ID
    async fn load_epoch_mapping(&self, epoch_id: u64) -> IngestResult<Option<EpochMapping>>;

    /// Get all block mappings in a range
    async fn get_block_mappings_range(
        &self,
        start_block: u64,
        end_block: u64,
    ) -> IngestResult<Vec<BlockMapping>>;

    /// Get all batch mappings in a range
    async fn get_batch_mappings_range(
        &self,
        start_batch: u64,
        end_batch: u64,
    ) -> IngestResult<Vec<BatchMapping>>;

    /// Delete block mapping
    async fn delete_block_mapping(&self, block_number: u64) -> IngestResult<()>;

    /// Delete batch mapping
    async fn delete_batch_mapping(&self, batch_id: u64) -> IngestResult<()>;

    /// Delete epoch mapping
    async fn delete_epoch_mapping(&self, epoch_id: u64) -> IngestResult<()>;
}

/// In-memory mapping storage for testing
#[derive(Debug, Default)]
pub struct MemoryMappingStorage {
    block_mappings: std::sync::Arc<std::sync::Mutex<HashMap<u64, BlockMapping>>>,
    batch_mappings: std::sync::Arc<std::sync::Mutex<HashMap<u64, BatchMapping>>>,
    epoch_mappings: std::sync::Arc<std::sync::Mutex<HashMap<u64, EpochMapping>>>,
}

#[async_trait::async_trait]
impl MappingStorage for MemoryMappingStorage {
    async fn save_block_mapping(&self, mapping: BlockMapping) -> IngestResult<()> {
        let block_number = mapping.block_number;
        let mut storage = self.block_mappings.lock().unwrap();
        storage.insert(block_number, mapping);
        debug!("Saved block mapping for block {}", block_number);
        Ok(())
    }

    async fn load_block_mapping(&self, block_number: u64) -> IngestResult<Option<BlockMapping>> {
        let storage = self.block_mappings.lock().unwrap();
        Ok(storage.get(&block_number).cloned())
    }

    async fn save_batch_mapping(&self, mapping: BatchMapping) -> IngestResult<()> {
        let batch_id = mapping.batch_id;
        let mut storage = self.batch_mappings.lock().unwrap();
        storage.insert(batch_id, mapping);
        debug!("Saved batch mapping for batch {}", batch_id);
        Ok(())
    }

    async fn load_batch_mapping(&self, batch_id: u64) -> IngestResult<Option<BatchMapping>> {
        let storage = self.batch_mappings.lock().unwrap();
        Ok(storage.get(&batch_id).cloned())
    }

    async fn save_epoch_mapping(&self, mapping: EpochMapping) -> IngestResult<()> {
        let epoch_id = mapping.epoch_id;
        let mut storage = self.epoch_mappings.lock().unwrap();
        storage.insert(epoch_id, mapping);
        debug!("Saved epoch mapping for epoch {}", epoch_id);
        Ok(())
    }

    async fn load_epoch_mapping(&self, epoch_id: u64) -> IngestResult<Option<EpochMapping>> {
        let storage = self.epoch_mappings.lock().unwrap();
        Ok(storage.get(&epoch_id).cloned())
    }

    async fn get_block_mappings_range(
        &self,
        start_block: u64,
        end_block: u64,
    ) -> IngestResult<Vec<BlockMapping>> {
        let storage = self.block_mappings.lock().unwrap();
        let mappings: Vec<BlockMapping> = storage
            .iter()
            .filter(|(block_num, _)| **block_num >= start_block && **block_num <= end_block)
            .map(|(_, mapping)| mapping.clone())
            .collect();
        Ok(mappings)
    }

    async fn get_batch_mappings_range(
        &self,
        start_batch: u64,
        end_batch: u64,
    ) -> IngestResult<Vec<BatchMapping>> {
        let storage = self.batch_mappings.lock().unwrap();
        let mappings: Vec<BatchMapping> = storage
            .iter()
            .filter(|(batch_id, _)| **batch_id >= start_batch && **batch_id <= end_batch)
            .map(|(_, mapping)| mapping.clone())
            .collect();
        Ok(mappings)
    }

    async fn delete_block_mapping(&self, block_number: u64) -> IngestResult<()> {
        let mut storage = self.block_mappings.lock().unwrap();
        storage.remove(&block_number);
        debug!("Deleted block mapping for block {}", block_number);
        Ok(())
    }

    async fn delete_batch_mapping(&self, batch_id: u64) -> IngestResult<()> {
        let mut storage = self.batch_mappings.lock().unwrap();
        storage.remove(&batch_id);
        debug!("Deleted batch mapping for batch {}", batch_id);
        Ok(())
    }

    async fn delete_epoch_mapping(&self, epoch_id: u64) -> IngestResult<()> {
        let mut storage = self.epoch_mappings.lock().unwrap();
        storage.remove(&epoch_id);
        debug!("Deleted epoch mapping for epoch {}", epoch_id);
        Ok(())
    }
}

/// Mapping manager for handling block/batch/epoch relationships
pub struct MappingManager {
    storage: Box<dyn MappingStorage>,
    stats: AssemblyStats,
}

impl MappingManager {
    /// Create a new mapping manager
    pub fn new(storage: Box<dyn MappingStorage>) -> Self {
        Self {
            storage,
            stats: AssemblyStats::default(),
        }
    }

    /// Create block mapping from batch data
    pub fn create_block_mapping(
        &self,
        block_number: u64,
        block_hash: FixedBytes<32>,
        batch_id: u64,
        batch_index: u32,
        epoch_id: u64,
    ) -> BlockMapping {
        BlockMapping {
            block_number,
            block_hash,
            batch_id,
            batch_index,
            epoch_id,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }

    /// Create batch mapping from batch data
    pub fn create_batch_mapping(
        &self,
        batch_id: u64,
        batch_hash: FixedBytes<32>,
        start_block: u64,
        end_block: u64,
        epoch_id: u64,
    ) -> BatchMapping {
        BatchMapping {
            batch_id,
            batch_hash,
            start_block,
            end_block,
            block_count: (end_block - start_block + 1) as u32,
            epoch_id,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }

    /// Create epoch mapping from epoch data
    pub fn create_epoch_mapping(
        &self,
        epoch_id: u64,
        epoch_hash: FixedBytes<32>,
        start_block: u64,
        end_block: u64,
        batch_count: u32,
    ) -> EpochMapping {
        EpochMapping {
            epoch_id,
            epoch_hash,
            start_block,
            end_block,
            block_count: (end_block - start_block + 1) as u32,
            batch_count,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }

    /// Save mappings and update statistics
    pub async fn save_mappings(&mut self, mappings: Vec<BlockMapping>) -> IngestResult<()> {
        for mapping in mappings {
            self.storage.save_block_mapping(mapping.clone()).await?;
            self.stats.total_blocks += 1;
        }
        self.stats.last_assembly = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        Ok(())
    }

    /// Get current statistics
    pub fn get_stats(&self) -> &AssemblyStats {
        &self.stats
    }
}

impl Clone for MemoryMappingStorage {
    fn clone(&self) -> Self {
        Self {
            block_mappings: self.block_mappings.clone(),
            batch_mappings: self.batch_mappings.clone(),
            epoch_mappings: self.epoch_mappings.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy_primitives::{FixedBytes, U256};

    #[tokio::test]
    async fn test_memory_mapping_storage() {
        let storage = MemoryMappingStorage::default();
        
        let block_mapping = BlockMapping {
            block_number: 100,
            block_hash: FixedBytes::from([1u8; 32]),
            batch_id: 1,
            batch_index: 0,
            epoch_id: 1,
            timestamp: 1234567890,
        };

        // Save and load block mapping
        storage.save_block_mapping(block_mapping.clone()).await.unwrap();
        let loaded = storage.load_block_mapping(100).await.unwrap();
        assert_eq!(loaded, Some(block_mapping));

        // Delete block mapping
        storage.delete_block_mapping(100).await.unwrap();
        let loaded = storage.load_block_mapping(100).await.unwrap();
        assert_eq!(loaded, None);
    }

    #[tokio::test]
    async fn test_mapping_manager() {
        let storage = Box::new(MemoryMappingStorage::default());
        let manager = MappingManager::new(storage);

        let block_mapping = manager.create_block_mapping(
            100,
            FixedBytes::from([1u8; 32]),
            1,
            0,
            1,
        );

        assert_eq!(block_mapping.block_number, 100);
        assert_eq!(block_mapping.batch_id, 1);
        assert_eq!(block_mapping.epoch_id, 1);
    }

    #[test]
    fn test_batch_mapping_creation() {
        let storage = MemoryMappingStorage::default();
        let manager = MappingManager::new(Box::new(storage));

        let batch_mapping = manager.create_batch_mapping(
            1,
            FixedBytes::from([1u8; 32]),
            100,
            200,
            1,
        );

        assert_eq!(batch_mapping.batch_id, 1);
        assert_eq!(batch_mapping.start_block, 100);
        assert_eq!(batch_mapping.end_block, 200);
        assert_eq!(batch_mapping.block_count, 101);
    }

    #[test]
    fn test_epoch_mapping_creation() {
        let storage = MemoryMappingStorage::default();
        let manager = MappingManager::new(Box::new(storage));

        let epoch_mapping = manager.create_epoch_mapping(
            1,
            FixedBytes::from([1u8; 32]),
            100,
            200,
            5,
        );

        assert_eq!(epoch_mapping.epoch_id, 1);
        assert_eq!(epoch_mapping.start_block, 100);
        assert_eq!(epoch_mapping.end_block, 200);
        assert_eq!(epoch_mapping.block_count, 101);
        assert_eq!(epoch_mapping.batch_count, 5);
    }
}
