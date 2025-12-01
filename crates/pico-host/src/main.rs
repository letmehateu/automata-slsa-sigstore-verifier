//! Pico zkVM host program for Sigstore attestation verification
//!
//! This CLI tool generates zero-knowledge proofs of Sigstore attestation bundle
//! verification using Pico zkVM.

mod cli;
mod config;
mod prover;

use anyhow::{Context, Result};
use clap::Parser;
use sigstore_verifier::types::result::{VerificationOptions, VerificationResult};
use sigstore_zkvm_traits::traits::ZkVmProver;
use sigstore_zkvm_traits::utils::{
    display_proof_result, display_verification_result, write_proof_artifact, ProofArtifact,
};
use sigstore_zkvm_traits::workflow::prepare_guest_input_local;

#[tokio::main]
async fn main() -> Result<()> {
    // Load .env file if present (ignore errors if file doesn't exist)
    dotenvy::dotenv().ok();

    // Parse CLI arguments
    let cli = crate::cli::Cli::parse();

    match cli.command {
        crate::cli::Commands::ProgramId => {
            handle_program_id()?;
        }
        crate::cli::Commands::Prove(args) => {
            handle_prove(args).await?;
        }
    }

    Ok(())
}

/// Handle the program-id command
///
/// Displays the Pico program identifier (VK hash).
fn handle_program_id() -> Result<()> {
    // Create prover to get program ID
    let prover =
        crate::prover::PicoProver::new().context("Failed to create Pico prover")?;

    let program_id = prover
        .program_identifier()
        .context("Failed to get program identifier")?;

    let circuit_version = crate::prover::PicoProver::circuit_version();

    println!("Program ID:      {}", program_id);
    println!("Circuit Version: {}", circuit_version);

    Ok(())
}

/// Handle the prove command
///
/// Generates a proof of Sigstore attestation verification.
async fn handle_prove(args: crate::cli::ProveArgs) -> Result<()> {
    println!("Pico Sigstore Proof Generation");
    println!("===============================\n");

    // Step 1: Prepare guest input
    println!("Preparing guest input...");
    println!("   Bundle:       {}", args.bundle_path.display());
    println!("   Trusted Root: {}", args.trust_roots_path.display());
    println!("   Artifacts:    {}", args.artifacts_path.display());
    println!("   Field Type:   {}", args.field_type.as_str());

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

    println!("Guest input prepared\n");

    // Step 2: Create prover
    println!("Initializing Pico prover...");
    let prover =
        crate::prover::PicoProver::new().context("Failed to create Pico prover")?;
    println!("Prover initialized\n");

    // Step 3: Build config
    let config = crate::config::PicoConfig::from_cli_args(&args);

    // Step 4: Generate proof
    println!("Generating proof...");
    let (journal, proof) = prover
        .prove(&config, &prover_input)
        .await
        .context("Failed to generate proof")?;

    println!("Proof generated successfully\n");

    // Step 5: Display proof result
    display_proof_result(&journal, &proof);

    // Step 6: Decode and display verification result
    println!("\nDecoding verification result...");
    let verification_result = VerificationResult::from_slice(&journal).map_err(|e| {
        anyhow::anyhow!(
            "Failed to decode verification result from journal: {}",
            e
        )
    })?;

    display_verification_result(&verification_result);

    // Step 7: Write artifact if output path provided
    if let Some(ref output_path) = args.output_path {
        println!("\nWriting proof artifact...");

        let artifact = ProofArtifact {
            zkvm: "pico".to_string(),
            program_id: prover.program_identifier()?,
            circuit_version: crate::prover::PicoProver::circuit_version(),
            journal: format!("0x{}", hex::encode(&journal)),
            proof: format!("0x{}", hex::encode(&proof)),
        };

        write_proof_artifact(output_path, &artifact)
            .context("Failed to write proof artifact")?;
    }

    println!("\nSuccess!");

    Ok(())
}
