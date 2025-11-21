pub mod crypto;
pub mod error;
pub mod fetcher;
pub mod parser;
pub mod types;
pub mod verifier;

use std::path::Path;

use error::VerificationError;
use parser::{extract_oidc_identity, parse_bundle_from_bytes, parse_bundle_from_path, parse_dsse_payload};
use types::{CertificateChain, VerificationOptions, VerificationResult};
use verifier::{
    get_signing_time, verify_certificate_chain, verify_dsse_signature,
    verify_signing_time_in_validity, verify_subject_digest, verify_transparency_log,
};

/// Main attestation verifier
#[derive(Debug, Clone, Default)]
pub struct AttestationVerifier {}

impl AttestationVerifier {
    /// Create a new verifier instance
    pub fn new() -> Self {
        Self::default()
    }

    /// Verify a sigstore bundle from a file path
    ///
    /// # Arguments
    ///
    /// * `bundle_path` - Path to the sigstore bundle JSON file
    /// * `trust_bundle` - Certificate chain (intermediates and root) for verification
    /// * `options` - Verification options
    ///
    /// # Returns
    ///
    /// On success, returns `VerificationResult` containing:
    /// - Certificate chain hashes (leaf, intermediates, root)
    /// - Signing time
    /// - Subject digest
    /// - OIDC identity (if present)
    pub fn verify_bundle(
        &self,
        bundle_path: &Path,
        trust_bundle: &CertificateChain,
        options: VerificationOptions,
    ) -> Result<VerificationResult, VerificationError> {
        let bundle = parse_bundle_from_path(bundle_path)?;
        self.verify_bundle_internal(&bundle, trust_bundle, options)
    }

    /// Verify a sigstore bundle from raw JSON bytes
    ///
    /// # Arguments
    ///
    /// * `bundle_json` - Raw JSON bytes of the sigstore bundle
    /// * `trust_bundle` - Certificate chain (intermediates and root) for verification
    /// * `options` - Verification options
    ///
    /// # Returns
    ///
    /// On success, returns `VerificationResult` containing:
    /// - Certificate chain hashes (leaf, intermediates, root)
    /// - Signing time
    /// - Subject digest
    /// - OIDC identity (if present)
    pub fn verify_bundle_bytes(
        &self,
        bundle_json: &[u8],
        trust_bundle: &CertificateChain,
        options: VerificationOptions,
    ) -> Result<VerificationResult, VerificationError> {
        let bundle = parse_bundle_from_bytes(bundle_json)?;
        self.verify_bundle_internal(&bundle, trust_bundle, options)
    }

    fn verify_bundle_internal(
        &self,
        bundle: &types::SigstoreBundle,
        trust_bundle: &CertificateChain,
        options: VerificationOptions,
    ) -> Result<VerificationResult, VerificationError> {
        // Step 1: Parse and verify subject digest
        let statement = parse_dsse_payload(&bundle.dsse_envelope)?;
        let subject_digest = verify_subject_digest(&statement, options.expected_digest.as_deref())?;

        // Step 2: Get signing time (from integrated time - RFC3161 not yet supported)
        let signing_time = get_signing_time(bundle)?;

        // Step 3: Verify certificate chain and get hashes
        let (chain, certificate_hashes) = verify_certificate_chain(bundle, trust_bundle)?;

        // Step 3b: Verify signing time is within certificate validity period
        let leaf_cert = parser::parse_der_certificate(&chain.leaf)
            .map_err(|e| VerificationError::InvalidBundleFormat(e.to_string()))?;
        verify_signing_time_in_validity(&signing_time, &leaf_cert)?;

        // Step 4: Verify DSSE signature
        verify_dsse_signature(&bundle.dsse_envelope, &chain)?;

        // Step 5: Verify transparency log
        verify_transparency_log(bundle)?;

        // Step 6: Extract OIDC identity from certificate extensions
        let oidc_identity = extract_oidc_identity(&leaf_cert).ok();

        // Step 7: Verify OIDC identity against expected values (if specified)
        if let Some(ref identity) = oidc_identity {
            if let Some(ref expected_issuer) = options.expected_issuer {
                if let Some(ref actual_issuer) = identity.issuer {
                    if actual_issuer != expected_issuer {
                        return Err(VerificationError::InvalidBundleFormat(format!(
                            "OIDC issuer mismatch: expected '{}', got '{}'",
                            expected_issuer, actual_issuer
                        )));
                    }
                } else {
                    return Err(VerificationError::InvalidBundleFormat(
                        "Expected OIDC issuer but none found in certificate".to_string(),
                    ));
                }
            }

            if let Some(ref expected_subject) = options.expected_subject {
                if let Some(ref actual_subject) = identity.subject {
                    if actual_subject != expected_subject {
                        return Err(VerificationError::InvalidBundleFormat(format!(
                            "OIDC subject mismatch: expected '{}', got '{}'",
                            expected_subject, actual_subject
                        )));
                    }
                } else {
                    return Err(VerificationError::InvalidBundleFormat(
                        "Expected OIDC subject but none found in certificate".to_string(),
                    ));
                }
            }
        } else if options.expected_issuer.is_some() || options.expected_subject.is_some() {
            return Err(VerificationError::InvalidBundleFormat(
                "Expected OIDC identity but could not extract from certificate".to_string(),
            ));
        }

        Ok(VerificationResult {
            certificate_hashes,
            signing_time,
            subject_digest,
            oidc_identity,
        })
    }
}
