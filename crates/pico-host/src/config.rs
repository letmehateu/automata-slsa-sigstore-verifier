//! Configuration types for Pico proving
//!
//! Defines configuration structures for Pico zkVM prover.

use crate::cli::ProveArgs;
use std::path::PathBuf;

/// Pico prover configuration
#[derive(Debug, Clone)]
pub struct PicoConfig {
    /// Path to the directory containing EVM proof artifacts (vm_pk, vm_vk, constraints.json)
    pub artifacts_path: PathBuf,

    /// Field type for proving backend (e.g., "kb" for KoalaBear, "bb" for BabyBear)
    /// Default: "kb" (KoalaBear)
    pub field_type: String,
}

impl Default for PicoConfig {
    fn default() -> Self {
        Self {
            artifacts_path: PathBuf::from("./artifacts"),
            field_type: "kb".to_string(), // KoalaBear is the default
        }
    }
}

impl PicoConfig {
    /// Create a new PicoConfig with custom artifacts path
    pub fn new(artifacts_path: PathBuf) -> Self {
        Self {
            artifacts_path,
            field_type: "kb".to_string(),
        }
    }

    /// Set the field type for the proving backend
    pub fn with_field_type(mut self, field_type: String) -> Self {
        self.field_type = field_type;
        self
    }

    /// Build a PicoConfig from CLI arguments
    pub fn from_cli_args(args: &ProveArgs) -> Self {
        PicoConfig {
            artifacts_path: args.artifacts_path.clone(),
            field_type: args.field_type.as_str().to_string(),
        }
    }
}
