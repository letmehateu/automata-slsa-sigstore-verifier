// This example demonstrates verifying a Sigstore bundle by fetching
// certificate trust bundles from remote URLs.
//
// For an alternative approach using local trust root bundles (JSONL format),
// see the verify_bundle_with_trusted_root example.

use sigstore_verifier::fetcher::trust_bundle::{
    fetch_fulcio_trust_bundle, fetch_trust_bundle_from_url,
};
use sigstore_verifier::types::certificate::FulcioInstance;
use sigstore_verifier::types::result::VerificationOptions;
use sigstore_verifier::AttestationVerifier;
use std::env;
use std::path::PathBuf;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <path-to-sigstore-bundle.json>", args[0]);
        std::process::exit(1);
    }

    let bundle_path = PathBuf::from(&args[1]);

    if !bundle_path.exists() {
        eprintln!("Error: File not found: {}", bundle_path.display());
        std::process::exit(1);
    }

    let bundle_json = std::fs::read_to_string(&bundle_path).expect("Failed to read bundle file");
    let fulcio_instance = FulcioInstance::from_bundle_json(&bundle_json)
        .expect("Failed to detect Fulcio instance from bundle");

    println!("Verifying bundle: {}", bundle_path.display());
    println!();

    let verifier = AttestationVerifier::new();

    let options = VerificationOptions {
        expected_digest: None,
        expected_issuer: None,
        expected_subject: None,
    };

    let fulcio_issuer_chain =
        fetch_fulcio_trust_bundle(&fulcio_instance).expect("Failed to fetch Fulcio trust bundle");

    let tsa_trust_bundle = match fulcio_instance {
        FulcioInstance::GitHub => Some(
            fetch_trust_bundle_from_url(
                "https://timestamp.githubapp.com/api/v1/timestamp/certchain",
            )
            .expect("Failed to fetch TSA trust bundle"),
        ),
        _ => None,
    };

    match verifier.verify_bundle(
        &bundle_path,
        options,
        &fulcio_issuer_chain,
        tsa_trust_bundle.as_ref(),
    ) {
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
