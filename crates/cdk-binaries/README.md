# CDK Binaries

Command line tools for Reth CDK operations, providing data ingestion and finality monitoring capabilities.

## Overview

The `cdk-binaries` crate provides two main command line tools:

- **`reth-cdk ingest`**: Ingest batches from data source into Reth
- **`reth-cdk finality`**: Monitor L1 finality and trigger rollbacks

## Installation

```bash
cargo build --release --package cdk-binaries
```

## Usage

### Ingest Command

Ingest batches from a data source into Reth:

```bash
reth-cdk ingest \
  --datastream http://localhost:8080/batches \
  --from-checkpoint auto \
  --reth-rpc http://localhost:8545 \
  --max-batches 100 \
  --enable-metrics
```

#### Options

- `--datastream <URL>`: Data source URL (default: `http://localhost:8080/batches`)
- `--from-checkpoint <checkpoint>`: Starting checkpoint - `auto`, `latest`, or specific checkpoint (default: `auto`)
- `--reth-rpc <URL>`: Reth RPC URL (default: `http://localhost:8545`)
- `--max-batches <count>`: Maximum number of batches to process, 0 = unlimited (default: `0`)
- `--enable-metrics`: Enable metrics collection (default: `true`)

### Finality Command

Monitor L1 finality and trigger rollbacks:

```bash
reth-cdk finality \
  --l1-rpc http://localhost:8545 \
  --bridge 0x1234567890123456789012345678901234567890 \
  --reth-rpc http://localhost:8545 \
  --poll-interval 30 \
  --enable-metrics
```

#### Options

- `--l1-rpc <URL>`: L1 RPC URL (default: `http://localhost:8545`)
- `--bridge <address>`: Bridge contract address (required)
- `--reth-rpc <URL>`: Reth RPC URL (default: `http://localhost:8545`)
- `--poll-interval <seconds>`: Polling interval in seconds (default: `30`)
- `--enable-metrics`: Enable metrics collection (default: `true`)

## Features

### Ingest Tool

- **Batch Processing**: Fetches and processes batches from data source
- **Block Assembly**: Converts batches to standard block inputs for Reth
- **Mapping Storage**: Maintains batch-to-block and epoch mappings
- **Checkpoint Support**: Supports resumable ingestion from checkpoints
- **Metrics**: Real-time metrics for batch processing performance
- **Error Handling**: Robust error handling with retry logic

### Finality Tool

- **L1 Monitoring**: Monitors L1 finality status via bridge contract
- **Rollback Detection**: Automatically detects and handles rollbacks
- **Reth Integration**: Triggers rollbacks in Reth when needed
- **Metrics**: Tracks finality status and rollback events
- **Configurable Polling**: Adjustable polling interval for L1 checks

## Architecture

The binaries integrate with the following CDK modules:

- **cdk-datastream**: Data source abstraction and checkpoint management
- **cdk-ingest**: Block assembly and mapping storage
- **cdk-finality**: L1 finality monitoring and rollback management
- **cdk-observe**: Unified logging and metrics collection

## Error Handling

Both tools implement comprehensive error handling:

- **Retry Logic**: Exponential backoff for transient failures
- **Graceful Degradation**: Continue operation despite individual failures
- **Detailed Logging**: Structured logging for debugging and monitoring
- **Metrics**: Error counters and performance metrics

## Configuration

The tools use the following configuration sources (in order of precedence):

1. Command line arguments
2. Environment variables
3. Default values

### Environment Variables

- `CDK_DATASTREAM_URL`: Default data source URL
- `CDK_RETH_RPC_URL`: Default Reth RPC URL
- `CDK_L1_RPC_URL`: Default L1 RPC URL
- `CDK_BRIDGE_ADDRESS`: Default bridge contract address
- `CDK_LOG_LEVEL`: Log level (trace, debug, info, warn, error)

## Examples

### Basic Ingest

```bash
# Ingest from default data source
reth-cdk ingest

# Ingest with custom settings
reth-cdk ingest \
  --datastream https://api.example.com/batches \
  --max-batches 50
```

### Basic Finality Monitoring

```bash
# Monitor finality with bridge contract
reth-cdk finality \
  --bridge 0x1234567890123456789012345678901234567890

# Custom polling interval
reth-cdk finality \
  --bridge 0x1234567890123456789012345678901234567890 \
  --poll-interval 60
```

### Production Setup

```bash
# Ingest with metrics and custom RPC
reth-cdk ingest \
  --datastream https://production-api.com/batches \
  --reth-rpc https://reth.example.com:8545 \
  --enable-metrics

# Finality monitoring with custom L1
reth-cdk finality \
  --l1-rpc https://eth-mainnet.alchemyapi.io/v2/YOUR_KEY \
  --bridge 0x1234567890123456789012345678901234567890 \
  --reth-rpc https://reth.example.com:8545
```

## Development

### Building

```bash
cargo build --package cdk-binaries
```

### Testing

```bash
cargo test --package cdk-binaries
```

### Running Tests

```bash
# Unit tests
cargo test --package cdk-binaries --lib

# Integration tests
cargo test --package cdk-binaries --test integration
```

## Dependencies

- **clap**: Command line argument parsing
- **tokio**: Async runtime
- **anyhow**: Error handling
- **reqwest**: HTTP client
- **chrono**: Time handling
- **cdk-***: CDK modules for core functionality

## License

This project is part of the Reth CDK adaptation layer and follows the same license as the main Reth project.
