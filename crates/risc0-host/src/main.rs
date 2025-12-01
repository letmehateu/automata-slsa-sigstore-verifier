//! RISC0 zkVM host program for Sigstore attestation verification
//!
//! This CLI tool generates zero-knowledge proofs of Sigstore attestation bundle
//! verification using RISC0 zkVM.

mod cli;
mod config;
mod prover;
mod proving {
    pub mod boundless;
}

use anyhow::{Context, Result};
use clap::Parser;
use sigstore_verifier::types::result::{VerificationOptions, VerificationResult};
use sigstore_zkvm_traits::traits::ZkVmProver;
use sigstore_zkvm_traits::utils::{display_proof_result, display_verification_result, write_proof_artifact, ProofArtifact};
use sigstore_zkvm_traits::workflow::prepare_guest_input_local;

#[tokio::main]
async fn main() -> Result<()> {
    // Load .env file if present (ignore errors if file doesn't exist)
    dotenvy::dotenv().ok();

    // Parse CLI arguments
    let cli = crate::cli::Cli::parse();

    match cli.command {
        crate::cli::Commands::ImageId => {
            handle_image_id()?;
        }
        crate::cli::Commands::Prove(args) => {
            handle_prove(args).await?;
        }
    }

    Ok(())
}

/// Handle the image-id command
///
/// Displays the RISC0 ImageID of the guest program.
fn handle_image_id() -> Result<()> {
    // Create prover to get image ID
    let prover = crate::prover::Risc0Prover::new()
        .context("Failed to create RISC0 prover")?;

    let image_id = prover.program_identifier()
        .context("Failed to get program identifier")?;

    let circuit_version = crate::prover::Risc0Prover::circuit_version();

    println!("Image ID:        {}", image_id);
    println!("Circuit Version: {}", circuit_version);

    Ok(())
}

/// Handle the prove command
///
/// Generates a proof of Sigstore attestation verification.
async fn handle_prove(args: crate::cli::ProveArgs) -> Result<()> {
    println!("RISC0 Sigstore Proof Generation");
    println!("================================\n");

    // Step 1: Prepare guest input
    println!("📦 Preparing guest input...");
    println!("   Bundle:       {}", args.bundle_path.display());
    println!("   Trusted Root: {}", args.trust_roots_path.display());

    let verification_options = VerificationOptions {
        expected_digest: None,
        expected_issuer: None,
        expected_subject: None,
    };

    let prover_input = prepare_guest_input_local(
        &args.bundle_path,
        &args.trust_roots_path,
        verification_options,
    )
    .context("Failed to prepare guest input")?;

    println!("✓ Guest input prepared\n");

    // Step 2: Create prover
    println!("🔧 Initializing RISC0 prover...");
    let prover = crate::prover::Risc0Prover::new()
        .context("Failed to create RISC0 prover")?;
    println!("✓ Prover initialized\n");

    // Step 3: Build config
    let config = crate::config::Risc0Config::from_cli_args(&args);

    // Step 4: Generate proof
    println!("⚙️  Generating proof...");
    let (journal, seal) = prover
        .prove(&config, &prover_input)
        .await
        .context("Failed to generate proof")?;

    println!("✓ Proof generated successfully\n");

    // Step 5: Display proof result
    display_proof_result(&journal, &seal);

    // Step 6: Decode and display verification result
    println!("\n🔍 Decoding verification result...");
    let verification_result = VerificationResult::from_slice(&journal)
        .map_err(|e| anyhow::anyhow!("Failed to decode verification result from journal: {}", e))?;

    display_verification_result(&verification_result);

    // Step 7: Write artifact if output path provided
    if let Some(ref output_path) = args.output_path {
        println!("\n💾 Writing proof artifact...");
    
        let artifact = ProofArtifact {
            zkvm: "risc0".to_string(),
            program_id: format!("0x{}", prover.program_identifier()?),
            circuit_version: crate::prover::Risc0Prover::circuit_version(),
            journal: format!("0x{}", hex::encode(&journal)),
            proof: format!("0x{}", hex::encode(&seal)),
        };
        
        write_proof_artifact(output_path, &artifact)
            .context("Failed to write proof artifact")?;
    }

    println!("\n✅ Success!");

    Ok(())
}
