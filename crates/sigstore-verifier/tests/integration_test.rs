use sigstore_verifier::types::certificate::FulcioInstance;
use sigstore_verifier::types::result::VerificationOptions;
use sigstore_verifier::AttestationVerifier;
use std::path::PathBuf;

#[test]
#[cfg(feature = "fetcher")]
fn test_verify_rekor_bundle() {
    use sigstore_verifier::fetcher::trust_bundle::fetch_fulcio_trust_bundle;

    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.pop();
    path.pop();
    path.push("samples/actions-attest-build-provenance-attestation-13532655.sigstore.json");

    // Auto-detect Fulcio instance from bundle
    let bundle_json = std::fs::read_to_string(&path).expect("Failed to read bundle");
    let instance =
        FulcioInstance::from_bundle_json(&bundle_json).expect("Failed to detect Fulcio instance");

    // Fetch trust bundle for detected instance
    // In production, the client should fetch and cache this
    let trust_bundle = fetch_fulcio_trust_bundle(&instance).expect("Failed to fetch trust bundle");

    let verifier = AttestationVerifier::new();
    let options = VerificationOptions {
        expected_digest: None,
        expected_issuer: None,
        expected_subject: None,
    };

    let result = verifier.verify_bundle(&path, options, &trust_bundle, None);
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

#[test]
fn test_verify_rfc3161_bundle() {
    use sigstore_verifier::fetcher::jsonl::parser::{
        load_trusted_root_from_jsonl, select_certificate_authority, select_timestamp_authority,
    };
    use sigstore_verifier::parser::bundle::{
        extract_bundle_timestamp, parse_bundle_from_path,
    };

    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.pop();
    path.pop();
    path.push("samples/actions-attest-build-provenance-attestation-13581567.sigstore.json");

    // Auto-detect Fulcio instance from bundle
    let bundle_json = std::fs::read_to_string(&path).expect("Failed to read bundle");
    let fulcio_instance =
        FulcioInstance::from_bundle_json(&bundle_json).expect("Failed to detect Fulcio instance");

    // Load trusted roots for Fulcio and TSA
    let mut trusted_root_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    trusted_root_path.pop();
    trusted_root_path.pop();
    trusted_root_path.push("samples/trusted_root.jsonl");
    let trusted_root_content = std::fs::read_to_string(&trusted_root_path)
        .expect("Failed to read trusted root file");
    let trust_roots = load_trusted_root_from_jsonl(&trusted_root_content)
        .expect("Failed to parse trusted root JSONL");

    // Parse the Sigstore bundle
    let bundle = parse_bundle_from_path(&path).expect("Failed to parse bundle");

    // Extract timestamp from the bundle
    let timestamp = extract_bundle_timestamp(&bundle).expect("Failed to extract timestamp");

    let verifier = AttestationVerifier::new();
    let options = VerificationOptions {
        expected_digest: None,
        expected_issuer: None,
        expected_subject: None,
    };

    let fulcio_chain = select_certificate_authority(&trust_roots, &fulcio_instance, timestamp)
        .expect("Failed to select certificate authority");
    let tsa_chain = select_timestamp_authority(&trust_roots, &fulcio_instance, timestamp)
        .expect("Failed to select timestamp authority");

    let result = verifier.verify_bundle(&path, options, &fulcio_chain, Some(&tsa_chain));
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