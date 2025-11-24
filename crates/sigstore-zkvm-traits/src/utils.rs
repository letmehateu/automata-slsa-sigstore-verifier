//! Utility functions for zkVM proof generation and artifact management
//!
//! This module provides shared utilities for all zkVM implementations including:
//! - Proof artifact serialization
//! - Result display functions
//! - Common output formatting

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use sigstore_verifier::types::result::VerificationResult;
use std::fs;
use std::path::Path;

/// Proof artifact structure for serialization
///
/// This structure contains all the necessary information to verify a proof on-chain:
/// - zkvm: The zkVM system used (e.g., "risc0", "sp1")
/// - program_id: The unique identifier of the guest program (e.g., ImageID for RISC0)
/// - circuit_version: The version of the zkVM circuit used
/// - journal: Hex-encoded public output/journal from the guest program
/// - proof: Hex-encoded proof bytes (e.g., Groth16 proof, Merkle proof)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProofArtifact {
    pub zkvm: String,
    pub program_id: String,
    pub circuit_version: String,
    pub journal: String,
    pub proof: String,
}

/// Write a proof artifact to a JSON file
///
/// Creates the parent directory if it doesn't exist and writes the artifact
/// as pretty-printed JSON.
///
/// # Arguments
///
/// * `output_path` - Path where the artifact JSON file will be written
/// * `artifact` - The proof artifact to serialize
///
/// # Returns
///
/// Returns `Ok(())` if successful, or an error if file operations fail.
///
/// # Example
///
/// ```ignore
/// let artifact = ProofArtifact {
///     zkvm: "risc0".to_string(),
///     program_id: "0x1234...".to_string(),
///     circuit_version: "1.0.0".to_string(),
///     journal: hex::encode(&journal_bytes),
///     proof: hex::encode(&proof_bytes),
/// };
/// write_proof_artifact(Path::new("output/proof.json"), &artifact)?;
/// ```
pub fn write_proof_artifact(output_path: &Path, artifact: &ProofArtifact) -> Result<()> {
    // Create parent directories if they don't exist
    if let Some(parent) = output_path.parent() {
        fs::create_dir_all(parent)
            .context(format!("Failed to create directory: {}", parent.display()))?;
    }

    // Serialize to pretty JSON
    let json = serde_json::to_string_pretty(artifact)
        .context("Failed to serialize proof artifact")?;

    // Write to file
    fs::write(output_path, json)
        .context(format!("Failed to write proof artifact to: {}", output_path.display()))?;

    println!("✓ Proof artifact written to: {}", output_path.display());
    Ok(())
}

/// Display verification result in a readable format
///
/// Prints the verification result with formatted output including:
/// - Subject digest
/// - Signing time
/// - Certificate hashes (leaf, intermediates, root)
/// - OIDC identity information (if present)
///
/// # Arguments
///
/// * `result` - The verification result to display
///
/// # Example
///
/// ```ignore
/// let result = VerificationResult::from_slice(&journal)?;
/// display_verification_result(&result);
/// ```
pub fn display_verification_result(result: &VerificationResult) {
    println!("\n=== Verification Result ===");
    println!("Subject digest: {}", hex::encode(&result.subject_digest));
    println!("Signing time:   {}", result.signing_time);

    println!("\nCertificate Hashes:");
    println!("  Leaf:   {}", hex::encode(result.certificate_hashes.leaf));
    if !result.certificate_hashes.intermediates.is_empty() {
        println!("  Intermediates:");
        for (i, intermediate) in result.certificate_hashes.intermediates.iter().enumerate() {
            println!("    [{}] {}", i, hex::encode(intermediate));
        }
    }
    println!("  Root:   {}", hex::encode(result.certificate_hashes.root));

    if let Some(ref oidc) = result.oidc_identity {
        println!("\nOIDC Identity:");
        if let Some(ref issuer) = oidc.issuer {
            println!("  Issuer:       {}", issuer);
        }
        if let Some(ref subject) = oidc.subject {
            println!("  Subject:      {}", subject);
        }
        if let Some(ref workflow_ref) = oidc.workflow_ref {
            println!("  Workflow:     {}", workflow_ref);
        }
        if let Some(ref repository) = oidc.repository {
            println!("  Repository:   {}", repository);
        }
        if let Some(ref event_name) = oidc.event_name {
            println!("  Event:        {}", event_name);
        }
    }
}

/// Display proof generation result summary
///
/// Prints a summary of the proof generation including journal and proof sizes.
///
/// # Arguments
///
/// * `journal` - The public output/journal bytes
/// * `seal` - The proof bytes
/// * `proof_type` - Description of the proof type (e.g., "Groth16", "Merkle", "Seal")
///
/// # Example
///
/// ```ignore
/// display_proof_result(&journal, &seal);
/// ```
pub fn display_proof_result(journal: &[u8], seal: &[u8]) {
    println!("\n=== Proof Generation Result ===");
    println!("Journal: {}", hex::encode(&journal));
    if seal.is_empty() {
        println!("<empty-proof> (DEV_MODE)");
    } else {
        println!("Proof: {}", hex::encode(&seal));
    }
}
