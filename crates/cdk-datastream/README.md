# CDK Datastream

Data stream source for CDK batch ingestion with checkpoint support.

## Overview

This crate provides the `BatchSource` trait and implementations for consuming batches from various data sources with checkpoint support for resumable ingestion.

## Features

- **BatchSource Trait**: Core abstraction for batch data sources
- **Checkpoint Support**: Resumable ingestion with checkpoint management
- **HTTP Source**: HTTP-based batch source implementation
- **Memory Storage**: In-memory checkpoint storage for testing
- **Error Handling**: Comprehensive error types for datastream operations

## Usage

### Basic HTTP Source

```rust
use cdk_datastream::{HttpBatchSource, HttpBatchSourceConfig};
use url::Url;

// Create HTTP batch source
let url = Url::parse("http://localhost:8080").unwrap();
let config = HttpBatchSourceConfig {
    base_url: url,
    ..Default::default()
};
let mut source = HttpBatchSource::new(config);

// Check health
source.health_check().await?;

// Get metadata
let metadata = source.metadata().await?;
println!("Source: {} v{}", metadata.name, metadata.version);

// Fetch batches
while let Some(batch) = source.next().await? {
    println!("Received batch {} with {} blocks", 
             batch.id.number, batch.block_count());
    
    // Process batch...
}
```

### Checkpoint Management

```rust
use cdk_datastream::{Checkpoint, MemoryCheckpointStorage, CheckpointStorage};

let storage = MemoryCheckpointStorage::default();

// Save checkpoint
let checkpoint = Checkpoint::new(
    U256::from(100),
    FixedBytes::from([1u8; 32]),
    U256::from(1000),
    1234567890,
);
storage.save_checkpoint(checkpoint).await?;

// Load checkpoint
let loaded = storage.load_checkpoint().await?;
if let Some(checkpoint) = loaded {
    println!("Resuming from batch {}", checkpoint.last_batch_id);
}
```

### Custom Batch Source

```rust
use cdk_datastream::{BatchSource, SourceMetadata, DatastreamError};
use async_trait::async_trait;

#[derive(Debug)]
struct CustomBatchSource {
    // Your implementation
}

#[async_trait]
impl BatchSource for CustomBatchSource {
    async fn next(&mut self) -> Result<Option<Batch>, DatastreamError> {
        // Implement batch fetching logic
        Ok(None)
    }

    async fn checkpoint(&self) -> Result<Checkpoint, DatastreamError> {
        // Return current checkpoint
        Ok(Checkpoint::new(/* ... */))
    }

    async fn set_checkpoint(&mut self, checkpoint: Checkpoint) -> Result<(), DatastreamError> {
        // Set checkpoint for resumable ingestion
        Ok(())
    }

    async fn health_check(&self) -> Result<(), DatastreamError> {
        // Check if source is healthy
        Ok(())
    }

    async fn metadata(&self) -> Result<SourceMetadata, DatastreamError> {
        // Return source metadata
        Ok(SourceMetadata::new(
            "Custom Source".to_string(),
            "1.0".to_string(),
            "custom://source".to_string(),
            true,
        ))
    }
}
```

## API Reference

### BatchSource Trait

The core trait for batch data sources:

- `next()`: Get the next batch from the source
- `checkpoint()`: Get current checkpoint for resumable ingestion
- `set_checkpoint()`: Set checkpoint to resume from
- `health_check()`: Check if source is healthy
- `metadata()`: Get source metadata

### Checkpoint

Represents the state of batch ingestion:

- `last_batch_id`: Last successfully processed batch ID
- `last_batch_hash`: Last successfully processed batch hash
- `last_l1_block`: L1 block number where last batch was submitted
- `timestamp`: When checkpoint was created
- `metadata`: Additional metadata

### CheckpointStorage Trait

Trait for persisting checkpoints:

- `save_checkpoint()`: Save a checkpoint
- `load_checkpoint()`: Load the latest checkpoint
- `delete_checkpoint()`: Delete a checkpoint

## Error Handling

The crate provides comprehensive error types:

- `NetworkError`: Network-related errors
- `HttpError`: HTTP-specific errors
- `SerializationError`: JSON serialization errors
- `CheckpointError`: Checkpoint-related errors
- `SourceUnavailable`: Source availability errors
- `InvalidBatchData`: Invalid batch data errors
- `ConfigError`: Configuration errors
- `TimeoutError`: Timeout errors
- `InternalError`: Internal errors

## Metrics

This crate exposes the following metrics (when used with `cdk-observe`):

- `cdk_datastream_batches_fetched`: Number of batches fetched
- `cdk_datastream_checkpoints_saved`: Number of checkpoints saved
- `cdk_datastream_errors_total`: Total number of errors
- `cdk_datastream_source_health`: Source health status
