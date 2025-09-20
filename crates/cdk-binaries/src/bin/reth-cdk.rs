//! CDK binaries - Command line tools for Reth CDK operations

use clap::{Parser, Subcommand};
use anyhow::Result;

use cdk_binaries::{IngestCommand, FinalityCommand};

/// Reth CDK command line tools
#[derive(Parser)]
#[command(name = "reth-cdk")]
#[command(about = "Reth CDK command line tools for data ingestion and finality monitoring")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Ingest batches from data source into Reth
    Ingest(IngestCommand),
    /// Monitor L1 finality and trigger rollbacks
    Finality(FinalityCommand),
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize observability
    let config = cdk_observe::ObservabilityConfig::default();
    let tracing_config = cdk_observe::TracingConfig::new(&config);
    tracing_config.init().map_err(|e| anyhow::anyhow!("Failed to initialize tracing: {}", e))?;

    let cli = Cli::parse();

    match cli.command {
        Commands::Ingest(cmd) => cmd.run().await,
        Commands::Finality(cmd) => cmd.run().await,
    }
}
