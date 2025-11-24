//! # Sigstore zkVM Library
//!
//! This crate provides a trait-based interface for generating zero-knowledge proofs
//! of sigstore attestation bundle verification using various zkVM implementations.
//!
//! ## Overview
//!
//! The core abstraction is the `ZkVmProver` trait, which defines a common interface
//! for different zkVM backends (RISC0, SP1, etc.) to generate proofs that verify
//! sigstore bundles inside a zero-knowledge virtual machine.
//!
//! ## Architecture
//!
//! The verification workflow consists of:
//! 1. **Input Preparation**: Package the sigstore bundle, verification options, and
//!    trust bundles into a `ProverInput`
//! 2. **Proof Generation**: Call `prove()` on a zkVM implementation with the input
//! 3. **Output Extraction**: Deserialize the `ProverOutput` from the public output
//! 4. **On-chain Verification**: Use the proof bytes and program identifier for
//!    on-chain verification
//!
//! ## Usage
//!
//! Future zkVM implementations (RISC0, SP1) will implement the `ZkVmProver` trait:
//!
//! ```ignore
//! use sigstore_zkvm::types::{ZkVmProver, ProverInput};
//!
//! // Create prover instance
//! let prover = Risc0Prover::new()?;
//!
//! // Prepare input
//! let input = ProverInput::new(
//!     bundle_json,
//!     verification_options,
//!     trust_bundle_pem,
//!     tsa_cert_chain_pem,
//! );
//!
//! // Generate proof
//! let (public_output, proof_bytes) = prover.prove(&config, &input).await?;
//! ```

pub mod error;
pub mod traits;
pub mod types;
pub mod utils;
pub mod workflow;