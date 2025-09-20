# CDK Snapshot and Index Interoperability Tools

This module provides tools for snapshot and index interoperability between Reth and Erigon MDBX databases, enabling data migration and validation.

## Features

- **Database Conversion**: Convert between Reth and Erigon MDBX database formats
- **Snapshot Validation**: Validate snapshot integrity and metadata
- **Data Migration**: Migrate data between different database formats
- **Checksum Verification**: Ensure data integrity during conversion

## Usage

### Converting Reth to Erigon

```rust
use cdk_snap::{RethToErigonConverter, ConversionOptions};
use std::path::Path;

let converter = RethToErigonConverter;
let options = ConversionOptions::default();
let metadata = converter.convert(
    Path::new("reth_db"),
    Path::new("erigon_db"),
    &options
).await?;
```

### Converting Erigon to Reth

```rust
use cdk_snap::{ErigonToRethConverter, ConversionOptions};
use std::path::Path;

let converter = ErigonToRethConverter;
let options = ConversionOptions::default();
let metadata = converter.convert(
    Path::new("erigon_db"),
    Path::new("reth_db"),
    &options
).await?;
```

### Validating Snapshots

```rust
use cdk_snap::{SnapValidator, SnapMetadata};

let validator = SnapValidator;

// Validate metadata
validator.validate_metadata(&metadata)?;

// Validate file integrity
let is_valid = validator.validate_file(Path::new("snapshot.bin")).await?;
```

## Configuration

### Conversion Options

```rust
use cdk_snap::ConversionOptions;

let options = ConversionOptions {
    compress: true,              // Enable compression
    compression_level: 6,        // Compression level (1-22)
    batch_size: 1000,           // Batch size for processing
    validate_checksums: true,    // Validate checksums
    progress_interval: 1000,      // Progress callback interval
};
```

## Error Handling

The module provides comprehensive error handling for various failure scenarios:

- `SnapError::Io`: I/O errors during file operations
- `SnapError::Serialization`: JSON serialization/deserialization errors
- `SnapError::InvalidFormat`: Invalid snapshot format
- `SnapError::VersionMismatch`: Version compatibility issues
- `SnapError::ChecksumMismatch`: Data integrity failures
- `SnapError::Database`: Database-specific errors
- `SnapError::Conversion`: Conversion process errors
- `SnapError::Validation`: Data validation failures

## Testing

Run the tests with:

```bash
cargo test --package cdk-snap
```

## Architecture

The module is organized into several components:

- **converter.rs**: Database conversion logic
- **validator.rs**: Snapshot validation and integrity checks
- **types.rs**: Core data structures and types
- **error.rs**: Error definitions and handling

## Future Enhancements

- [ ] Parallel processing for large datasets
- [ ] Incremental conversion support
- [ ] Compression algorithm selection
- [ ] Progress reporting and cancellation
- [ ] Memory-mapped file support for large files
- [ ] Custom validation rules
- [ ] Snapshot format versioning
- [ ] Cross-platform compatibility improvements
