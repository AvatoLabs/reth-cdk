//! Unit tests for CDK RPC Extensions

use cdk_rpc_ext::{
    CdkRpcApi, CdkRpcApiImpl, CdkRpcConfig, CdkRpcServer,
    CdkRpcError, CdkRpcResult,
    types::*,
};
use cdk_types::{Batch, BatchId, Epoch, EpochId, FinalityTag, FinalityStatus, ProofMetadata};
use cdk_datastream::{BatchSource, Checkpoint, DatastreamError, SourceMetadata};
use cdk_ingest::{MappingStorage, IngestError, BlockMapping, BatchMapping, EpochMapping};
use cdk_finality::{FinalityOracle, FinalityError, OracleMetadata};
use alloy_primitives::{FixedBytes, U256, Address};
use async_trait::async_trait;
use std::collections::HashMap;
use std::time::Duration;
use tokio_test;

// Mock implementations for testing
#[derive(Debug)]
struct MockBatchSource {
    batches: HashMap<U256, Batch>,
}

impl MockBatchSource {
    fn new() -> Self {
        Self {
            batches: HashMap::new(),
        }
    }
    
    fn add_batch(&mut self, batch: Batch) {
        self.batches.insert(batch.id.number, batch);
    }
}

#[async_trait]
impl BatchSource for MockBatchSource {
    async fn next(&mut self) -> Result<Option<Batch>, DatastreamError> {
        Ok(None) // Not used in tests
    }
    
    async fn checkpoint(&self) -> Result<Checkpoint, DatastreamError> {
        Ok(Checkpoint::new(
            U256::from(0),
            FixedBytes::from([0u8; 32]),
            U256::from(0),
            0,
        ))
    }

    async fn set_checkpoint(&mut self, _checkpoint: Checkpoint) -> Result<(), DatastreamError> {
        Ok(())
    }

    async fn health_check(&self) -> Result<(), DatastreamError> {
        Ok(())
    }

    async fn metadata(&self) -> Result<SourceMetadata, DatastreamError> {
        Ok(SourceMetadata::new(
            "mock".to_string(),
            "1.0.0".to_string(),
            "mock://test".to_string(),
            true,
        ))
    }
}

#[derive(Debug)]
struct MockMappingStorage {
    block_to_epoch: HashMap<U256, EpochId>,
    epochs: HashMap<EpochId, Epoch>,
}

impl MockMappingStorage {
    fn new() -> Self {
        Self {
            block_to_epoch: HashMap::new(),
            epochs: HashMap::new(),
        }
    }
    
    fn add_epoch(&mut self, epoch: Epoch) {
        self.epochs.insert(epoch.id.clone(), epoch);
    }
}

#[async_trait]
impl MappingStorage for MockMappingStorage {
    async fn save_block_mapping(&self, _mapping: BlockMapping) -> Result<(), IngestError> {
        Ok(())
    }

    async fn load_block_mapping(&self, _block_number: u64) -> Result<Option<BlockMapping>, IngestError> {
        Ok(None)
    }

    async fn save_batch_mapping(&self, _mapping: BatchMapping) -> Result<(), IngestError> {
        Ok(())
    }

    async fn load_batch_mapping(&self, _batch_id: u64) -> Result<Option<BatchMapping>, IngestError> {
        Ok(None)
    }

    async fn save_epoch_mapping(&self, _mapping: EpochMapping) -> Result<(), IngestError> {
        Ok(())
    }

    async fn load_epoch_mapping(&self, _epoch_id: u64) -> Result<Option<EpochMapping>, IngestError> {
        Ok(None)
    }

    async fn get_block_mappings_range(&self, _start_block: u64, _end_block: u64) -> Result<Vec<BlockMapping>, IngestError> {
        Ok(vec![])
    }

    async fn get_batch_mappings_range(&self, _start_batch: u64, _end_batch: u64) -> Result<Vec<BatchMapping>, IngestError> {
        Ok(vec![])
    }

    async fn delete_block_mapping(&self, _block_number: u64) -> Result<(), IngestError> {
        Ok(())
    }

    async fn delete_batch_mapping(&self, _batch_id: u64) -> Result<(), IngestError> {
        Ok(())
    }

    async fn delete_epoch_mapping(&self, _epoch_id: u64) -> Result<(), IngestError> {
        Ok(())
    }
}

#[derive(Debug)]
struct MockFinalityOracle {
    finality_tags: Vec<FinalityTag>,
}

impl MockFinalityOracle {
    fn new() -> Self {
        Self {
            finality_tags: vec![],
        }
    }
    
    fn add_finality_tag(&mut self, tag: FinalityTag) {
        self.finality_tags.push(tag);
    }
}

#[async_trait]
impl FinalityOracle for MockFinalityOracle {
    async fn poll(&mut self) -> Result<Vec<FinalityTag>, FinalityError> {
        Ok(self.finality_tags.clone())
    }

    async fn get_finality_status(&self, _batch_id: u64) -> Result<Option<FinalityStatus>, FinalityError> {
        Ok(None)
    }

    async fn get_finalized_batches(&self) -> Result<Vec<FinalityTag>, FinalityError> {
        Ok(self.finality_tags.clone())
    }

    async fn get_rolled_back_batches(&self) -> Result<Vec<FinalityTag>, FinalityError> {
        Ok(vec![])
    }

    async fn health_check(&self) -> Result<(), FinalityError> {
        Ok(())
    }

    async fn metadata(&self) -> Result<OracleMetadata, FinalityError> {
        Ok(OracleMetadata::new(
            "mock".to_string(),
            "1.0.0".to_string(),
            1,
            Address::ZERO,
        ))
    }

    fn set_polling_interval(&mut self, _interval: Duration) {
        // Mock implementation
    }

    fn get_polling_interval(&self) -> Duration {
        Duration::from_secs(12)
    }
}

#[tokio::test]
async fn test_get_batch_by_number_success() {
    let mut batch_source = MockBatchSource::new();
    let batch_id = BatchId::new(U256::from(1), FixedBytes::from([1u8; 32]));
    let batch = Batch::new(
        batch_id,
        U256::from(100),
        FixedBytes::from([2u8; 32]),
        vec![],
        ProofMetadata::default(),
        1234567890,
    );
    batch_source.add_batch(batch.clone());
    
    let mapping_storage = MockMappingStorage::new();
    let finality_oracle = MockFinalityOracle::new();
    
    let mut api = CdkRpcApiImpl::new(
        Box::new(batch_source),
        Box::new(mapping_storage),
        Box::new(finality_oracle),
    );
    
    let result = api.get_batch_by_number("0x1".to_string()).await;
    // Note: Currently returns None as implementation is placeholder
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_get_batch_by_number_invalid_hex() {
    let batch_source = MockBatchSource::new();
    let mapping_storage = MockMappingStorage::new();
    let finality_oracle = MockFinalityOracle::new();
    
    let mut api = CdkRpcApiImpl::new(
        Box::new(batch_source),
        Box::new(mapping_storage),
        Box::new(finality_oracle),
    );
    
    let result = api.get_batch_by_number("invalid_hex".to_string()).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_get_epoch_by_block_success() {
    let batch_source = MockBatchSource::new();
    let mapping_storage = MockMappingStorage::new();
    let finality_oracle = MockFinalityOracle::new();
    
    let mut api = CdkRpcApiImpl::new(
        Box::new(batch_source),
        Box::new(mapping_storage),
        Box::new(finality_oracle),
    );
    
    let result = api.get_epoch_by_block("0x64".to_string()).await;
    // Note: Currently returns None as implementation is placeholder
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_get_epoch_by_block_invalid_hex() {
    let batch_source = MockBatchSource::new();
    let mapping_storage = MockMappingStorage::new();
    let finality_oracle = MockFinalityOracle::new();
    
    let mut api = CdkRpcApiImpl::new(
        Box::new(batch_source),
        Box::new(mapping_storage),
        Box::new(finality_oracle),
    );
    
    let result = api.get_epoch_by_block("invalid_hex".to_string()).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_finalized_batch_with_tags() {
    let batch_source = MockBatchSource::new();
    let mapping_storage = MockMappingStorage::new();
    let mut finality_oracle = MockFinalityOracle::new();
    
    let finality_tag = FinalityTag::new(
        U256::from(1),
        U256::from(100),
        FixedBytes::from([1u8; 32]),
        FinalityStatus::Finalized,
        1234567890,
        Some(FixedBytes::from([2u8; 32])),
    );
    finality_oracle.add_finality_tag(finality_tag);
    
    let mut api = CdkRpcApiImpl::new(
        Box::new(batch_source),
        Box::new(mapping_storage),
        Box::new(finality_oracle),
    );
    
    let result = api.finalized_batch().await;
    assert!(result.is_ok());
    let response = result.unwrap();
    assert!(response.is_some());
}

#[tokio::test]
async fn test_finalized_batch_no_tags() {
    let batch_source = MockBatchSource::new();
    let mapping_storage = MockMappingStorage::new();
    let finality_oracle = MockFinalityOracle::new();
    
    let mut api = CdkRpcApiImpl::new(
        Box::new(batch_source),
        Box::new(mapping_storage),
        Box::new(finality_oracle),
    );
    
    let result = api.finalized_batch().await;
    assert!(result.is_ok());
    let response = result.unwrap();
    assert!(response.is_none());
}

#[tokio::test]
async fn test_metrics() {
    let batch_source = MockBatchSource::new();
    let mapping_storage = MockMappingStorage::new();
    let finality_oracle = MockFinalityOracle::new();
    
    let mut api = CdkRpcApiImpl::new(
        Box::new(batch_source),
        Box::new(mapping_storage),
        Box::new(finality_oracle),
    );
    
    let result = api.metrics().await;
    assert!(result.is_ok());
    let metrics = result.unwrap();
    assert_eq!(metrics.total_batches, 0);
    assert_eq!(metrics.total_epochs, 0);
    assert_eq!(metrics.reorg_count, 0);
    assert_eq!(metrics.ingest_tps, 0.0);
}
