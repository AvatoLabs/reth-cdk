//! Integration tests for CDK RPC Extensions

use cdk_rpc_ext::{
    CdkRpcConfig, CdkRpcServer,
    CdkRpcError, CdkRpcResult,
};
use cdk_datastream::{BatchSource, Checkpoint, DatastreamError, SourceMetadata};
use cdk_ingest::{MappingStorage, IngestError, BlockMapping, BatchMapping, EpochMapping};
use cdk_finality::{FinalityOracle, FinalityError, OracleMetadata};
use alloy_primitives::{FixedBytes, U256, Address};
use async_trait::async_trait;
use std::collections::HashMap;
use std::time::Duration;
use tokio_test;

// Mock implementations for integration testing
#[derive(Debug)]
struct MockBatchSource {
    batches: HashMap<U256, cdk_types::Batch>,
}

impl MockBatchSource {
    fn new() -> Self {
        Self {
            batches: HashMap::new(),
        }
    }
}

#[async_trait]
impl BatchSource for MockBatchSource {
    async fn next(&mut self) -> Result<Option<cdk_types::Batch>, DatastreamError> {
        Ok(None)
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
    block_to_epoch: HashMap<U256, cdk_types::EpochId>,
    epochs: HashMap<cdk_types::EpochId, cdk_types::Epoch>,
}

impl MockMappingStorage {
    fn new() -> Self {
        Self {
            block_to_epoch: HashMap::new(),
            epochs: HashMap::new(),
        }
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
    finality_tags: Vec<cdk_types::FinalityTag>,
}

impl MockFinalityOracle {
    fn new() -> Self {
        Self {
            finality_tags: vec![],
        }
    }
}

#[async_trait]
impl FinalityOracle for MockFinalityOracle {
    async fn poll(&mut self) -> Result<Vec<cdk_types::FinalityTag>, FinalityError> {
        Ok(self.finality_tags.clone())
    }

    async fn get_finality_status(&self, _batch_id: u64) -> Result<Option<cdk_types::FinalityStatus>, FinalityError> {
        Ok(None)
    }

    async fn get_finalized_batches(&self) -> Result<Vec<cdk_types::FinalityTag>, FinalityError> {
        Ok(self.finality_tags.clone())
    }

    async fn get_rolled_back_batches(&self) -> Result<Vec<cdk_types::FinalityTag>, FinalityError> {
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
async fn test_server_config_default() {
    let config = CdkRpcConfig::default();
    assert!(config.enable_batch_queries);
    assert!(config.enable_epoch_queries);
    assert!(config.enable_finality_queries);
    assert!(config.enable_metrics);
    assert_eq!(config.max_batch_history, 1000);
    assert_eq!(config.max_epoch_history, 100);
}

#[tokio::test]
async fn test_server_config_custom() {
    let config = CdkRpcConfig {
        enable_batch_queries: false,
        enable_epoch_queries: true,
        enable_finality_queries: false,
        enable_metrics: true,
        max_batch_history: 500,
        max_epoch_history: 50,
        address: "127.0.0.1:8546".parse().unwrap(),
    };
    
    assert!(!config.enable_batch_queries);
    assert!(config.enable_epoch_queries);
    assert!(!config.enable_finality_queries);
    assert!(config.enable_metrics);
    assert_eq!(config.max_batch_history, 500);
    assert_eq!(config.max_epoch_history, 50);
}

#[tokio::test]
async fn test_server_creation() {
    let config = CdkRpcConfig::default();
    let batch_source = MockBatchSource::new();
    let mapping_storage = MockMappingStorage::new();
    let finality_oracle = MockFinalityOracle::new();
    
    let server = CdkRpcServer::new(
        config,
        Box::new(batch_source),
        Box::new(mapping_storage),
        Box::new(finality_oracle),
        "http://localhost:8545".to_string(),
    ).await.unwrap();
    
    // Server should be created successfully
    assert!(true);
}

#[tokio::test]
async fn test_rpc_module_creation() {
    let config = CdkRpcConfig::default();
    let batch_source = MockBatchSource::new();
    let mapping_storage = MockMappingStorage::new();
    let finality_oracle = MockFinalityOracle::new();
    
    let server = CdkRpcServer::new(
        config,
        Box::new(batch_source),
        Box::new(mapping_storage),
        Box::new(finality_oracle),
        "http://localhost:8545".to_string(),
    ).await.unwrap();
    
    // Server should be created successfully
    assert!(true);
}

#[tokio::test]
async fn test_error_conversions() {
    // Test error conversion from datastream error
    let datastream_error = cdk_datastream::DatastreamError::NetworkError("test".to_string());
    let rpc_error: CdkRpcError = datastream_error.into();
    assert!(matches!(rpc_error, CdkRpcError::DataSourceError(_)));
    
    // Test error conversion from ingest error
    let ingest_error = cdk_ingest::IngestError::InvalidBatchData("test".to_string());
    let rpc_error: CdkRpcError = ingest_error.into();
    assert!(matches!(rpc_error, CdkRpcError::InternalError(_)));
    
    // Test error conversion from finality error
    let finality_error = cdk_finality::FinalityError::L1RpcError("test".to_string());
    let rpc_error: CdkRpcError = finality_error.into();
    assert!(matches!(rpc_error, CdkRpcError::FinalityOracleError(_)));
}
