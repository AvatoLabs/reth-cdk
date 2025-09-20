//! Block assembler for CDK batch ingestion
//!
//! This crate provides the `BlockAssembler` trait and implementations for
//! converting batches into standard blocks that can be fed to Reth's import API.
//! It also maintains mappings between blocks and batches/epochs.

pub mod assembler;
pub mod error;
pub mod mapping;
pub mod validator;

pub use assembler::*;
pub use error::*;
pub use mapping::*;
pub use validator::*;
