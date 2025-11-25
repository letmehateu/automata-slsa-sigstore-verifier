#![no_main]
sp1_zkvm::entrypoint!(main);

use sigstore_verifier::{
    AttestationVerifier,
    types::result::VerificationResult
};
use sigstore_zkvm_traits::types::ProverInput;

fn main() {
    // read the values passed from host
    let input_bytes: Vec<u8> = sp1_zkvm::io::read_vec();

    let input: ProverInput = ProverInput::parse_input(&input_bytes)
        .expect("Failed to parse ProverInput");

    let verifier = AttestationVerifier::new();

    let output = verifier.verify_bundle_bytes(
        &input.bundle_json,
        input.verification_options,
        &input.trust_bundle,
        input.tsa_cert_chain.as_ref(),
    );

    assert!(output.is_ok(), "Failed to verify bundle");

    let verification_result: VerificationResult = output.unwrap();
    sp1_zkvm::io::commit_slice(&verification_result.as_slice());
}