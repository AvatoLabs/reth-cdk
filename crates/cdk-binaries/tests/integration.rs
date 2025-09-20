//! Integration tests for CDK binaries

#[cfg(test)]
mod tests {
    use cdk_binaries::{IngestCommand, FinalityCommand, parse_checkpoint, validate_url, retry_delay, format_duration};
    use std::time::Duration;

    #[test]
    fn test_ingest_command_creation() {
        let cmd = IngestCommand {
            datastream: "http://localhost:8080/batches".to_string(),
            from_checkpoint: "auto".to_string(),
            reth_rpc: "http://localhost:8545".to_string(),
            max_batches: 10,
            enable_metrics: true,
        };
        
        assert_eq!(cmd.datastream, "http://localhost:8080/batches");
        assert_eq!(cmd.from_checkpoint, "auto");
        assert_eq!(cmd.reth_rpc, "http://localhost:8545");
        assert_eq!(cmd.max_batches, 10);
        assert!(cmd.enable_metrics);
    }

    #[test]
    fn test_finality_command_creation() {
        let cmd = FinalityCommand {
            l1_rpc: "http://localhost:8545".to_string(),
            bridge: "0x1234567890123456789012345678901234567890".to_string(),
            reth_rpc: "http://localhost:8545".to_string(),
            poll_interval: 30,
            enable_metrics: true,
        };
        
        assert_eq!(cmd.l1_rpc, "http://localhost:8545");
        assert_eq!(cmd.bridge, "0x1234567890123456789012345678901234567890");
        assert_eq!(cmd.reth_rpc, "http://localhost:8545");
        assert_eq!(cmd.poll_interval, 30);
        assert!(cmd.enable_metrics);
    }

    #[test]
    fn test_parse_checkpoint() {
        assert!(parse_checkpoint("auto").unwrap().is_none());
        assert!(parse_checkpoint("latest").unwrap().is_none());
    }

    #[test]
    fn test_validate_url() {
        assert!(validate_url("http://localhost:8080").is_ok());
        assert!(validate_url("https://example.com").is_ok());
        assert!(validate_url("invalid-url").is_err());
    }

    #[test]
    fn test_retry_delay() {
        let delay1 = retry_delay(0);
        let delay2 = retry_delay(1);
        let delay3 = retry_delay(10);
        
        assert!(delay2 > delay1);
        assert_eq!(delay3, Duration::from_secs(60)); // Max delay
    }

    #[test]
    fn test_format_duration() {
        assert_eq!(format_duration(Duration::from_secs(30)), "30s");
        assert_eq!(format_duration(Duration::from_secs(90)), "1m 30s");
        assert_eq!(format_duration(Duration::from_secs(3661)), "1h 1m 1s");
    }
}
