use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use super::certificate::OidcIdentity;
use alloy_sol_types::{sol, SolValue};

// Define Solidity-compatible struct for ABI encoding
sol! {
    /// Solidity-compatible representation of VerificationResult
    ///
    /// Note: This struct is used for ABI encoding only. The actual serialization format is:
    /// [8 bytes: uint64 signing_time] || abi.encode(VerificationResultEncoded)
    #[derive(Debug, PartialEq)]
    struct VerificationResultEncoded {
        bytes32[] certificateHashes;  // [leaf, ...intermediates, root]
        bytes subjectDigest;
        string oidcIssuer;
        string oidcSubject;
        string oidcWorkflowRef;
        string oidcRepository;
        string oidcEventName;
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationResult {
    pub certificate_hashes: CertificateChainHashes,
    pub signing_time: DateTime<Utc>,
    pub subject_digest: Vec<u8>,
    pub oidc_identity: Option<OidcIdentity>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CertificateChainHashes {
    pub leaf: [u8; 32],
    pub intermediates: Vec<[u8; 32]>,
    pub root: [u8; 32],
}

impl CertificateChainHashes {
    pub fn as_tuple(&self) -> ([u8; 32], Vec<[u8; 32]>, [u8; 32]) {
        (self.leaf, self.intermediates.clone(), self.root)
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct VerificationOptions {
    /// Optional expected digest to verify against the subject digest in the attestation
    pub expected_digest: Option<Vec<u8>>,

    /// Allow certificates without valid SCT (Signed Certificate Timestamp) verification
    ///
    /// **NOT YET IMPLEMENTED** - This field is reserved for future Certificate Transparency verification.
    ///
    /// When implemented, this will control whether to require valid SCTs embedded in the
    /// Fulcio certificate. SCTs prove that the certificate was logged in Certificate
    /// Transparency logs, providing an additional audit trail.
    ///
    /// - `false` (recommended): Require valid SCT verification (stricter security policy)
    /// - `true`: Skip SCT verification (more permissive, current behavior)
    pub allow_insecure_sct: bool,

    /// Optional expected OIDC issuer (e.g., "https://token.actions.githubusercontent.com")
    pub expected_issuer: Option<String>,

    /// Optional expected OIDC subject (e.g., "repo:owner/repo:ref:refs/heads/main")
    pub expected_subject: Option<String>,
}

impl VerificationResult {
    /// Serialize the VerificationResult into a Solidity-compatible byte array
    ///
    /// The serialization format is:
    /// ```text
    /// [8 bytes: uint64 signing_time (big-endian)]
    /// ||
    /// abi.encode(
    ///   bytes32[] certificateHashes,  // [leaf, ...intermediates, root]
    ///   bytes subjectDigest,
    ///   string oidcIssuer,
    ///   string oidcSubject,
    ///   string oidcWorkflowRef,
    ///   string oidcRepository,
    ///   string oidcEventName
    /// )
    /// ```
    ///
    /// # Returns
    ///
    /// A byte vector containing the serialized data suitable for Solidity smart contract consumption.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let result = VerificationResult { /* ... */ };
    /// let encoded = result.as_slice();
    /// // Pass encoded to Solidity contract
    /// ```
    pub fn as_slice(&self) -> Vec<u8> {
        // Convert signing_time to u64 Unix timestamp (big-endian)
        let timestamp = self.signing_time.timestamp() as u64;
        let timestamp_bytes = timestamp.to_be_bytes();

        // Build certificate hashes array: [leaf, ...intermediates, root]
        let mut cert_hashes = Vec::with_capacity(2 + self.certificate_hashes.intermediates.len());
        cert_hashes.push(self.certificate_hashes.leaf.into());
        for intermediate in &self.certificate_hashes.intermediates {
            cert_hashes.push((*intermediate).into());
        }
        cert_hashes.push(self.certificate_hashes.root.into());

        // Extract OIDC fields, using empty strings for None
        let (issuer, subject, workflow_ref, repository, event_name) = if let Some(ref oidc) = self.oidc_identity {
            (
                oidc.issuer.clone().unwrap_or_default(),
                oidc.subject.clone().unwrap_or_default(),
                oidc.workflow_ref.clone().unwrap_or_default(),
                oidc.repository.clone().unwrap_or_default(),
                oidc.event_name.clone().unwrap_or_default(),
            )
        } else {
            (String::new(), String::new(), String::new(), String::new(), String::new())
        };

        // Create the Solidity-compatible struct
        let encoded_struct = VerificationResultEncoded {
            certificateHashes: cert_hashes,
            subjectDigest: self.subject_digest.clone().into(),
            oidcIssuer: issuer,
            oidcSubject: subject,
            oidcWorkflowRef: workflow_ref,
            oidcRepository: repository,
            oidcEventName: event_name,
        };

        // Encode using standard ABI encoding
        let abi_encoded = encoded_struct.abi_encode();

        // Prepend timestamp bytes
        let mut result = Vec::with_capacity(8 + abi_encoded.len());
        result.extend_from_slice(&timestamp_bytes);
        result.extend_from_slice(&abi_encoded);

        result
    }

    /// Deserialize a VerificationResult from a Solidity-compatible byte array
    ///
    /// This is the inverse operation of `as_slice()`. It parses the byte array
    /// and reconstructs the VerificationResult.
    ///
    /// # Arguments
    ///
    /// * `data` - The byte slice to deserialize
    ///
    /// # Returns
    ///
    /// Returns `Ok(VerificationResult)` if deserialization succeeds, or an error message
    /// describing what went wrong.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The data is shorter than 8 bytes (minimum size for timestamp)
    /// - ABI decoding fails
    /// - The certificate hashes array has fewer than 2 elements
    ///
    /// # Example
    ///
    /// ```ignore
    /// let encoded_data = result.as_slice();
    /// let decoded = VerificationResult::from_slice(&encoded_data).unwrap();
    /// assert_eq!(result, decoded);
    /// ```
    pub fn from_slice(data: &[u8]) -> Result<Self, String> {
        // Need at least 8 bytes for timestamp
        if data.len() < 8 {
            return Err(format!("Data too short: expected at least 8 bytes, got {}", data.len()));
        }

        // Extract timestamp (first 8 bytes, big-endian)
        let timestamp_bytes: [u8; 8] = data[0..8].try_into().unwrap();
        let timestamp = u64::from_be_bytes(timestamp_bytes);

        // Decode the remaining ABI-encoded data
        let abi_data = &data[8..];
        let decoded = VerificationResultEncoded::abi_decode(abi_data)
            .map_err(|e| format!("Failed to ABI decode: {}", e))?;

        // Extract certificate hashes: first is leaf, last is root, middle are intermediates
        if decoded.certificateHashes.len() < 2 {
            return Err(format!(
                "Certificate hashes array must have at least 2 elements (leaf and root), got {}",
                decoded.certificateHashes.len()
            ));
        }

        let leaf = decoded.certificateHashes[0].0;
        let root = decoded.certificateHashes[decoded.certificateHashes.len() - 1].0;
        let intermediates: Vec<[u8; 32]> = decoded.certificateHashes[1..decoded.certificateHashes.len() - 1]
            .iter()
            .map(|h| h.0)
            .collect();

        // Reconstruct OIDC identity (only if any field is non-empty)
        let oidc_identity = if decoded.oidcIssuer.is_empty()
            && decoded.oidcSubject.is_empty()
            && decoded.oidcWorkflowRef.is_empty()
            && decoded.oidcRepository.is_empty()
            && decoded.oidcEventName.is_empty()
        {
            None
        } else {
            Some(OidcIdentity {
                issuer: if decoded.oidcIssuer.is_empty() { None } else { Some(decoded.oidcIssuer) },
                subject: if decoded.oidcSubject.is_empty() { None } else { Some(decoded.oidcSubject) },
                workflow_ref: if decoded.oidcWorkflowRef.is_empty() { None } else { Some(decoded.oidcWorkflowRef) },
                repository: if decoded.oidcRepository.is_empty() { None } else { Some(decoded.oidcRepository) },
                event_name: if decoded.oidcEventName.is_empty() { None } else { Some(decoded.oidcEventName) },
            })
        };

        // Convert timestamp to DateTime<Utc>
        let signing_time = DateTime::from_timestamp(timestamp as i64, 0)
            .ok_or_else(|| format!("Invalid timestamp: {}", timestamp))?;

        Ok(VerificationResult {
            certificate_hashes: CertificateChainHashes {
                leaf,
                intermediates,
                root,
            },
            signing_time,
            subject_digest: decoded.subjectDigest.to_vec(),
            oidc_identity,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_as_slice_from_slice_roundtrip_with_full_data() {
        // Create a test VerificationResult with all fields populated
        let original = VerificationResult {
            certificate_hashes: CertificateChainHashes {
                leaf: [1u8; 32],
                intermediates: vec![[2u8; 32], [3u8; 32]],
                root: [4u8; 32],
            },
            signing_time: DateTime::from_timestamp(1700000000, 0).unwrap(),
            subject_digest: vec![5u8; 32], // SHA256 digest
            oidc_identity: Some(OidcIdentity {
                issuer: Some("https://token.actions.githubusercontent.com".to_string()),
                subject: Some("repo:owner/repo:ref:refs/heads/main".to_string()),
                workflow_ref: Some("owner/repo/.github/workflows/ci.yml@refs/heads/main".to_string()),
                repository: Some("owner/repo".to_string()),
                event_name: Some("push".to_string()),
            }),
        };

        // Serialize and deserialize
        let encoded = original.as_slice();
        let decoded = VerificationResult::from_slice(&encoded).expect("Failed to decode");

        // Verify all fields match
        assert_eq!(original.certificate_hashes.leaf, decoded.certificate_hashes.leaf);
        assert_eq!(original.certificate_hashes.intermediates, decoded.certificate_hashes.intermediates);
        assert_eq!(original.certificate_hashes.root, decoded.certificate_hashes.root);
        assert_eq!(original.signing_time.timestamp(), decoded.signing_time.timestamp());
        assert_eq!(original.subject_digest, decoded.subject_digest);
        assert_eq!(original.oidc_identity, decoded.oidc_identity);
    }

    #[test]
    fn test_as_slice_from_slice_roundtrip_no_intermediates() {
        // Test with no intermediate certificates
        let original = VerificationResult {
            certificate_hashes: CertificateChainHashes {
                leaf: [10u8; 32],
                intermediates: vec![],
                root: [20u8; 32],
            },
            signing_time: DateTime::from_timestamp(1600000000, 0).unwrap(),
            subject_digest: vec![30u8; 32],
            oidc_identity: None,
        };

        let encoded = original.as_slice();
        let decoded = VerificationResult::from_slice(&encoded).expect("Failed to decode");

        assert_eq!(original.certificate_hashes.leaf, decoded.certificate_hashes.leaf);
        assert_eq!(original.certificate_hashes.intermediates.len(), 0);
        assert_eq!(decoded.certificate_hashes.intermediates.len(), 0);
        assert_eq!(original.certificate_hashes.root, decoded.certificate_hashes.root);
        assert_eq!(original.oidc_identity, decoded.oidc_identity);
    }

    #[test]
    fn test_as_slice_from_slice_roundtrip_partial_oidc() {
        // Test with partial OIDC identity (some fields None)
        let original = VerificationResult {
            certificate_hashes: CertificateChainHashes {
                leaf: [100u8; 32],
                intermediates: vec![[101u8; 32]],
                root: [102u8; 32],
            },
            signing_time: DateTime::from_timestamp(1650000000, 0).unwrap(),
            subject_digest: vec![103u8; 32],
            oidc_identity: Some(OidcIdentity {
                issuer: Some("https://example.com".to_string()),
                subject: Some("test-subject".to_string()),
                workflow_ref: None,
                repository: None,
                event_name: None,
            }),
        };

        let encoded = original.as_slice();
        let decoded = VerificationResult::from_slice(&encoded).expect("Failed to decode");

        assert_eq!(original.oidc_identity, decoded.oidc_identity);
    }

    #[test]
    fn test_from_slice_error_too_short() {
        // Test with data that's too short (less than 8 bytes)
        let short_data = vec![1u8, 2, 3, 4];
        let result = VerificationResult::from_slice(&short_data);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Data too short"));
    }

    #[test]
    fn test_from_slice_error_invalid_abi_encoding() {
        // Test with valid timestamp but invalid ABI encoding
        let mut invalid_data = vec![0u8; 8]; // Valid timestamp bytes
        invalid_data.extend_from_slice(&[255u8; 32]); // Invalid ABI data
        let result = VerificationResult::from_slice(&invalid_data);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Failed to ABI decode"));
    }

    #[test]
    fn test_as_slice_format() {
        // Verify the format: first 8 bytes should be timestamp
        let original = VerificationResult {
            certificate_hashes: CertificateChainHashes {
                leaf: [1u8; 32],
                intermediates: vec![],
                root: [2u8; 32],
            },
            signing_time: DateTime::from_timestamp(1700000000, 0).unwrap(),
            subject_digest: vec![3u8; 32],
            oidc_identity: None,
        };

        let encoded = original.as_slice();

        // First 8 bytes should be the timestamp in big-endian
        let timestamp_bytes: [u8; 8] = encoded[0..8].try_into().unwrap();
        let timestamp = u64::from_be_bytes(timestamp_bytes);
        assert_eq!(timestamp, 1700000000);

        // Remaining bytes should be ABI-encoded
        assert!(encoded.len() > 8);
    }

    #[test]
    fn test_certificate_hashes_ordering() {
        // Verify that certificate hashes are encoded in the correct order
        let original = VerificationResult {
            certificate_hashes: CertificateChainHashes {
                leaf: [11u8; 32],
                intermediates: vec![[22u8; 32], [33u8; 32], [44u8; 32]],
                root: [55u8; 32],
            },
            signing_time: DateTime::from_timestamp(1700000000, 0).unwrap(),
            subject_digest: vec![66u8; 32],
            oidc_identity: None,
        };

        let encoded = original.as_slice();
        let decoded = VerificationResult::from_slice(&encoded).expect("Failed to decode");

        // Verify order: leaf, intermediates (in order), root
        assert_eq!(decoded.certificate_hashes.leaf, [11u8; 32]);
        assert_eq!(decoded.certificate_hashes.intermediates, vec![[22u8; 32], [33u8; 32], [44u8; 32]]);
        assert_eq!(decoded.certificate_hashes.root, [55u8; 32]);
    }
}
