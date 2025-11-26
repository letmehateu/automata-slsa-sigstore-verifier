//! Command-line interface definitions for pico-host
//!
//! Defines all CLI commands, subcommands, and arguments using clap.

use clap::{Args, Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    name = "pico-host",
    author,
    version,
    about = "Pico zkVM host program for Sigstore attestation verification",
    long_about = "Generate zero-knowledge proofs of Sigstore attestation bundle verification using Pico zkVM"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Display the Pico program identifier (VK hash)
    #[command(name = "program-id")]
    ProgramId,

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

    /// Path to the Pico artifacts directory (vm_pk, vm_vk, constraints.json)
    #[arg(long = "artifacts", value_name = "PATH", default_value = "./pico-proof-artifacts")]
    pub artifacts_path: PathBuf,

    /// Field type for the proving backend
    #[arg(
        long = "field-type",
        value_enum,
        default_value = "kb",
        value_name = "TYPE"
    )]
    pub field_type: FieldType,

    /// Path to write the proof artifact JSON file
    #[arg(long = "output", value_name = "PATH")]
    pub output_path: Option<PathBuf>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum FieldType {
    /// KoalaBear field (default)
    #[value(name = "kb")]
    KoalaBear,

    /// BabyBear field
    #[value(name = "bb")]
    BabyBear,
}

impl FieldType {
    /// Convert to the string format expected by pico-sdk
    pub fn as_str(&self) -> &'static str {
        match self {
            FieldType::KoalaBear => "kb",
            FieldType::BabyBear => "bb",
        }
    }
}
