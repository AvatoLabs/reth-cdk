# CDK Ingest

Block assembler for CDK batch ingestion and block conversion.

## Overview

This crate provides the `BlockAssembler` trait and implementations for converting batches into standard blocks that can be fed to Reth's import API. It also maintains mappings between blocks and batches/epochs.

## Features

- **BlockAssembler Trait**: Core abstraction for block assembly
- **Block Inputs**: Standardized block input data structures
- **Mapping Management**: Block/batch/epoch relationship tracking
- **Validation**: Comprehensive batch and block validation
- **Memory Storage**: In-memory mapping storage for testing
- **Error Handling**: Detailed error types for ingestion operations

## Usage

### Basic Block Assembly

```rust
use cdk_ingest::{BlockAssembler, BlockInputs, IngestError};
use cdk_types::Batch;
use async_trait::async_trait;

#[derive(Debug)]
struct MyBlockAssembler {
    // Your implementation
}

#[async_trait]
impl BlockAssembler for MyBlockAssembler {
    async fn assemble(&mut self, batch: &Batch) -> Result<Vec<BlockInputs>, IngestError> {
        let mut blocks = Vec::new();
        
        for (index, block_in_batch) in batch.blocks.iter().enumerate() {
            let block_input = BlockInputs {
                number: block_in_batch.number.to::<u64>(),
                hash: block_in_batch.hash,
                parent_hash: block_in_batch.parent_hash,
                state_root: block_in_batch.state_root,
                receipts_root: block_in_batch.receipt_root,
                transactions_root: block_in_batch.tx_root,
                timestamp: block_in_batch.timestamp,
                gas_limit: 30000000,
                gas_used: 15000000,
                base_fee_per_gas: Some(1000000000),
                extra_data: alloy_primitives::Bytes::new(),
                transactions: vec![],
            };
            blocks.push(block_input);
        }
        
        Ok(blocks)
    }

    async fn validate_batch(&self, batch: &Batch) -> Result<(), IngestError> {
        // Implement validation logic
        Ok(())
    }

    // ... implement other trait methods
}
```

### Mapping Management

```rust
use cdk_ingest::{MappingManager, MemoryMappingStorage, BlockMapping};

// Create mapping manager with memory storage
let storage = Box::new(MemoryMappingStorage::default());
let mut manager = MappingManager::new(storage);

// Create block mapping
let block_mapping = manager.create_block_mapping(
    100,                                    // block number
    FixedBytes::from([1u8; 32]),          // block hash
    1,                                      // batch ID
    0,                                      // batch index
    1,                                      // epoch ID
);

// Save mappings
manager.save_mappings(vec![block_mapping]).await?;

// Get statistics
let stats = manager.get_stats();
println!("Total blocks: {}", stats.total_blocks);
```

### Batch Validation

```rust
use cdk_ingest::{BatchValidator, IngestError};

let validator = BatchValidator::new(
    1000,           // max blocks per batch
    10 * 1024 * 1024, // max batch size (10MB)
    true,           // strict mode
);

// Validate batch
validator.validate_batch(&batch).await?;

// Validate block inputs
validator.validate_block_inputs(&block_inputs).await?;
```

## API Reference

### BlockAssembler Trait

The core trait for block assembly:

- `assemble()`: Convert batch to block inputs
- `validate_batch()`: Validate batch before assembly
- `get_block_mapping()`: Get block mapping by number
- `get_batch_mapping()`: Get batch mapping by ID
- `get_epoch_mapping()`: Get epoch mapping by ID
- `update_mappings()`: Update mappings after import
- `get_stats()`: Get assembly statistics

### BlockInputs

Standardized block input data:

- `number`: Block number
- `hash`: Block hash
- `parent_hash`: Parent block hash
- `state_root`: State root
- `receipts_root`: Receipts root
- `transactions_root`: Transactions root
- `timestamp`: Block timestamp
- `gas_limit`: Gas limit
- `gas_used`: Gas used
- `base_fee_per_gas`: Base fee (EIP-1559)
- `extra_data`: Extra data
- `transactions`: Transaction inputs

### Mapping Types

#### BlockMapping
- `block_number`: Block number
- `block_hash`: Block hash
- `batch_id`: Batch ID this block belongs to
- `batch_index`: Index within the batch
- `epoch_id`: Epoch ID this block belongs to
- `timestamp`: When mapping was created

#### BatchMapping
- `batch_id`: Batch ID
- `batch_hash`: Batch hash
- `start_block`: Start block number
- `end_block`: End block number
- `block_count`: Number of blocks
- `epoch_id`: Epoch ID this batch belongs to
- `timestamp`: When mapping was created

#### EpochMapping
- `epoch_id`: Epoch ID
- `epoch_hash`: Epoch hash
- `start_block`: Start block number
- `end_block`: End block number
- `block_count`: Number of blocks
- `batch_count`: Number of batches
- `timestamp`: When mapping was created

### Validation

#### BatchValidator

Validates batches and blocks:

- `validate_batch()`: Validate batch data
- `validate_block_inputs()`: Validate block inputs
- `validate_transaction_input()`: Validate transaction inputs

Configuration:
- `max_blocks_per_batch`: Maximum blocks per batch
- `max_batch_size_bytes`: Maximum batch size
- `strict_mode`: Enable strict validation

## Error Handling

The crate provides comprehensive error types:

- `AssemblyError`: Block assembly errors
- `ValidationError`: Validation errors
- `MappingError`: Mapping-related errors
- `BatchProcessingError`: Batch processing errors
- `BlockConversionError`: Block conversion errors
- `TransactionProcessingError`: Transaction processing errors
- `StateRootMismatch`: State root validation errors
- `InvalidBatchData`: Invalid batch data
- `InvalidBlockData`: Invalid block data
- `StorageError`: Storage-related errors
- `ConfigError`: Configuration errors
- `InternalError`: Internal errors

## Metrics

This crate exposes the following metrics (when used with `cdk-observe`):

- `cdk_ingest_blocks_assembled`: Number of blocks assembled
- `cdk_ingest_batches_processed`: Number of batches processed
- `cdk_ingest_validation_errors`: Number of validation errors
- `cdk_ingest_mapping_operations`: Number of mapping operations
- `cdk_ingest_assembly_duration`: Time spent on assembly
