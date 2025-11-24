#![no_main]

use risc0_zkvm::guest::env;
risc0_zkvm::guest::entry!(main);

use sigstore_verifier::{
    AttestationVerifier,
    types::result::VerificationResult
};
use sigstore_zkvm_traits::types::ProverInput;

fn main() {
    let input: ProverInput = env::read();

    let verifier = AttestationVerifier::new();

    let output = verifier.verify_bundle_bytes(
        &input.bundle_json,
        input.verification_options,
        &input.trust_bundle,
        input.tsa_cert_chain.as_ref(),
    );

    assert!(output.is_ok(), "Failed to verify bundle");

    let verification_result: VerificationResult = output.unwrap();
    env::commit_slice(&verification_result.as_slice());
}