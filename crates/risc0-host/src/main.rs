use risc0_zkvm::{default_executor, ExecutorEnv};
use sigstore_risc0_methods::SIGSTORE_RISC0_GUEST_ELF;
use sigstore_verifier::fetcher::jsonl::parser::{
    load_trusted_root_from_jsonl, select_certificate_authority, select_timestamp_authority,
};
use sigstore_verifier::parser::bundle::{extract_bundle_timestamp, parse_bundle_from_path};
use sigstore_verifier::types::certificate::FulcioInstance;
use sigstore_verifier::types::result::{VerificationOptions, VerificationResult};
use sigstore_zkvm_traits::types::ProverInput;
use std::fs;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting RISC0 zkVM Host Program");

    // Read the attestation bundle from samples/
    let bundle_path = Path::new("samples/actions-attest-build-provenance-attestation-13532655.sigstore.json");
    println!("Reading bundle from: {}", bundle_path.display());
    let bundle_json = fs::read(bundle_path)?;
    println!("Bundle size: {} bytes", bundle_json.len());

    // Auto-detect Fulcio instance from bundle
    let bundle_json_str = String::from_utf8(bundle_json.clone())?;
    let fulcio_instance = FulcioInstance::from_bundle_json(&bundle_json_str)
        .expect("Failed to detect Fulcio instance");
    println!("Detected Fulcio instance: {:?}", fulcio_instance);

    // Load trusted roots for Fulcio and TSA
    let trusted_root_path = "samples/trusted_root.jsonl";
    println!("Reading trusted root from: {}", trusted_root_path);
    let trusted_root_content = fs::read_to_string(trusted_root_path)?;
    let trust_roots = load_trusted_root_from_jsonl(&trusted_root_content)?;
    println!("Loaded {} trust root entries", trust_roots.len());

    // Parse the Sigstore bundle
    let bundle = parse_bundle_from_path(bundle_path)?;

    // Extract timestamp from the bundle
    let timestamp = extract_bundle_timestamp(&bundle)?;
    println!("Bundle timestamp: {}", timestamp);

    // Select the appropriate certificate chains based on Fulcio instance and timestamp
    let fulcio_chain = select_certificate_authority(&trust_roots, &fulcio_instance, timestamp)?;
    println!(
        "Selected Fulcio chain with {} intermediates",
        fulcio_chain.intermediates.len()
    );

    let tsa_chain = select_timestamp_authority(&trust_roots, &fulcio_instance, timestamp)?;
    println!(
        "Selected TSA chain with {} intermediates",
        tsa_chain.intermediates.len()
    );

    // Create verification options
    let verification_options = VerificationOptions {
        expected_digest: None,
        allow_insecure_sct: true,
        expected_issuer: None,
        expected_subject: None,
    };

    // Create the ProverInput with properly selected certificate chains
    let prover_input = ProverInput::new(
        bundle_json,
        verification_options,
        fulcio_chain,
        Some(tsa_chain),
    );

    println!("Preparing zkVM execution environment...");

    // Create the executor environment and write the input
    let env = ExecutorEnv::builder()
        .write(&prover_input)?
        .build()?;

    println!("Executing zkVM guest program...");
    println!("Note: Running in RISC0_DEV_MODE (set via environment variable)");

    // Execute the guest program
    let session = default_executor()
        .execute(env, SIGSTORE_RISC0_GUEST_ELF)?;

    println!("zkVM execution completed successfully!");

    // Read the verification result from the session
    let verification_result: VerificationResult = session.journal.decode()?;

    println!("\n=== Verification Result ===");
    println!("Subject digest: {}", hex::encode(&verification_result.subject_digest));
    println!("Signing time: {}", verification_result.signing_time);
    println!("Leaf cert hash: {}", hex::encode(verification_result.certificate_hashes.leaf));
    println!("Root cert hash: {}", hex::encode(verification_result.certificate_hashes.root));

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
