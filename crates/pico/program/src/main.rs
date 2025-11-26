#![no_main]
pico_sdk::entrypoint!(main);

use pico_sdk::io::{commit_bytes, read_vec};

use sigstore_verifier::{AttestationVerifier, types::result::VerificationResult};
use sigstore_zkvm_traits::types::ProverInput;

fn main() {
    // Read input from host
    let input_bytes: Vec<u8> = read_vec();

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
    commit_bytes(&verification_result.as_slice());
}
