use risc0_zkvm::{default_executor, ExecutorEnv};
use sigstore_risc0_methods::SIGSTORE_RISC0_GUEST_ELF;
use sigstore_verifier::types::result::{VerificationOptions, VerificationResult};
use sigstore_zkvm_traits::workflow::prepare_guest_input_local;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting RISC0 zkVM Host Program");

    // Define paths
    let bundle_path =
        Path::new("samples/actions-attest-build-provenance-attestation-13532655.sigstore.json");
    let trusted_root_path = Path::new("samples/trusted_root.jsonl");

    println!("Reading bundle from: {}", bundle_path.display());
    println!("Reading trusted root from: {}", trusted_root_path.display());

    // Create verification options
    let verification_options = VerificationOptions {
        expected_digest: None,
        allow_insecure_sct: false,
        expected_issuer: None,
        expected_subject: None,
    };

    // Prepare guest input using the unified workflow interface
    println!("Preparing guest input...");
    let prover_input =
        prepare_guest_input_local(bundle_path, trusted_root_path, verification_options)?;

    println!("Guest input prepared successfully");

    println!("Preparing zkVM execution environment...");

    // Create the executor environment and write the input
    let env = ExecutorEnv::builder().write(&prover_input)?.build()?;

    println!("Executing zkVM guest program...");
    println!("Note: Running in RISC0_DEV_MODE (set via environment variable)");

    // Execute the guest program
    let session = default_executor().execute(env, SIGSTORE_RISC0_GUEST_ELF)?;

    println!("zkVM execution completed successfully!");

    // Read the verification result from the session
    let verification_result_bytes = session.journal.bytes;
    let verification_result = VerificationResult::from_slice(&verification_result_bytes)?;

    println!("\n=== Verification Result ===");
    println!(
        "Subject digest: {}",
        hex::encode(&verification_result.subject_digest)
    );
    println!("Signing time: {}", verification_result.signing_time);
    println!(
        "Leaf cert hash: {}",
        hex::encode(verification_result.certificate_hashes.leaf)
    );
    println!(
        "Root cert hash: {}",
        hex::encode(verification_result.certificate_hashes.root)
    );

    if let Some(oidc) = verification_result.oidc_identity {
        println!("\nOIDC Identity:");
        if let Some(issuer) = oidc.issuer {
            println!("  Issuer: {}", issuer);
        }
        if let Some(subject) = oidc.subject {
            println!("  Subject: {}", subject);
        }
    }

    println!("\n=== Success! ===");
    println!("The attestation was successfully verified inside the zkVM!");

    Ok(())
}
