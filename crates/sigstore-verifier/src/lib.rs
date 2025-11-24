pub mod crypto;
pub mod error;
pub mod fetcher;
pub mod parser;
pub mod types;
pub mod verifier;

use std::path::Path;

use base64::engine::general_purpose::STANDARD as BASE64;
use base64::Engine;
use error::VerificationError;
use parser::bundle::{parse_bundle_from_bytes, parse_bundle_from_path, parse_dsse_payload};
use parser::certificate::{certs_to_chain, parse_der_certificate};
use parser::identity::extract_oidc_identity;
use parser::rfc3161::parse_rfc3161_timestamp;
use types::certificate::CertificateChain;
use types::result::{VerificationOptions, VerificationResult};
use verifier::certificate::{verify_certificate_chain, verify_tsa_certificate_chain};
use verifier::rfc3161::verify_rfc3161_timestamp;
use verifier::signature::verify_dsse_signature;
use verifier::subject::verify_subject_digest;
use verifier::timestamp::{get_integrated_time, get_rfc3161_time, verify_signing_time_in_validity};
use verifier::transparency::verify_transparency_log;

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
    /// * `options` - Verification options
    /// * `trust_bundle` - Certificate chain (intermediates and root) for verification
    /// * `tsa_cert_chain` - Optional TSA certificate chain for RFC 3161 timestamp verification
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
        options: VerificationOptions,
        trust_bundle: &CertificateChain,
        tsa_cert_chain: Option<&CertificateChain>,
    ) -> Result<VerificationResult, VerificationError> {
        let bundle = parse_bundle_from_path(bundle_path)?;
        self.verify_bundle_internal(&bundle, options, trust_bundle, tsa_cert_chain)
    }

    /// Verify a sigstore bundle from raw JSON bytes
    ///
    /// # Arguments
    ///
    /// * `bundle_json` - Raw JSON bytes of the sigstore bundle
    /// * `options` - Verification options
    /// * `trust_bundle` - Certificate chain (intermediates and root) for verification
    /// * `tsa_cert_chain` - Optional TSA certificate chain for RFC 3161 timestamp verification
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
        options: VerificationOptions,
        trust_bundle: &CertificateChain,
        tsa_cert_chain: Option<&CertificateChain>,
    ) -> Result<VerificationResult, VerificationError> {
        let bundle = parse_bundle_from_bytes(bundle_json)?;
        self.verify_bundle_internal(&bundle, options, trust_bundle, tsa_cert_chain)
    }

    fn verify_bundle_internal(
        &self,
        bundle: &types::bundle::SigstoreBundle,
        options: VerificationOptions,
        trust_bundle: &CertificateChain,
        tsa_cert_chain: Option<&CertificateChain>,
    ) -> Result<VerificationResult, VerificationError> {
        // Step 1: Parse and verify subject digest
        let statement = parse_dsse_payload(&bundle.dsse_envelope)?;
        let subject_digest = verify_subject_digest(&statement, options.expected_digest.as_deref())?;

        // Step 2: Validate exactly one timestamp mechanism and get signing time
        let has_rfc3161 = bundle
            .verification_material
            .timestamp_verification_data
            .as_ref()
            .and_then(|td| td.rfc3161_timestamps.as_ref())
            .map(|ts| !ts.is_empty())
            .unwrap_or(false);

        let has_tlog = bundle
            .verification_material
            .tlog_entries
            .as_ref()
            .map(|entries| !entries.is_empty())
            .unwrap_or(false);

        // Validate we have a TSA chain for RFC 3161 path
        if has_rfc3161 && tsa_cert_chain.is_none() {
            return Err(error::TimestampError::MissingTSAChain.into());
        }

        // Get signing time from appropriate mechanism
        let signing_time = match (has_rfc3161, has_tlog) {
            (true, true) => return Err(error::TimestampError::BothTimestampMechanisms.into()),
            (false, false) => return Err(error::TimestampError::NoTimestamp.into()),
            (true, false) => get_rfc3161_time(bundle)?,
            (false, true) => get_integrated_time(
                &bundle.verification_material.tlog_entries.as_ref().unwrap()[0],
            )?,
        };

        // Step 3: Verify certificate chain and get hashes
        let (chain, certificate_hashes) = verify_certificate_chain(bundle, trust_bundle)?;

        // Step 3b: Verify signing time is within certificate validity period
        let leaf_cert = parse_der_certificate(&chain.leaf)
            .map_err(|e| VerificationError::InvalidBundleFormat(e.to_string()))?;
        verify_signing_time_in_validity(&signing_time, &leaf_cert)?;

        // Step 4: Verify DSSE signature
        verify_dsse_signature(&bundle.dsse_envelope, &chain)?;

        // Step 5: Verify timestamp mechanism (RFC 3161 OR Rekor, mutually exclusive)
        if has_rfc3161 {
            // RFC 3161 path: verify TSA chain and timestamp signature
            let tsa_chain = {
                let timestamp_data = &bundle
                    .verification_material
                    .timestamp_verification_data
                    .as_ref()
                    .unwrap() // Safe: checked by has_rfc3161
                    .rfc3161_timestamps
                    .as_ref()
                    .unwrap()[0]; // Safe: has_rfc3161 validates non-empty

                // Decode and parse RFC 3161 timestamp
                let timestamp_der = BASE64
                    .decode(&timestamp_data.signed_timestamp)
                    .map_err(|e| {
                        VerificationError::InvalidBundleFormat(format!(
                            "Failed to decode timestamp: {}",
                            e
                        ))
                    })?;

                let parsed_timestamp = parse_rfc3161_timestamp(&timestamp_der)?;

                // Try to extract embedded certificates (takes precedence)
                if let Some(embedded_certs) = parsed_timestamp.certificates {
                    if !embedded_certs.is_empty() {
                        // Embedded certs found - use them
                        let embedded_chain = certs_to_chain(embedded_certs).map_err(|e| {
                            error::TimestampError::InvalidTSACertificate(format!(
                                "Failed to parse embedded TSA certs: {}",
                                e
                            ))
                        })?;
                        embedded_chain
                    } else {
                        // Empty embedded cert list - fall back to user-provided
                        tsa_cert_chain.cloned().unwrap()
                    }
                } else {
                    // No embedded certs field at all - use user-provided
                    tsa_cert_chain.cloned().unwrap()
                }
            };

            // Verify TSA certificate chain and EKU
            verify_tsa_certificate_chain(&tsa_chain)?;

            // Verify RFC 3161 timestamp token (message imprint + PKCS7 signature)
            let signature_b64 = &bundle.dsse_envelope.signatures[0].sig;
            verify_rfc3161_timestamp(bundle, signature_b64, &tsa_chain)?;
        } else {
            // Rekor path: verify transparency log
            verify_transparency_log(bundle)?;
        }

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
