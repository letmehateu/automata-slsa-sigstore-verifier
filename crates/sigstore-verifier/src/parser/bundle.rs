use std::path::Path;

use base64::prelude::*;
use crate::error::VerificationError;
use crate::types::bundle::{DsseEnvelope, SigstoreBundle};
use crate::types::dsse::Statement;

pub fn parse_bundle_from_path(path: &Path) -> Result<SigstoreBundle, VerificationError> {
    let contents = std::fs::read_to_string(path)
        .map_err(|e| VerificationError::InvalidBundleFormat(e.to_string()))?;
    parse_bundle_from_str(&contents)
}

pub fn parse_bundle_from_bytes(bytes: &[u8]) -> Result<SigstoreBundle, VerificationError> {
    let bundle: SigstoreBundle = serde_json::from_slice(bytes)?;
    validate_bundle(&bundle)?;
    Ok(bundle)
}

pub fn parse_bundle_from_str(json: &str) -> Result<SigstoreBundle, VerificationError> {
    let bundle: SigstoreBundle = serde_json::from_str(json)?;
    validate_bundle(&bundle)?;
    Ok(bundle)
}

fn validate_bundle(bundle: &SigstoreBundle) -> Result<(), VerificationError> {
    if !bundle
        .media_type
        .starts_with("application/vnd.dev.sigstore.bundle")
    {
        return Err(VerificationError::InvalidBundleFormat(format!(
            "Unsupported media type: {}",
            bundle.media_type
        )));
    }

    if bundle.dsse_envelope.signatures.is_empty() {
        return Err(VerificationError::InvalidBundleFormat(
            "No signatures in DSSE envelope".to_string(),
        ));
    }

    Ok(())
}

pub fn parse_dsse_payload(envelope: &DsseEnvelope) -> Result<Statement, VerificationError> {
    let payload_bytes = BASE64_STANDARD.decode(&envelope.payload)?;
    let statement: Statement = serde_json::from_slice(&payload_bytes)?;
    Ok(statement)
}

pub fn decode_base64(input: &str) -> Result<Vec<u8>, VerificationError> {
    BASE64_STANDARD.decode(input).map_err(|e| e.into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_bundle_invalid_media_type() {
        use crate::types::bundle::{Certificate, Signature, VerificationMaterial};

        let mut bundle = SigstoreBundle {
            media_type: "invalid".to_string(),
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
                signatures: vec![Signature {
                    sig: String::new(),
                }],
            },
        };

        let result = validate_bundle(&bundle);
        assert!(result.is_err());

        bundle.media_type = "application/vnd.dev.sigstore.bundle.v0.3+json".to_string();
        assert!(validate_bundle(&bundle).is_ok());
    }
}
