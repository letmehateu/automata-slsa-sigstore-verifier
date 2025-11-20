use sigstore_verifier::{types::VerificationOptions, AttestationVerifier};
use std::path::PathBuf;

#[test]
#[ignore] // Requires network access to fetch trust bundles
fn test_verify_real_bundle() {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.pop();
    path.pop();
    path.push("samples/actions-attest-build-provenance-attestation-13532655.sigstore.json");

    let verifier = AttestationVerifier::new();
    let options = VerificationOptions {
        expected_digest: None,
        verify_rekor: true,
        allow_insecure_sct: false,
        expected_issuer: None,
        expected_subject: None,
    };

    let result = verifier.verify_bundle(&path, options);
    assert!(result.is_ok(), "Verification failed: {:?}", result.err());
    
    if let Ok(verification_result) = result {
        println!("Verification succeeded!");
        println!(
            "Leaf hash: {}",
            hex::encode(&verification_result.certificate_hashes.leaf)
        );
        println!(
            "Root hash: {}",
            hex::encode(&verification_result.certificate_hashes.root)
        );
        println!("Signing time: {}", verification_result.signing_time);
    }
}