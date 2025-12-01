//! Workflow module for preparing zkVM guest inputs
//!
//! This module provides utilities to prepare input data for zkVM guest programs
//! that verify Sigstore attestation bundles.

use crate::types::ProverInput;
use anyhow::{Context, Result};
use sigstore_verifier::fetcher::jsonl::parser::{
    load_trusted_root_from_jsonl, select_certificate_authority, select_timestamp_authority,
};
use sigstore_verifier::parser::bundle::{extract_bundle_timestamp, parse_bundle_from_path};
use sigstore_verifier::types::certificate::FulcioInstance;
use sigstore_verifier::types::result::VerificationOptions;
use std::fs;
use std::path::Path;

/// Prepare zkVM guest input from local files
///
/// This function reads the Sigstore bundle, trusted root, and prepares all necessary
/// data for the zkVM guest program to perform verification. It operates entirely on
/// local files without requiring the fetcher feature.
///
/// # Arguments
///
/// * `bundle_path` - Path to the Sigstore attestation bundle JSON file
/// * `trusted_root_path` - Path to the trusted root JSONL file containing CA and TSA certificate chains
/// * `options` - Verification options (expected digest, issuer, subject, etc.)
///
/// # Returns
///
/// Returns a `ProverInput` containing:
/// - The attestation bundle JSON
/// - Verification options
/// - Fulcio certificate chain
/// - TSA certificate chain (if available)
///
/// # Errors
///
/// This function will return an error if:
/// - The bundle file cannot be read or parsed
/// - The trusted root file cannot be read or parsed
/// - The Fulcio instance cannot be auto-detected from the bundle
/// - The appropriate certificate chains cannot be selected based on the bundle timestamp
///
/// # Example
///
/// ```ignore
/// use sigstore_zkvm_traits::workflow::prepare_guest_input_local;
/// use sigstore_verifier::types::result::VerificationOptions;
/// use std::path::Path;
///
/// let bundle_path = Path::new("samples/attestation.sigstore.json");
/// let trusted_root_path = Path::new("samples/trusted_root.jsonl");
/// let options = VerificationOptions {
///     expected_digest: None,
///     expected_issuer: None,
///     expected_subject: None,
/// };
///
/// let prover_input = prepare_guest_input_local(
///     bundle_path,
///     trusted_root_path,
///     options
/// )?;
/// ```
pub fn prepare_guest_input_local(
    bundle_path: &Path,
    trusted_root_path: &Path,
    options: VerificationOptions,
) -> Result<ProverInput> {
    // Read the attestation bundle
    let bundle_json = fs::read(bundle_path)
        .context(format!("Failed to read bundle from: {}", bundle_path.display()))?;

    // Auto-detect Fulcio instance from bundle
    let bundle_json_str = String::from_utf8(bundle_json.clone())
        .context("Failed to parse bundle as UTF-8")?;
    let fulcio_instance = FulcioInstance::from_bundle_json(&bundle_json_str)
        .map_err(|e| anyhow::anyhow!("Failed to detect Fulcio instance from bundle: {}", e))?;

    // Load trusted roots for Fulcio and TSA
    let trusted_root_content = fs::read_to_string(trusted_root_path)
        .context(format!("Failed to read trusted root from: {}", trusted_root_path.display()))?;
    let trust_roots = load_trusted_root_from_jsonl(&trusted_root_content)
        .context("Failed to parse trusted root JSONL")?;

    // Parse the Sigstore bundle
    let bundle = parse_bundle_from_path(bundle_path)
        .context("Failed to parse Sigstore bundle")?;

    // Extract timestamp from the bundle
    let timestamp = extract_bundle_timestamp(&bundle)
        .context("Failed to extract timestamp from bundle")?;

    // Select the appropriate certificate chains based on Fulcio instance and timestamp
    let fulcio_chain = select_certificate_authority(&trust_roots, &fulcio_instance, timestamp)
        .context("Failed to select Fulcio certificate authority")?;

    let tsa_chain = select_timestamp_authority(&trust_roots, &fulcio_instance, timestamp)
        .context("Failed to select TSA certificate authority")?;

    // Create the ProverInput with properly selected certificate chains
    Ok(ProverInput::new(
        bundle_json,
        options,
        fulcio_chain,
        Some(tsa_chain),
    ))
}
