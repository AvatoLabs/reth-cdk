//! Integration tests for CDK snapshot module

use cdk_snap::*;
use cdk_snap::converter::{DatabaseConverter, RethToErigonConverter, ErigonToRethConverter};
use cdk_snap::validator::SnapValidator;
use tempfile::TempDir;

#[test]
fn test_reth_to_erigon_conversion() {
    let temp_dir = TempDir::new().unwrap();
    let source_path = temp_dir.path().join("source");
    let target_path = temp_dir.path().join("target");
    
    // Create dummy source file
    std::fs::write(&source_path, b"dummy source data").unwrap();
    
    let converter = RethToErigonConverter;
    let options = ConversionOptions::default();
    
    let rt = tokio::runtime::Runtime::new().unwrap();
    let result = rt.block_on(converter.convert(&source_path, &target_path, &options));
    assert!(result.is_ok());
    
    let metadata = result.unwrap();
    assert_eq!(metadata.version, 1);
    assert_eq!(metadata.source_type, DatabaseType::Reth);
    assert_eq!(metadata.target_type, DatabaseType::ErigonMdbx);
}

#[test]
fn test_erigon_to_reth_conversion() {
    let temp_dir = TempDir::new().unwrap();
    let source_path = temp_dir.path().join("source");
    let target_path = temp_dir.path().join("target");
    
    // Create dummy source file
    std::fs::write(&source_path, b"dummy source data").unwrap();
    
    let converter = ErigonToRethConverter;
    let options = ConversionOptions::default();
    
    let rt = tokio::runtime::Runtime::new().unwrap();
    let result = rt.block_on(converter.convert(&source_path, &target_path, &options));
    assert!(result.is_ok());
    
    let metadata = result.unwrap();
    assert_eq!(metadata.version, 1);
    assert_eq!(metadata.source_type, DatabaseType::ErigonMdbx);
    assert_eq!(metadata.target_type, DatabaseType::Reth);
}

#[test]
fn test_snapshot_validation() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test_snapshot");
    
    // Create test file
    std::fs::write(&file_path, b"test snapshot data").unwrap();
    
    let validator = SnapValidator;
    let rt = tokio::runtime::Runtime::new().unwrap();
    let result = rt.block_on(validator.validate_file(&file_path));
    assert!(result.is_ok());
    assert!(result.unwrap());
}

#[test]
fn test_metadata_validation() {
    let validator = SnapValidator;
    
    // Valid metadata
    let valid_metadata = SnapMetadata {
        version: 1,
        timestamp: 1234567890,
        source_type: DatabaseType::Reth,
        target_type: DatabaseType::ErigonMdbx,
        checksum: "test_checksum".to_string(),
        record_count: 100,
        total_size: 1024,
    };
    
    let result = validator.validate_metadata(&valid_metadata);
    assert!(result.is_ok());
    
    // Invalid metadata - wrong version
    let invalid_metadata = SnapMetadata {
        version: 2,
        timestamp: 1234567890,
        source_type: DatabaseType::Reth,
        target_type: DatabaseType::ErigonMdbx,
        checksum: "test_checksum".to_string(),
        record_count: 100,
        total_size: 1024,
    };
    
    let result = validator.validate_metadata(&invalid_metadata);
    assert!(result.is_err());
}

#[test]
fn test_record_validation() {
    let validator = SnapValidator;
    
    // Valid record
    let valid_record = SnapRecord {
        key: b"test_key".to_vec(),
        value: b"test_value".to_vec(),
        record_type: RecordType::BlockHeader,
        block_number: Some(alloy_primitives::U256::from(123)),
    };
    
    let result = validator.validate_record(&valid_record);
    assert!(result.is_ok());
    
    // Invalid record - empty key
    let invalid_record = SnapRecord {
        key: vec![],
        value: b"test_value".to_vec(),
        record_type: RecordType::BlockHeader,
        block_number: Some(alloy_primitives::U256::from(123)),
    };
    
    let result = validator.validate_record(&invalid_record);
    assert!(result.is_err());
}

#[test]
fn test_conversion_options_default() {
    let options = ConversionOptions::default();
    assert!(options.compress);
    assert_eq!(options.compression_level, 6);
    assert_eq!(options.batch_size, 1000);
    assert!(options.validate_checksums);
    assert_eq!(options.progress_interval, 1000);
}
