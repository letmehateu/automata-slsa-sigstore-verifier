use std::path::PathBuf;
use sigstore_verifier::parser::bundle::{parse_bundle_from_path, parse_dsse_payload};
use base64::engine::general_purpose::STANDARD as BASE64;
use base64::Engine;

#[test]
fn test_rfc3161_bundle_parser() {
    use sigstore_verifier::parser::rfc3161::parse_rfc3161_timestamp;

    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.pop();
    path.pop();
    path.push("samples/actions-attest-build-provenance-attestation-13581567.sigstore.json");

    // Parse the Sigstore bundle
    let bundle = parse_bundle_from_path(&path).expect("Failed to parse bundle");
    println!("Parsed bundle: {:?}", bundle);

    // Extract timestamp from the bundle
    let timestamp_data = &bundle
        .verification_material
        .timestamp_verification_data
        .as_ref()
        .unwrap()
        .rfc3161_timestamps
        .as_ref()
        .unwrap()[0];

    let timestamp_der = BASE64
        .decode(&timestamp_data.signed_timestamp)
        .expect("Failed to decode base64 timestamp");

    let timestamp =
        parse_rfc3161_timestamp(&timestamp_der).expect("Failed to parse RFC3161 timestamp");
    println!("Parsed RFC3161 timestamp: {:?}", timestamp);

    // Extract DSSE Payload from bundle
    let statement =
        parse_dsse_payload(&bundle.dsse_envelope).expect("Failed to parse DSSE payload");
    println!("Extracted DSSE statement: {:?}", statement);
}

#[test]
fn test_rekor_bundle_parser() {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.pop();
    path.pop();
    path.push("samples/actions-attest-build-provenance-attestation-13532655.sigstore.json");

    // Parse the Sigstore bundle
    let bundle = parse_bundle_from_path(&path).expect("Failed to parse bundle");
    println!("Parsed bundle: {:?}", bundle);

    // Extract DSSE Payload from bundle
    let statement =
        parse_dsse_payload(&bundle.dsse_envelope).expect("Failed to parse DSSE payload");
    println!("Extracted DSSE statement: {:?}", statement);
}