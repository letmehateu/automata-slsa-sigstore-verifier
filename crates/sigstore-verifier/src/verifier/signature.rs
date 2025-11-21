use crate::crypto::signature::PublicKey;
use crate::error::VerificationError;
use crate::parser::bundle::decode_base64;
use crate::parser::certificate::parse_der_certificate;
use crate::types::bundle::DsseEnvelope;
use crate::types::certificate::CertificateChain;

const DSSE_PREFIX: &[u8] = b"DSSEv1";

pub fn verify_dsse_signature(
    envelope: &DsseEnvelope,
    chain: &CertificateChain,
) -> Result<(), VerificationError> {
    if envelope.signatures.is_empty() {
        return Err(VerificationError::InvalidBundleFormat(
            "No signatures in envelope".to_string(),
        ));
    }

    // Parse leaf certificate to extract public key
    let leaf_cert = parse_der_certificate(&chain.leaf)
        .map_err(|e| VerificationError::InvalidBundleFormat(e.to_string()))?;
    let public_key = PublicKey::from_certificate(&leaf_cert)?;

    // DSSE signature is over: "DSSEv1" || len(payloadType) || payloadType || len(payload) || payload
    let pae = create_pae(&envelope.payload_type, &envelope.payload)?;

    // Verify the first signature (bundles typically have one signature)
    let signature_bytes = decode_base64(&envelope.signatures[0].sig)?;

    public_key
        .verify_signature(&pae, &signature_bytes)
        .map_err(|e| e.into())
}

fn create_pae(payload_type: &str, payload_b64: &str) -> Result<Vec<u8>, VerificationError> {
    // Decode base64 payload
    let payload = decode_base64(payload_b64)?;

    // PAE = "DSSEv1" || len(payloadType) || payloadType || len(payload) || payload
    let mut pae = Vec::new();

    // Add prefix
    pae.extend_from_slice(DSSE_PREFIX);
    pae.push(b' ');

    // Add payloadType length (as decimal string) and space
    let payload_type_len = payload_type.len().to_string();
    pae.extend_from_slice(payload_type_len.as_bytes());
    pae.push(b' ');

    // Add payloadType and space
    pae.extend_from_slice(payload_type.as_bytes());
    pae.push(b' ');

    // Add payload length (as decimal string) and space
    let payload_len = payload.len().to_string();
    pae.extend_from_slice(payload_len.as_bytes());
    pae.push(b' ');

    // Add payload
    pae.extend_from_slice(&payload);

    Ok(pae)
}

#[cfg(test)]
mod tests {
    use super::*;
    use base64::prelude::*;

    #[test]
    fn test_create_pae() {
        let payload_type = "application/vnd.in-toto+json";
        let payload_b64 = BASE64_STANDARD.encode(b"test payload");

        let result = create_pae(payload_type, &payload_b64);
        assert!(result.is_ok());

        let pae = result.unwrap();
        assert!(pae.starts_with(DSSE_PREFIX));
    }

    #[test]
    fn test_create_pae_empty() {
        let payload_type = "test";
        let payload_b64 = BASE64_STANDARD.encode(b"");

        let result = create_pae(payload_type, &payload_b64);
        assert!(result.is_ok());
    }
}
