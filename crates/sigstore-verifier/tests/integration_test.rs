use sigstore_verifier::{
    fetcher::fetch_trust_bundle,
    types::{FulcioInstance, VerificationOptions},
    AttestationVerifier,
};
use std::path::PathBuf;

#[test]
#[ignore] // Requires network access to fetch trust bundles
fn test_verify_rekor_bundle() {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.pop();
    path.pop();
    path.push("samples/actions-attest-build-provenance-attestation-13532655.sigstore.json");

    // Auto-detect Fulcio instance from bundle
    let bundle_json = std::fs::read_to_string(&path).expect("Failed to read bundle");
    let instance = FulcioInstance::from_bundle_json(&bundle_json)
        .expect("Failed to detect Fulcio instance");

    // Fetch trust bundle for detected instance
    // In production, the client should fetch and cache this
    let trust_bundle = fetch_trust_bundle(&instance).expect("Failed to fetch trust bundle");

    let verifier = AttestationVerifier::new();
    let options = VerificationOptions {
        expected_digest: None,
        allow_insecure_sct: false, // Not yet implemented
        expected_issuer: None,
        expected_subject: None,
    };

    let result = verifier.verify_bundle(&path, &trust_bundle, options);
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
