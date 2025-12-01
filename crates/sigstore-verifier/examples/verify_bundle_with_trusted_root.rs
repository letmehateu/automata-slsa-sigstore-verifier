use sigstore_verifier::fetcher::jsonl::parser::{
    load_trusted_root_from_jsonl, select_certificate_authority, select_timestamp_authority,
};
use sigstore_verifier::parser::bundle::{extract_bundle_timestamp, parse_bundle_from_path};
use sigstore_verifier::types::certificate::FulcioInstance;
use sigstore_verifier::types::result::VerificationOptions;
use sigstore_verifier::AttestationVerifier;
use std::env;
use std::path::PathBuf;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!(
            "Usage: {} <path-to-sigstore-bundle.json> <path-to-trusted_root.jsonl>",
            args[0]
        );
        eprintln!();
        eprintln!("Example:");
        eprintln!(
            "  {} bundle.json samples/trusted_root.jsonl",
            args[0]
        );
        std::process::exit(1);
    }

    let bundle_path = PathBuf::from(&args[1]);
    let trusted_root_path = PathBuf::from(&args[2]);

    if !bundle_path.exists() {
        eprintln!("Error: Bundle file not found: {}", bundle_path.display());
        std::process::exit(1);
    }

    if !trusted_root_path.exists() {
        eprintln!(
            "Error: Trusted root file not found: {}",
            trusted_root_path.display()
        );
        std::process::exit(1);
    }

    println!("Verifying bundle: {}", bundle_path.display());
    println!("Using trusted root: {}", trusted_root_path.display());
    println!();

    // Load and parse the trusted root bundle
    let trusted_root_content = std::fs::read_to_string(&trusted_root_path)
        .expect("Failed to read trusted root file");

    let trust_roots = load_trusted_root_from_jsonl(&trusted_root_content)
        .expect("Failed to parse trusted root JSONL");

    println!("Loaded {} trust bundle(s) from JSONL", trust_roots.len());

    // Parse the Sigstore bundle
    let bundle = parse_bundle_from_path(&bundle_path).expect("Failed to parse bundle");

    // Extract timestamp from the bundle
    let timestamp = extract_bundle_timestamp(&bundle).expect("Failed to extract timestamp");
    println!("Bundle timestamp: {} (Unix seconds)", timestamp);

    // Detect Fulcio instance
    let bundle_json = std::fs::read_to_string(&bundle_path).expect("Failed to read bundle file");
    let fulcio_instance = FulcioInstance::from_bundle_json(&bundle_json)
        .expect("Failed to detect Fulcio instance from bundle");

    println!("Detected Fulcio instance: {:?}", fulcio_instance);
    println!();

    // Select appropriate certificate chains from trusted root
    let fulcio_chain = select_certificate_authority(&trust_roots, &fulcio_instance, timestamp)
        .expect("Failed to select certificate authority");

    let tsa_chain = select_timestamp_authority(&trust_roots, &fulcio_instance, timestamp)
        .expect("Failed to select timestamp authority");

    println!("Selected certificate authority and timestamp authority from trusted root");
    println!();

    // Verify the bundle
    let verifier = AttestationVerifier::new();

    let options = VerificationOptions {
        expected_digest: None,
        expected_issuer: None,
        expected_subject: None,
    };

    match verifier.verify_bundle(&bundle_path, options, &fulcio_chain, Some(&tsa_chain)) {
        Ok(result) => {
            println!("✓ Verification SUCCESS\n");

            println!("Certificate Chain Hashes:");
            println!("  Leaf:   {}", hex::encode(&result.certificate_hashes.leaf));
            for (i, hash) in result.certificate_hashes.intermediates.iter().enumerate() {
                println!("  Int[{}]: {}", i, hex::encode(hash));
            }
            println!("  Root:   {}", hex::encode(&result.certificate_hashes.root));
            println!();

            println!("Signing Time: {}", result.signing_time.to_rfc3339());
            println!("Subject Digest: {}", hex::encode(&result.subject_digest));

            if let Some(ref identity) = result.oidc_identity {
                println!("\nOIDC Identity:");
                if let Some(ref issuer) = identity.issuer {
                    println!("  Issuer: {}", issuer);
                }
                if let Some(ref subject) = identity.subject {
                    println!("  Subject: {}", subject);
                }
            }
        }
        Err(e) => {
            eprintln!("✗ Verification FAILED");
            eprintln!("\nError: {}", e);
            std::process::exit(1);
        }
    }
}
