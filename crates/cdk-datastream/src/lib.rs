//! Data stream source for CDK batch ingestion
//!
//! This crate provides the `BatchSource` trait and implementations for
//! consuming batches from various data sources with checkpoint support
//! for resumable ingestion.

pub mod checkpoint;
pub mod error;
pub mod http_source;
pub mod source;
pub mod websocket_source;
pub mod grpc_source;
pub mod filesystem_source;

pub use checkpoint::*;
pub use error::*;
pub use http_source::*;
pub use source::*;
pub use websocket_source::*;
pub use grpc_source::*;
pub use filesystem_source::*;
