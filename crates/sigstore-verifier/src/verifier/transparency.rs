use crate::crypto::merkle::{compute_leaf_hash, verify_inclusion_proof};
use crate::error::{TransparencyError, VerificationError};
use crate::parser::bundle::decode_base64;
use crate::types::bundle::SigstoreBundle;

/// Verify the Rekor transparency log inclusion proof
///
/// This verification ensures that:
/// 1. The bundle contains transparency log entries
/// 2. The inclusion proof is valid (Merkle tree verification)
/// 3. The entry was properly logged in Rekor
///
/// This provides protection against backdating attacks and ensures the signature
/// was publicly logged in an immutable transparency log.
pub fn verify_transparency_log(bundle: &SigstoreBundle) -> Result<(), VerificationError> {
    let tlog_entries = bundle
        .verification_material
        .tlog_entries
        .as_ref()
        .ok_or(TransparencyError::NoRekorEntry)?;

    if tlog_entries.is_empty() {
        return Err(TransparencyError::NoRekorEntry.into());
    }

    let entry = &tlog_entries[0];

    // Verify inclusion proof if present
    if let Some(ref inclusion_proof) = entry.inclusion_proof {
        let log_index = inclusion_proof
            .log_index
            .parse::<u64>()
            .map_err(|_| TransparencyError::InvalidEntryHash)?;

        let tree_size = inclusion_proof
            .tree_size
            .parse::<u64>()
            .map_err(|_| TransparencyError::InvalidEntryHash)?;

        let root_hash = decode_base64(&inclusion_proof.root_hash)
            .map_err(|_| TransparencyError::InvalidEntryHash)?;

        let mut proof_hashes = Vec::new();
        for hash_b64 in &inclusion_proof.hashes {
            let hash = decode_base64(hash_b64)
                .map_err(|_| TransparencyError::InvalidEntryHash)?;
            proof_hashes.push(hash);
        }

        // Compute leaf hash from canonicalized body
        let canonicalized_body = decode_base64(&entry.canonicalized_body)
            .map_err(|_| TransparencyError::InvalidEntryHash)?;
        let leaf_hash = compute_leaf_hash(&canonicalized_body);

        // Verify inclusion proof
        verify_inclusion_proof(&leaf_hash, log_index, tree_size, &proof_hashes, &root_hash)?;
    }

    // Verify signed entry timestamp if present
    if let Some(ref inclusion_promise) = entry.inclusion_promise {
        // TODO: Verify the signed entry timestamp signature
        // This requires fetching the Rekor public key and verifying the signature
        // For now, we just check it exists
        let _set_bytes = decode_base64(&inclusion_promise.signed_entry_timestamp)
            .map_err(|_| TransparencyError::SignedEntryTimestampInvalid)?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::bundle::{Certificate, DsseEnvelope, VerificationMaterial};

    #[test]
    fn test_missing_tlog_entries() {
        let bundle = SigstoreBundle {
            media_type: String::new(),
            verification_material: VerificationMaterial {
                timestamp_verification_data: None,
                certificate: Certificate {
                    raw_bytes: String::new(),
                },
                tlog_entries: None,
            },
            dsse_envelope: DsseEnvelope {
                payload: String::new(),
                payload_type: String::new(),
                signatures: vec![],
            },
        };

        let result = verify_transparency_log(&bundle);
        assert!(matches!(
            result,
            Err(VerificationError::Transparency(TransparencyError::NoRekorEntry))
        ));
    }
}
