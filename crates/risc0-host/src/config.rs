//! Configuration types for RISC0 proving
//!
//! Defines configuration structures for different proving strategies.

use crate::cli::{BoundlessArgs, BoundlessProofType, ProveArgs, ProveStrategy};

/// Proving strategy enum
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProvingStrategy {
    /// Local proving (not yet supported)
    Local,
    /// Boundless network proving
    Boundless,
}

/// RISC0 prover configuration
#[derive(Debug, Clone)]
pub struct Risc0Config {
    pub proving_strategy: ProvingStrategy,
    pub boundless: Option<BoundlessConfig>,
}

/// Boundless network configuration
#[derive(Debug, Clone)]
pub struct BoundlessConfig {
    pub rpc_url: String,
    pub private_key: String,
    pub program_url: Option<String>,
    pub proof_type: BoundlessProofType,
    pub min_price: Option<u128>,
    pub max_price: Option<u128>,
    pub timeout: Option<u32>,
    pub ramp_up_period: Option<u32>,
}

impl Risc0Config {
    /// Build a Risc0Config from CLI arguments
    ///
    /// # Arguments
    ///
    /// * `args` - The prove command arguments
    ///
    /// # Returns
    ///
    /// Returns a Risc0Config with the appropriate strategy and parameters.
    pub fn from_cli_args(args: &ProveArgs) -> Self {
        match &args.strategy {
            ProveStrategy::Local => Risc0Config {
                proving_strategy: ProvingStrategy::Local,
                boundless: None,
            },
            ProveStrategy::Boundless(boundless_args) => Risc0Config {
                proving_strategy: ProvingStrategy::Boundless,
                boundless: Some(BoundlessConfig::from_cli_args(boundless_args)),
            },
        }
    }
}

impl BoundlessConfig {
    /// Build a BoundlessConfig from CLI arguments
    ///
    /// # Arguments
    ///
    /// * `args` - The Boundless strategy arguments
    ///
    /// # Returns
    ///
    /// Returns a BoundlessConfig with all parameters from CLI args.
    pub fn from_cli_args(args: &BoundlessArgs) -> Self {
        BoundlessConfig {
            rpc_url: args.rpc_url.clone(),
            private_key: args.private_key.clone(),
            program_url: args.program_url.clone(),
            proof_type: args.proof_type,
            min_price: args.min_price,
            max_price: args.max_price,
            timeout: args.timeout,
            ramp_up_period: args.ramp_up_period,
        }
    }
}
