# CDK Finality

Finality oracle for CDK L1 finality checking and rollback management.

## Overview

This crate provides the `FinalityOracle` trait and implementations for checking L1 finality status and managing rollbacks. It reads from L1 contracts to determine when batches can be considered final.

## Features

- **FinalityOracle Trait**: Core abstraction for finality checking
- **L1 Client**: Ethereum mainnet client for contract interactions
- **Rollback Management**: Comprehensive rollback handling and tracking
- **Event Processing**: Finality event detection and processing
- **Health Monitoring**: Oracle health checks and monitoring
- **Error Handling**: Detailed error types for finality operations

## Usage

### Basic Finality Oracle

```rust
use cdk_finality::{FinalityOracle, FinalityOracleConfig, L1Client};
use alloy_primitives::Address;
use async_trait::async_trait;

#[derive(Debug)]
struct MyFinalityOracle {
    l1_client: L1Client,
    bridge_address: Address,
    // Your implementation
}

#[async_trait]
impl FinalityOracle for MyFinalityOracle {
    async fn poll(&mut self) -> FinalityResult<Vec<FinalityTag>> {
        // Implement finality polling logic
        Ok(vec![])
    }

    async fn get_finality_status(&self, batch_id: u64) -> FinalityResult<Option<FinalityStatus>> {
        // Check finality status for specific batch
        Ok(None)
    }

    async fn get_finalized_batches(&self) -> FinalityResult<Vec<FinalityTag>> {
        // Get all finalized batches
        Ok(vec![])
    }

    async fn get_rolled_back_batches(&self) -> FinalityResult<Vec<FinalityTag>> {
        // Get all rolled back batches
        Ok(vec![])
    }

    async fn health_check(&self) -> FinalityResult<()> {
        // Check oracle health
        Ok(())
    }

    async fn metadata(&self) -> FinalityResult<OracleMetadata> {
        // Return oracle metadata
        Ok(OracleMetadata::new(
            "My Oracle".to_string(),
            "1.0".to_string(),
            1,
            self.bridge_address,
        ))
    }

    fn set_polling_interval(&mut self, interval: Duration) {
        // Set polling interval
    }

    fn get_polling_interval(&self) -> Duration {
        Duration::from_secs(12)
    }
}
```

### L1 Client Usage

```rust
use cdk_finality::{L1Client, L1ClientConfig};

// Create L1 client
let config = L1ClientConfig {
    rpc_url: "https://mainnet.infura.io/v3/YOUR_KEY".to_string(),
    timeout: Duration::from_secs(30),
    max_retries: 3,
    retry_delay: Duration::from_secs(1),
    api_key: Some("your_api_key".to_string()),
};

let mut client = L1Client::new(config)?;

// Initialize client
client.initialize().await?;

// Get current block number
let block_number = client.get_current_block_number().await?;

// Get block data
let block = client.get_block_by_number(block_number).await?;

// Call contract method
let result = client.call_contract(
    Address::from([0x1234u8; 20]),
    &[0x12, 0x34, 0x56],
    Some(block_number),
).await?;

// Health check
client.health_check().await?;
```

### Rollback Management

```rust
use cdk_finality::{RollbackManager, RollbackConfig, FinalityUpdate, FinalityEventType};

// Create rollback manager
let config = RollbackConfig {
    required_confirmations: 12,
    max_rollback_depth: 1000,
    rollback_timeout: Duration::from_secs(3600),
    auto_execute: true,
    validate_rollbacks: true,
};

let mut manager = RollbackManager::new(config);

// Process finality update
let update = FinalityUpdate {
    tag: finality_tag,
    event_type: FinalityEventType::RolledBack,
    l1_block_number: 1000,
    tx_hash: Some(tx_hash),
    detected_at: timestamp,
};

let actions = manager.process_finality_update(update).await?;

// Check rollback history
let history = manager.get_rollback_history();
let is_rolled_back = manager.is_batch_rolled_back(batch_id);

// Clean up old records
manager.cleanup_old_records(Duration::from_secs(86400)); // 24 hours
```

## API Reference

### FinalityOracle Trait

The core trait for finality checking:

- `poll()`: Poll for new finality updates
- `get_finality_status()`: Get finality status for specific batch
- `get_finalized_batches()`: Get all finalized batches
- `get_rolled_back_batches()`: Get all rolled back batches
- `health_check()`: Check oracle health
- `metadata()`: Get oracle metadata
- `set_polling_interval()`: Set polling interval
- `get_polling_interval()`: Get current polling interval

### L1Client

Ethereum mainnet client:

- `new()`: Create new L1 client
- `from_rpc_url()`: Create from RPC URL string
- `initialize()`: Initialize client (get chain ID)
- `get_current_block_number()`: Get current L1 block number
- `get_block_by_number()`: Get block by number
- `call_contract()`: Call contract method
- `health_check()`: Perform health check
- `get_metadata()`: Get client metadata

### RollbackManager

Rollback management:

- `process_finality_update()`: Process finality update
- `get_rollback_history()`: Get rollback history
- `get_pending_rollbacks()`: Get pending rollbacks
- `is_batch_rolled_back()`: Check if batch was rolled back
- `get_rollback_record()`: Get rollback record for batch
- `cleanup_old_records()`: Clean up old records

### Configuration Types

#### FinalityOracleConfig
- `l1_rpc_url`: L1 RPC URL
- `bridge_address`: Bridge contract address
- `polling_interval`: Polling interval
- `max_retries`: Maximum number of retries
- `retry_delay`: Retry delay
- `confirmation_blocks`: Confirmation blocks required
- `strict_mode`: Enable strict mode

#### L1ClientConfig
- `rpc_url`: L1 RPC URL
- `timeout`: Request timeout
- `max_retries`: Maximum number of retries
- `retry_delay`: Retry delay
- `api_key`: API key for authentication

#### RollbackConfig
- `required_confirmations`: Required confirmations before executing rollback
- `max_rollback_depth`: Maximum rollback depth
- `rollback_timeout`: Rollback timeout
- `auto_execute`: Enable automatic rollback execution
- `validate_rollbacks`: Enable rollback validation

## Error Handling

The crate provides comprehensive error types:

- `L1RpcError`: L1 RPC errors
- `ContractCallError`: Contract call errors
- `NetworkError`: Network-related errors
- `SerializationError`: JSON serialization errors
- `OracleError`: Oracle-specific errors
- `ConfigError`: Configuration errors
- `TimeoutError`: Timeout errors
- `InvalidFinalityData`: Invalid finality data
- `BridgeContractError`: Bridge contract errors
- `RollbackError`: Rollback-related errors
- `HealthCheckError`: Health check failures
- `InternalError`: Internal errors

## Metrics

This crate exposes the following metrics (when used with `cdk-observe`):

- `cdk_finality_polls_total`: Number of finality polls
- `cdk_finality_updates_detected`: Number of finality updates detected
- `cdk_finality_rollbacks_executed`: Number of rollbacks executed
- `cdk_finality_l1_lag`: L1 lag in blocks
- `cdk_finality_oracle_health`: Oracle health status
- `cdk_finality_contract_calls`: Number of contract calls
