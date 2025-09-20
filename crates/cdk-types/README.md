# CDK Types

Core types for CDK (Celestia Data Availability) integration with Reth.

## Overview

This crate defines the fundamental data structures used throughout the CDK integration:

- **Batch**: Represents a batch of blocks with L1 origin information
- **Epoch**: Represents a time period with start/end block boundaries  
- **FinalityTag**: Represents finality status from L1 contracts

## Usage

```rust
use cdk_types::{Batch, BatchId, Epoch, EpochId, FinalityTag, FinalityStatus};
use alloy_primitives::{FixedBytes, U256};

// Create a batch
let batch_id = BatchId::new(U256::from(1), FixedBytes::from([1u8; 32]));
let batch = Batch::new(
    batch_id,
    U256::from(100), // L1 origin block
    FixedBytes::from([2u8; 32]), // L1 origin hash
    vec![], // blocks
    proof_meta, // proof metadata
    1234567890, // timestamp
);

// Create an epoch
let epoch_id = EpochId::new(U256::from(1), FixedBytes::from([1u8; 32]));
let epoch = Epoch::new(
    epoch_id,
    U256::from(100), // start block
    U256::from(200), // end block
    U256::from(10),  // start batch
    U256::from(20),  // end batch
    1234567890,      // start timestamp
    1234567890 + 3600, // end timestamp
);

// Create a finality tag
let finality_tag = FinalityTag::new(
    U256::from(1), // batch ID
    U256::from(100), // L1 block
    FixedBytes::from([1u8; 32]), // L1 block hash
    FinalityStatus::Finalized,
    1234567890, // timestamp
    Some(FixedBytes::from([2u8; 32])), // tx hash
);
```

## Features

- **Alloy Integration**: Uses `alloy-primitives` for consistent Ethereum primitive handling
- **Serialization**: Supports RLP encoding/decoding and JSON serialization
- **Type Safety**: Strong typing for all CDK-specific concepts
- **Error Handling**: Comprehensive error types for CDK operations

## Metrics

This crate exposes the following metrics (when used with `cdk-observe`):

- `cdk_batch_count`: Number of batches processed
- `cdk_epoch_count`: Number of epochs processed  
- `cdk_finality_tags`: Number of finality tags processed
