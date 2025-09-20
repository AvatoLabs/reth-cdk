# CDK RPC Extensions

Extended RPC API for CDK (Celestia Data Availability) integration with Reth.

## Overview

This crate provides extended RPC methods for querying CDK-specific data structures like batches, epochs, and finality information. It integrates with the existing CDK modules to expose batch/epoch metrics and queries through JSON-RPC.

## Features

- **Extended RPC Methods**: CDK-specific query methods
- **Batch Queries**: Get batch information by number
- **Epoch Queries**: Get epoch information by block number
- **Finality Status**: Query finalized batch information
- **Metrics**: CDK-specific metrics and statistics
- **JSON-RPC Integration**: Seamless integration with jsonrpsee

## Usage

### Basic RPC Server Setup

```rust
use cdk_rpc_ext::{CdkRpcServer, CdkRpcConfig};
use jsonrpsee::server::ServerBuilder;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = CdkRpcConfig::default();
    let cdk_server = CdkRpcServer::new(config).await?;
    
    let listener = TcpListener::bind("127.0.0.1:8545").await?;
    let server = ServerBuilder::new()
        .build(listener)
        .await?;
    
    server.start(cdk_server.into_rpc()).await?;
    Ok(())
}
```

### Available RPC Methods

#### cdk_getBatchByNumber
Get batch information by batch number.

```json
{
  "jsonrpc": "2.0",
  "method": "cdk_getBatchByNumber",
  "params": ["0x1"],
  "id": 1
}
```

#### cdk_getEpochByBlock
Get epoch information by block number.

```json
{
  "jsonrpc": "2.0",
  "method": "cdk_getEpochByBlock", 
  "params": ["0x64"],
  "id": 1
}
```

#### cdk_finalizedBatch
Get the latest finalized batch information.

```json
{
  "jsonrpc": "2.0",
  "method": "cdk_finalizedBatch",
  "params": [],
  "id": 1
}
```

#### cdk_metrics
Get CDK-specific metrics and statistics.

```json
{
  "jsonrpc": "2.0",
  "method": "cdk_metrics",
  "params": [],
  "id": 1
}
```

## Configuration

The RPC server can be configured through `CdkRpcConfig`:

```rust
use cdk_rpc_ext::CdkRpcConfig;

let config = CdkRpcConfig {
    enable_batch_queries: true,
    enable_epoch_queries: true,
    enable_finality_queries: true,
    enable_metrics: true,
    max_batch_history: 1000,
    max_epoch_history: 100,
};
```

## Error Handling

All RPC methods return detailed error information:

```rust
use cdk_rpc_ext::CdkRpcError;

match result {
    Ok(data) => println!("Success: {:?}", data),
    Err(CdkRpcError::BatchNotFound(batch_id)) => {
        println!("Batch {} not found", batch_id);
    }
    Err(CdkRpcError::EpochNotFound(block_number)) => {
        println!("Epoch for block {} not found", block_number);
    }
    Err(e) => println!("Other error: {}", e),
}
```

## Integration with Reth

This crate is designed to integrate seamlessly with Reth's existing RPC infrastructure:

1. **Dependency Injection**: CDK components are injected through the server configuration
2. **Async Support**: All methods are async and non-blocking
3. **Error Propagation**: Errors are properly propagated and serialized
4. **Metrics Integration**: Built-in support for Prometheus metrics

## Testing

Run the test suite:

```bash
cargo test --package cdk-rpc-ext
```

The test suite includes:
- Unit tests for individual RPC methods
- Integration tests with mock data sources
- Error handling tests
- Performance benchmarks

## Metrics

This crate exposes the following metrics (when used with `cdk-observe`):

- `cdk_rpc_requests_total`: Total number of RPC requests
- `cdk_rpc_request_duration_seconds`: RPC request duration histogram
- `cdk_rpc_errors_total`: Total number of RPC errors by type
- `cdk_batch_queries_total`: Number of batch queries
- `cdk_epoch_queries_total`: Number of epoch queries
- `cdk_finality_queries_total`: Number of finality queries
