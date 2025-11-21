use base64::{engine::general_purpose::STANDARD as BASE64, Engine};
use chrono::{DateTime, Utc};

use crate::error::TimestampError;
use crate::parser::rfc3161::{parse_rfc3161_timestamp, MessageImprint, Rfc3161Timestamp};
use crate::types::{CertificateChain, SigstoreBundle};

/// Verify RFC 3161 timestamp token
///
/// This function:
/// 1. Parses the RFC 3161 timestamp from the bundle
/// 2. Verifies the message imprint matches the DSSE signature bytes
/// 3. Verifies the PKCS#7 signature on the timestamp token
/// 4. Returns the signing time from the timestamp
///
/// # Arguments
///
/// * `bundle` - The sigstore bundle containing the RFC 3161 timestamp
/// * `signature_b64` - Base64-encoded DSSE signature bytes
/// * `tsa_chain` - TSA certificate chain for verification
///
/// # Returns
///
/// The signing time from the timestamp token on success
pub fn verify_rfc3161_timestamp(
    bundle: &SigstoreBundle,
    signature_b64: &str,
    tsa_chain: &CertificateChain,
) -> Result<DateTime<Utc>, TimestampError> {
    // Extract RFC 3161 timestamp from bundle
    let rfc3161_timestamps = bundle
        .verification_material
        .timestamp_verification_data
        .as_ref()
        .and_then(|td| td.rfc3161_timestamps.as_ref())
        .ok_or_else(|| TimestampError::Rfc3161Parse("No RFC3161 timestamps in bundle".to_string()))?;

    if rfc3161_timestamps.is_empty() {
        return Err(TimestampError::Rfc3161Parse("Empty RFC3161 timestamps array".to_string()));
    }

    // Use the first timestamp
    let timestamp = &rfc3161_timestamps[0];

    // Decode the base64-encoded timestamp
    let timestamp_der = BASE64
        .decode(&timestamp.signed_timestamp)
        .map_err(|e| TimestampError::Rfc3161Parse(format!("Failed to decode timestamp base64: {}", e)))?;

    // Parse the RFC 3161 timestamp token
    let parsed_timestamp = parse_rfc3161_timestamp(&timestamp_der)?;

    // Decode the DSSE signature bytes
    let signature_bytes = BASE64
        .decode(signature_b64)
        .map_err(|e| TimestampError::Rfc3161Parse(format!("Failed to decode signature base64: {}", e)))?;

    // Verify message imprint matches the signature
    verify_message_imprint(&signature_bytes, &parsed_timestamp.tst_info.message_imprint)?;

    // Verify PKCS#7 signature on the timestamp token
    verify_pkcs7_signature(&timestamp_der, tsa_chain)?;

    Ok(parsed_timestamp.tst_info.gen_time)
}

/// Verify that the message imprint in the timestamp matches the hash of the signature bytes
///
/// This is the core verification that proves the timestamp is for this specific signature.
/// The message imprint should be the hash of the DSSE signature bytes.
fn verify_message_imprint(
    signature_bytes: &[u8],
    message_imprint: &MessageImprint,
) -> Result<(), TimestampError> {
    // Compute hash of signature bytes using the algorithm from the timestamp
    let computed_hash = message_imprint.hash_algorithm.hash(signature_bytes);

    // Compare with the expected hash from the timestamp
    if computed_hash != message_imprint.hashed_message {
        return Err(TimestampError::MessageImprintMismatch {
            expected: hex::encode(&message_imprint.hashed_message),
            actual: hex::encode(&computed_hash),
        });
    }

    Ok(())
}

/// Verify the PKCS#7/CMS signature on the timestamp token
///
/// This verifies that the timestamp was actually signed by the TSA using
/// the provided certificate chain.
///
/// # Arguments
///
/// * `timestamp_der` - DER-encoded timestamp token (SignedData)
/// * `tsa_chain` - TSA certificate chain for verification
fn verify_pkcs7_signature(
    timestamp_der: &[u8],
    tsa_chain: &CertificateChain,
) -> Result<(), TimestampError> {
    use cms::content_info::ContentInfo;
    use cms::signed_data::SignedData;
    use der::{Decode, Encode};

    // Parse ContentInfo
    let content_info = ContentInfo::from_der(timestamp_der)
        .map_err(|e| TimestampError::Rfc3161Parse(format!("Failed to parse ContentInfo: {}", e)))?;

    // Extract SignedData
    let signed_data_bytes = content_info
        .content
        .to_der()
        .map_err(|e| TimestampError::Rfc3161Parse(format!("Failed to encode SignedData: {}", e)))?;

    let signed_data = SignedData::from_der(&signed_data_bytes)
        .map_err(|e| TimestampError::Rfc3161Parse(format!("Failed to parse SignedData: {}", e)))?;

    // Verify we have signer infos
    if signed_data.signer_infos.0.is_empty() {
        return Err(TimestampError::Rfc3161SignatureInvalid);
    }

    // Get the first signer info
    let signer_info = signed_data.signer_infos.0.iter().next()
        .ok_or_else(|| TimestampError::Rfc3161SignatureInvalid)?;

    // Get the encapsulated content (TSTInfo) that was signed
    let signed_content = signed_data
        .encap_content_info
        .econtent
        .as_ref()
        .ok_or_else(|| TimestampError::Rfc3161Parse("No encapsulated content".to_string()))?
        .value();

    // Parse the TSA leaf certificate from the chain
    let tsa_leaf_cert = crate::parser::parse_der_certificate(&tsa_chain.leaf)
        .map_err(|e| TimestampError::InvalidTSACertificate(format!("Failed to parse TSA leaf certificate: {}", e)))?;

    // Extract public key from certificate
    let public_key_info = tsa_leaf_cert.public_key();
    let public_key_der = public_key_info.raw;

    // Verify the signature using the digest algorithm and signature algorithm from signer info
    verify_cms_signature(
        signed_content,
        &signer_info.signature.as_bytes(),
        public_key_der,
        &signer_info.digest_alg,
        &signer_info.signature_algorithm,
    )?;

    Ok(())
}

/// Verify CMS signature using the public key
///
/// This is a simplified verification that:
/// 1. Hashes the signed content with the digest algorithm
/// 2. Verifies the signature using the public key and signature algorithm
fn verify_cms_signature(
    signed_content: &[u8],
    signature: &[u8],
    public_key_der: &[u8],
    digest_alg: &x509_cert::spki::AlgorithmIdentifierOwned,
    sig_alg: &x509_cert::spki::AlgorithmIdentifierOwned,
) -> Result<(), TimestampError> {
    use sha2::{Digest, Sha256, Sha384};

    // Compute digest of signed content
    let digest = match digest_alg.oid.to_string().as_str() {
        "2.16.840.1.101.3.4.2.1" => Sha256::digest(signed_content).to_vec(), // SHA-256
        "2.16.840.1.101.3.4.2.2" => Sha384::digest(signed_content).to_vec(), // SHA-384
        other => {
            return Err(TimestampError::UnsupportedHashAlgorithm(format!(
                "Unsupported digest algorithm: {}",
                other
            )))
        }
    };

    // Verify signature based on algorithm
    // RSA with SHA-256: 1.2.840.113549.1.1.11
    // RSA with SHA-384: 1.2.840.113549.1.1.12
    // ECDSA with SHA-256: 1.2.840.10045.4.3.2
    // ECDSA with SHA-384: 1.2.840.10045.4.3.3
    match sig_alg.oid.to_string().as_str() {
        "1.2.840.113549.1.1.11" | "1.2.840.113549.1.1.12" => {
            verify_rsa_signature(&digest, signature, public_key_der)?
        }
        "1.2.840.10045.4.3.2" | "1.2.840.10045.4.3.3" => {
            verify_ecdsa_signature(&digest, signature, public_key_der)?
        }
        other => {
            return Err(TimestampError::Rfc3161Parse(format!(
                "Unsupported signature algorithm: {}",
                other
            )))
        }
    }

    Ok(())
}

/// Verify RSA signature
fn verify_rsa_signature(
    digest: &[u8],
    signature: &[u8],
    public_key_der: &[u8],
) -> Result<(), TimestampError> {
    use rsa::pkcs1v15::VerifyingKey;
    use rsa::signature::Verifier;
    use rsa::RsaPublicKey;
    use rsa::pkcs8::DecodePublicKey;
    use sha2::{Sha256, Sha384};

    // Parse RSA public key
    let public_key = RsaPublicKey::from_public_key_der(public_key_der)
        .map_err(|e| TimestampError::Rfc3161Parse(format!("Failed to parse RSA public key: {}", e)))?;

    // Create verifying key based on digest length
    match digest.len() {
        32 => {
            // SHA-256
            let verifying_key: VerifyingKey<Sha256> = VerifyingKey::new(public_key);
            let sig = rsa::pkcs1v15::Signature::try_from(signature)
                .map_err(|e| TimestampError::Rfc3161Parse(format!("Invalid RSA signature: {}", e)))?;
            verifying_key
                .verify(digest, &sig)
                .map_err(|_| TimestampError::Rfc3161SignatureInvalid)?;
        }
        48 => {
            // SHA-384
            let verifying_key: VerifyingKey<Sha384> = VerifyingKey::new(public_key);
            let sig = rsa::pkcs1v15::Signature::try_from(signature)
                .map_err(|e| TimestampError::Rfc3161Parse(format!("Invalid RSA signature: {}", e)))?;
            verifying_key
                .verify(digest, &sig)
                .map_err(|_| TimestampError::Rfc3161SignatureInvalid)?;
        }
        _ => {
            return Err(TimestampError::UnsupportedHashAlgorithm(format!(
                "Unexpected digest length: {}",
                digest.len()
            )))
        }
    }

    Ok(())
}

/// Verify ECDSA signature
fn verify_ecdsa_signature(
    digest: &[u8],
    signature: &[u8],
    public_key_der: &[u8],
) -> Result<(), TimestampError> {
    use ecdsa::signature::Verifier;
    use p256::ecdsa::{Signature as P256Signature, VerifyingKey as P256VerifyingKey};
    use p256::pkcs8::DecodePublicKey as P256DecodePublicKey;
    use p384::ecdsa::{Signature as P384Signature, VerifyingKey as P384VerifyingKey};
    use p384::pkcs8::DecodePublicKey as P384DecodePublicKey;

    // Try P-256 first, then P-384 based on digest length
    match digest.len() {
        32 => {
            // P-256 / SHA-256
            let verifying_key = P256VerifyingKey::from_public_key_der(public_key_der)
                .map_err(|e| TimestampError::Rfc3161Parse(format!("Failed to parse P-256 public key: {}", e)))?;

            let sig = P256Signature::from_der(signature)
                .map_err(|e| TimestampError::Rfc3161Parse(format!("Invalid ECDSA signature: {}", e)))?;

            verifying_key
                .verify(digest, &sig)
                .map_err(|_| TimestampError::Rfc3161SignatureInvalid)?;
        }
        48 => {
            // P-384 / SHA-384
            let verifying_key = P384VerifyingKey::from_public_key_der(public_key_der)
                .map_err(|e| TimestampError::Rfc3161Parse(format!("Failed to parse P-384 public key: {}", e)))?;

            let sig = P384Signature::from_der(signature)
                .map_err(|e| TimestampError::Rfc3161Parse(format!("Invalid ECDSA signature: {}", e)))?;

            verifying_key
                .verify(digest, &sig)
                .map_err(|_| TimestampError::Rfc3161SignatureInvalid)?;
        }
        _ => {
            return Err(TimestampError::UnsupportedHashAlgorithm(format!(
                "Unexpected digest length: {}",
                digest.len()
            )))
        }
    }

    Ok(())
}

/// Detect or validate TSA certificate chain
///
/// Returns the TSA chain to use for verification:
/// - If embedded certs exist in the timestamp, extract and use them
/// - Otherwise, use the provided tsa_cert_chain parameter
/// - If neither exists, return error
pub fn detect_or_validate_tsa_chain<'a>(
    timestamp: &Rfc3161Timestamp,
    tsa_cert_chain: Option<&'a CertificateChain>,
) -> Result<&'a CertificateChain, TimestampError> {
    // Check if certificates are embedded in the timestamp
    if timestamp.certificates.is_some() {
        // TODO: Convert embedded DER certificates to CertificateChain
        // For now, we'll require the user to provide the chain
        // This is a simplification that can be improved later
    }

    // Use provided chain if available
    tsa_cert_chain.ok_or(TimestampError::MissingTSAChain)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::rfc3161::{HashAlgorithm, MessageImprint};

    #[test]
    fn test_verify_message_imprint_success() {
        let signature = b"test signature bytes";
        let hash = HashAlgorithm::Sha256.hash(signature);

        let message_imprint = MessageImprint {
            hash_algorithm: HashAlgorithm::Sha256,
            hashed_message: hash,
        };

        let result = verify_message_imprint(signature, &message_imprint);
        assert!(result.is_ok());
    }

    #[test]
    fn test_verify_message_imprint_mismatch() {
        let signature = b"test signature bytes";
        let wrong_hash = vec![0u8; 32]; // Wrong hash

        let message_imprint = MessageImprint {
            hash_algorithm: HashAlgorithm::Sha256,
            hashed_message: wrong_hash,
        };

        let result = verify_message_imprint(signature, &message_imprint);
        assert!(result.is_err());
        assert!(matches!(result, Err(TimestampError::MessageImprintMismatch { .. })));
    }
}
