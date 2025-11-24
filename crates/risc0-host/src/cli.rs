//! Command-line interface definitions for risc0-host
//!
//! Defines all CLI commands, subcommands, and arguments using clap.

use clap::{Args, Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    name = "risc0-host",
    author,
    version,
    about = "RISC0 zkVM host program for Sigstore attestation verification",
    long_about = "Generate zero-knowledge proofs of Sigstore attestation bundle verification using RISC0 zkVM"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Display the RISC0 program ImageID
    #[command(name = "image-id")]
    ImageId,

    /// Generate a proof of attestation verification
    Prove(ProveArgs),
}

#[derive(Args, Debug)]
pub struct ProveArgs {
    /// Path to the Sigstore attestation bundle JSON file
    #[arg(long = "bundle", value_name = "PATH", required = true)]
    pub bundle_path: PathBuf,

    /// Path to the trusted root JSONL file
    #[arg(long = "trust-roots", value_name = "PATH", required = true)]
    pub trust_roots_path: PathBuf,

    /// Path to write the proof artifact JSON file
    #[arg(long = "output", value_name = "PATH")]
    pub output_path: Option<PathBuf>,

    /// Proving strategy
    #[command(subcommand)]
    pub strategy: ProveStrategy,
}

#[derive(Subcommand, Debug)]
pub enum ProveStrategy {
    /// Prove locally (not yet supported)
    Local,

    /// Prove using Boundless network
    Boundless(BoundlessArgs),
}

#[derive(Args, Debug, Clone)]
pub struct BoundlessArgs {
    /// Boundless RPC URL
    #[arg(
        long = "boundless-rpc-url",
        env = "BOUNDLESS_RPC_URL",
        value_name = "URL"
    )]
    pub rpc_url: String,

    /// Boundless private key (hex-encoded)
    #[arg(
        long = "boundless-private-key",
        env = "BOUNDLESS_PRIVATE_KEY",
        value_name = "WALLET_KEY",
        hide_env_values = true
    )]
    pub private_key: String,

    /// Program URL (optional, uses embedded ELF if not provided)
    #[arg(
        long = "program-url",
        env = "BOUNDLESS_PROGRAM_URL",
        value_name = "URL"
    )]
    pub program_url: Option<String>,

    /// Proof type
    #[arg(
        long = "proof-type",
        value_enum,
        default_value = "groth16",
        value_name = "TYPE"
    )]
    pub proof_type: BoundlessProofType,

    /// Minimum price for proof generation
    #[arg(long = "min-price", value_name = "WEI")]
    pub min_price: Option<u128>,

    /// Maximum price for proof generation
    #[arg(long = "max-price", value_name = "WEI")]
    pub max_price: Option<u128>,

    /// Timeout in seconds
    #[arg(long = "timeout", value_name = "SECONDS")]
    pub timeout: Option<u32>,

    /// Ramp-up period in seconds
    #[arg(long = "ramp-up-period", value_name = "SECONDS")]
    pub ramp_up_period: Option<u32>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum BoundlessProofType {
    /// Groth16 proof
    #[value(name = "groth16")]
    Groth16,

    /// Merkle proof
    #[value(name = "merkle")]
    Merkle,
}
