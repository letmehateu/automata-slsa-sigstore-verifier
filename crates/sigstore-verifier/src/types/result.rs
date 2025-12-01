use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use super::certificate::OidcIdentity;
use alloy_sol_types::{sol, SolValue};

// =============================================================================
// Solidity ABI Encoding Format
// =============================================================================
//
// The serialized VerificationResult has the following binary format:
//
// ┌─────────────────────────────────────────────────────────────────────────────┐
// │ [8 bytes]  signing_time          - uint64 big-endian Unix timestamp         │
// │ [1 byte]   timestamp_proof_type  - 0=None, 1=RFC3161, 2=Rekor               │
// │ [N bytes]  ABI-encoded VerificationResultEncoded struct                     │
// └─────────────────────────────────────────────────────────────────────────────┘
//
// Field descriptions:
//
// - certificateHashes: SHA256 hashes of the signing certificate chain
//   Format: [leaf_hash, ...intermediate_hashes, root_hash]
//
// - subjectDigest: The artifact digest from the attestation (typically SHA256)
//
// - subjectDigestAlgorithm: Hash algorithm for subjectDigest
//   0 = Unknown, 1 = SHA256, 2 = SHA384
//
// - oidcIssuer: OIDC token issuer (e.g., "https://token.actions.githubusercontent.com")
//
// - oidcSubject: OIDC subject claim
//
// - oidcWorkflowRef: GitHub workflow reference (GitHub Actions specific)
//
// - oidcRepository: Source repository (GitHub Actions specific)
//
// - oidcEventName: Trigger event name (GitHub Actions specific)
//
// - tsaChainHashes: For RFC 3161 timestamps, SHA256 hashes of TSA certificate chain
//   Format: [leaf_hash, ...intermediate_hashes, root_hash]. Empty for Rekor.
//
// - messageImprintAlgorithm: For RFC 3161, the hash algorithm used in the timestamp
//   0 = Unknown, 1 = SHA256, 2 = SHA384. Set to 0 for Rekor.
//
// - messageImprint: For RFC 3161, the hash of the DSSE signature that was timestamped.
//   This proves the timestamp was generated for this specific signature. Empty for Rekor.
//
// - rekorLogId: For Rekor, the SHA256 hash of Rekor's public key (identifies the log instance).
//   Zero bytes for RFC 3161.
//
// - rekorLogIndex: For Rekor, the tree leaf index (for Merkle proof verification).
//   Set to 0 for RFC 3161.
//
// - rekorEntryIndex: For Rekor, the entry index (for API queries to fetch the full entry).
//   Set to 0 for RFC 3161.
//
// =============================================================================

sol! {
    #[derive(Debug, PartialEq)]
    struct VerificationResultEncoded {
        bytes32[] certificateHashes;
        bytes subjectDigest;
        uint8 subjectDigestAlgorithm;
        string oidcIssuer;
        string oidcSubject;
        string oidcWorkflowRef;
        string oidcRepository;
        string oidcEventName;
        bytes32[] tsaChainHashes;
        uint8 messageImprintAlgorithm;
        bytes messageImprint;
        bytes32 rekorLogId;
        uint64 rekorLogIndex;
        uint64 rekorEntryIndex;
    }
}

/// Hash algorithm identifier for Solidity encoding
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum DigestAlgorithm {
    Unknown = 0,
    Sha256 = 1,
    Sha384 = 2,
}

impl DigestAlgorithm {
    pub fn from_u8(value: u8) -> Self {
        match value {
            1 => DigestAlgorithm::Sha256,
            2 => DigestAlgorithm::Sha384,
            _ => DigestAlgorithm::Unknown,
        }
    }
}

/// Timestamp proof type identifier
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum TimestampProofType {
    None = 0,
    Rfc3161 = 1,
    Rekor = 2,
}

impl TimestampProofType {
    pub fn from_u8(value: u8) -> Self {
        match value {
            1 => TimestampProofType::Rfc3161,
            2 => TimestampProofType::Rekor,
            _ => TimestampProofType::None,
        }
    }
}

/// Timestamp proof data - proves when the signature was created
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TimestampProof {
    /// No timestamp proof available
    None,

    /// RFC 3161 Timestamp Authority proof
    Rfc3161 {
        /// SHA256 hashes of TSA certificate chain [leaf, ...intermediates, root]
        tsa_chain_hashes: CertificateChainHashes,
        /// Hash algorithm used for the message imprint
        message_imprint_algorithm: DigestAlgorithm,
        /// The message imprint (hash of the DSSE signature)
        message_imprint: Vec<u8>,
    },

    /// Sigstore Rekor transparency log proof
    Rekor {
        /// SHA256 of Rekor's public key (identifies the log instance)
        log_id: [u8; 32],
        /// Tree leaf index (for Merkle proof verification against checkpoint)
        log_index: u64,
        /// Entry index (for API queries to fetch the full entry)
        entry_index: u64,
    },
}

impl Default for TimestampProof {
    fn default() -> Self {
        TimestampProof::None
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationResult {
    pub certificate_hashes: CertificateChainHashes,
    pub signing_time: DateTime<Utc>,
    pub subject_digest: Vec<u8>,
    pub subject_digest_algorithm: DigestAlgorithm,
    pub oidc_identity: Option<OidcIdentity>,
    pub timestamp_proof: TimestampProof,
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

    /// Optional expected OIDC issuer (e.g., "https://token.actions.githubusercontent.com")
    pub expected_issuer: Option<String>,

    /// Optional expected OIDC subject (e.g., "repo:owner/repo:ref:refs/heads/main")
    pub expected_subject: Option<String>,
}

impl VerificationResult {
    /// Serialize the VerificationResult into a Solidity-compatible byte array
    ///
    /// See the module-level documentation for the complete binary format specification.
    ///
    /// # Returns
    ///
    /// A byte vector containing the serialized data suitable for Solidity smart contract consumption.
    pub fn as_slice(&self) -> Vec<u8> {
        // Convert signing_time to u64 Unix timestamp (big-endian)
        let timestamp = self.signing_time.timestamp() as u64;
        let timestamp_bytes = timestamp.to_be_bytes();

        // Determine timestamp proof type
        let proof_type: u8 = match &self.timestamp_proof {
            TimestampProof::None => TimestampProofType::None as u8,
            TimestampProof::Rfc3161 { .. } => TimestampProofType::Rfc3161 as u8,
            TimestampProof::Rekor { .. } => TimestampProofType::Rekor as u8,
        };

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

        // Extract timestamp proof fields based on type
        let (tsa_chain_hashes, message_imprint_algorithm, message_imprint, rekor_log_id, rekor_log_index, rekor_entry_index) =
            match &self.timestamp_proof {
                TimestampProof::None => {
                    (vec![], 0u8, vec![], [0u8; 32], 0u64, 0u64)
                }
                TimestampProof::Rfc3161 {
                    tsa_chain_hashes,
                    message_imprint_algorithm,
                    message_imprint,
                } => {
                    let mut hashes = Vec::with_capacity(2 + tsa_chain_hashes.intermediates.len());
                    hashes.push(tsa_chain_hashes.leaf.into());
                    for intermediate in &tsa_chain_hashes.intermediates {
                        hashes.push((*intermediate).into());
                    }
                    hashes.push(tsa_chain_hashes.root.into());
                    (
                        hashes,
                        *message_imprint_algorithm as u8,
                        message_imprint.clone(),
                        [0u8; 32],
                        0u64,
                        0u64,
                    )
                }
                TimestampProof::Rekor { log_id, log_index, entry_index } => {
                    (vec![], 0u8, vec![], *log_id, *log_index, *entry_index)
                }
            };

        // Create the Solidity-compatible struct
        let encoded_struct = VerificationResultEncoded {
            certificateHashes: cert_hashes,
            subjectDigest: self.subject_digest.clone().into(),
            subjectDigestAlgorithm: self.subject_digest_algorithm as u8,
            oidcIssuer: issuer,
            oidcSubject: subject,
            oidcWorkflowRef: workflow_ref,
            oidcRepository: repository,
            oidcEventName: event_name,
            tsaChainHashes: tsa_chain_hashes,
            messageImprintAlgorithm: message_imprint_algorithm,
            messageImprint: message_imprint.into(),
            rekorLogId: rekor_log_id.into(),
            rekorLogIndex: rekor_log_index,
            rekorEntryIndex: rekor_entry_index,
        };

        // Encode using standard ABI encoding
        let abi_encoded = encoded_struct.abi_encode();

        // Build result: [timestamp (8 bytes)] || [proof_type (1 byte)] || [ABI-encoded data]
        let mut result = Vec::with_capacity(9 + abi_encoded.len());
        result.extend_from_slice(&timestamp_bytes);
        result.push(proof_type);
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
    /// - The data is shorter than 9 bytes (minimum size for timestamp + proof type)
    /// - ABI decoding fails
    /// - The certificate hashes array has fewer than 2 elements
    pub fn from_slice(data: &[u8]) -> Result<Self, String> {
        // Need at least 9 bytes for timestamp (8) + proof type (1)
        if data.len() < 9 {
            return Err(format!("Data too short: expected at least 9 bytes, got {}", data.len()));
        }

        // Extract timestamp (first 8 bytes, big-endian)
        let timestamp_bytes: [u8; 8] = data[0..8].try_into().unwrap();
        let timestamp = u64::from_be_bytes(timestamp_bytes);

        // Extract proof type (byte 9)
        let proof_type = TimestampProofType::from_u8(data[8]);

        // Decode the remaining ABI-encoded data
        let abi_data = &data[9..];
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

        // Reconstruct timestamp proof based on type
        let timestamp_proof = match proof_type {
            TimestampProofType::None => TimestampProof::None,
            TimestampProofType::Rfc3161 => {
                // Extract TSA chain hashes
                if decoded.tsaChainHashes.len() < 2 {
                    return Err(format!(
                        "TSA chain hashes must have at least 2 elements for RFC 3161, got {}",
                        decoded.tsaChainHashes.len()
                    ));
                }
                let tsa_leaf = decoded.tsaChainHashes[0].0;
                let tsa_root = decoded.tsaChainHashes[decoded.tsaChainHashes.len() - 1].0;
                let tsa_intermediates: Vec<[u8; 32]> = decoded.tsaChainHashes[1..decoded.tsaChainHashes.len() - 1]
                    .iter()
                    .map(|h| h.0)
                    .collect();

                TimestampProof::Rfc3161 {
                    tsa_chain_hashes: CertificateChainHashes {
                        leaf: tsa_leaf,
                        intermediates: tsa_intermediates,
                        root: tsa_root,
                    },
                    message_imprint_algorithm: DigestAlgorithm::from_u8(decoded.messageImprintAlgorithm),
                    message_imprint: decoded.messageImprint.to_vec(),
                }
            }
            TimestampProofType::Rekor => {
                TimestampProof::Rekor {
                    log_id: decoded.rekorLogId.0,
                    log_index: decoded.rekorLogIndex,
                    entry_index: decoded.rekorEntryIndex,
                }
            }
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
            subject_digest_algorithm: DigestAlgorithm::from_u8(decoded.subjectDigestAlgorithm),
            oidc_identity,
            timestamp_proof,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_as_slice_from_slice_roundtrip_with_rfc3161() {
        // Create a test VerificationResult with RFC 3161 timestamp proof
        let original = VerificationResult {
            certificate_hashes: CertificateChainHashes {
                leaf: [1u8; 32],
                intermediates: vec![[2u8; 32], [3u8; 32]],
                root: [4u8; 32],
            },
            signing_time: DateTime::from_timestamp(1700000000, 0).unwrap(),
            subject_digest: vec![5u8; 32],
            subject_digest_algorithm: DigestAlgorithm::Sha256,
            oidc_identity: Some(OidcIdentity {
                issuer: Some("https://token.actions.githubusercontent.com".to_string()),
                subject: Some("repo:owner/repo:ref:refs/heads/main".to_string()),
                workflow_ref: Some("owner/repo/.github/workflows/ci.yml@refs/heads/main".to_string()),
                repository: Some("owner/repo".to_string()),
                event_name: Some("push".to_string()),
            }),
            timestamp_proof: TimestampProof::Rfc3161 {
                tsa_chain_hashes: CertificateChainHashes {
                    leaf: [10u8; 32],
                    intermediates: vec![[11u8; 32]],
                    root: [12u8; 32],
                },
                message_imprint_algorithm: DigestAlgorithm::Sha256,
                message_imprint: vec![13u8; 32],
            },
        };

        let encoded = original.as_slice();
        let decoded = VerificationResult::from_slice(&encoded).expect("Failed to decode");

        // Verify all fields match
        assert_eq!(original.certificate_hashes.leaf, decoded.certificate_hashes.leaf);
        assert_eq!(original.certificate_hashes.intermediates, decoded.certificate_hashes.intermediates);
        assert_eq!(original.certificate_hashes.root, decoded.certificate_hashes.root);
        assert_eq!(original.signing_time.timestamp(), decoded.signing_time.timestamp());
        assert_eq!(original.subject_digest, decoded.subject_digest);
        assert_eq!(original.subject_digest_algorithm, decoded.subject_digest_algorithm);
        assert_eq!(original.oidc_identity, decoded.oidc_identity);

        // Verify RFC 3161 timestamp proof
        match (&original.timestamp_proof, &decoded.timestamp_proof) {
            (
                TimestampProof::Rfc3161 { tsa_chain_hashes: orig_tsa, message_imprint_algorithm: orig_alg, message_imprint: orig_imprint },
                TimestampProof::Rfc3161 { tsa_chain_hashes: dec_tsa, message_imprint_algorithm: dec_alg, message_imprint: dec_imprint },
            ) => {
                assert_eq!(orig_tsa.leaf, dec_tsa.leaf);
                assert_eq!(orig_tsa.intermediates, dec_tsa.intermediates);
                assert_eq!(orig_tsa.root, dec_tsa.root);
                assert_eq!(orig_alg, dec_alg);
                assert_eq!(orig_imprint, dec_imprint);
            }
            _ => panic!("Expected RFC 3161 timestamp proof"),
        }
    }

    #[test]
    fn test_as_slice_from_slice_roundtrip_with_rekor() {
        // Create a test VerificationResult with Rekor timestamp proof
        let original = VerificationResult {
            certificate_hashes: CertificateChainHashes {
                leaf: [1u8; 32],
                intermediates: vec![],
                root: [2u8; 32],
            },
            signing_time: DateTime::from_timestamp(1700000000, 0).unwrap(),
            subject_digest: vec![3u8; 32],
            subject_digest_algorithm: DigestAlgorithm::Sha256,
            oidc_identity: None,
            timestamp_proof: TimestampProof::Rekor {
                log_id: [20u8; 32],
                log_index: 12345678,
                entry_index: 87654321,
            },
        };

        let encoded = original.as_slice();
        let decoded = VerificationResult::from_slice(&encoded).expect("Failed to decode");

        // Verify Rekor timestamp proof
        match (&original.timestamp_proof, &decoded.timestamp_proof) {
            (
                TimestampProof::Rekor { log_id: orig_id, log_index: orig_idx, entry_index: orig_entry },
                TimestampProof::Rekor { log_id: dec_id, log_index: dec_idx, entry_index: dec_entry },
            ) => {
                assert_eq!(orig_id, dec_id);
                assert_eq!(orig_idx, dec_idx);
                assert_eq!(orig_entry, dec_entry);
            }
            _ => panic!("Expected Rekor timestamp proof"),
        }
    }

    #[test]
    fn test_as_slice_from_slice_roundtrip_no_timestamp_proof() {
        // Test with no timestamp proof
        let original = VerificationResult {
            certificate_hashes: CertificateChainHashes {
                leaf: [10u8; 32],
                intermediates: vec![],
                root: [20u8; 32],
            },
            signing_time: DateTime::from_timestamp(1600000000, 0).unwrap(),
            subject_digest: vec![30u8; 32],
            subject_digest_algorithm: DigestAlgorithm::Sha384,
            oidc_identity: None,
            timestamp_proof: TimestampProof::None,
        };

        let encoded = original.as_slice();
        let decoded = VerificationResult::from_slice(&encoded).expect("Failed to decode");

        assert_eq!(original.certificate_hashes.leaf, decoded.certificate_hashes.leaf);
        assert_eq!(original.certificate_hashes.intermediates.len(), 0);
        assert_eq!(decoded.certificate_hashes.intermediates.len(), 0);
        assert_eq!(original.certificate_hashes.root, decoded.certificate_hashes.root);
        assert_eq!(original.subject_digest_algorithm, decoded.subject_digest_algorithm);
        assert!(matches!(decoded.timestamp_proof, TimestampProof::None));
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
            subject_digest_algorithm: DigestAlgorithm::Sha256,
            oidc_identity: Some(OidcIdentity {
                issuer: Some("https://example.com".to_string()),
                subject: Some("test-subject".to_string()),
                workflow_ref: None,
                repository: None,
                event_name: None,
            }),
            timestamp_proof: TimestampProof::None,
        };

        let encoded = original.as_slice();
        let decoded = VerificationResult::from_slice(&encoded).expect("Failed to decode");

        assert_eq!(original.oidc_identity, decoded.oidc_identity);
    }

    #[test]
    fn test_from_slice_error_too_short() {
        // Test with data that's too short (less than 9 bytes)
        let short_data = vec![1u8, 2, 3, 4];
        let result = VerificationResult::from_slice(&short_data);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Data too short"));
    }

    #[test]
    fn test_from_slice_error_invalid_abi_encoding() {
        // Test with valid timestamp + proof type but invalid ABI encoding
        let mut invalid_data = vec![0u8; 9]; // Valid timestamp (8) + proof type (1)
        invalid_data.extend_from_slice(&[255u8; 32]); // Invalid ABI data
        let result = VerificationResult::from_slice(&invalid_data);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Failed to ABI decode"));
    }

    #[test]
    fn test_as_slice_format() {
        // Verify the format: first 8 bytes should be timestamp, byte 9 is proof type
        let original = VerificationResult {
            certificate_hashes: CertificateChainHashes {
                leaf: [1u8; 32],
                intermediates: vec![],
                root: [2u8; 32],
            },
            signing_time: DateTime::from_timestamp(1700000000, 0).unwrap(),
            subject_digest: vec![3u8; 32],
            subject_digest_algorithm: DigestAlgorithm::Sha256,
            oidc_identity: None,
            timestamp_proof: TimestampProof::Rekor {
                log_id: [4u8; 32],
                log_index: 999,
                entry_index: 1000,
            },
        };

        let encoded = original.as_slice();

        // First 8 bytes should be the timestamp in big-endian
        let timestamp_bytes: [u8; 8] = encoded[0..8].try_into().unwrap();
        let timestamp = u64::from_be_bytes(timestamp_bytes);
        assert_eq!(timestamp, 1700000000);

        // Byte 9 should be proof type (2 = Rekor)
        assert_eq!(encoded[8], TimestampProofType::Rekor as u8);

        // Remaining bytes should be ABI-encoded
        assert!(encoded.len() > 9);
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
            subject_digest_algorithm: DigestAlgorithm::Sha256,
            oidc_identity: None,
            timestamp_proof: TimestampProof::None,
        };

        let encoded = original.as_slice();
        let decoded = VerificationResult::from_slice(&encoded).expect("Failed to decode");

        // Verify order: leaf, intermediates (in order), root
        assert_eq!(decoded.certificate_hashes.leaf, [11u8; 32]);
        assert_eq!(decoded.certificate_hashes.intermediates, vec![[22u8; 32], [33u8; 32], [44u8; 32]]);
        assert_eq!(decoded.certificate_hashes.root, [55u8; 32]);
    }

    #[test]
    fn test_digest_algorithm_roundtrip() {
        // Test all digest algorithm values
        assert_eq!(DigestAlgorithm::from_u8(0), DigestAlgorithm::Unknown);
        assert_eq!(DigestAlgorithm::from_u8(1), DigestAlgorithm::Sha256);
        assert_eq!(DigestAlgorithm::from_u8(2), DigestAlgorithm::Sha384);
        assert_eq!(DigestAlgorithm::from_u8(255), DigestAlgorithm::Unknown);
    }

    #[test]
    fn test_timestamp_proof_type_roundtrip() {
        // Test all timestamp proof type values
        assert_eq!(TimestampProofType::from_u8(0), TimestampProofType::None);
        assert_eq!(TimestampProofType::from_u8(1), TimestampProofType::Rfc3161);
        assert_eq!(TimestampProofType::from_u8(2), TimestampProofType::Rekor);
        assert_eq!(TimestampProofType::from_u8(255), TimestampProofType::None);
    }
}
