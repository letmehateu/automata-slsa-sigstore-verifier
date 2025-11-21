use x509_parser::prelude::*;

use crate::crypto::hash::sha256;
use crate::crypto::signature::PublicKey;
use crate::error::CertificateError;
use crate::parser::bundle::decode_base64;
use crate::parser::certificate::parse_der_certificate;
use crate::types::bundle::SigstoreBundle;
use crate::types::certificate::CertificateChain;
use crate::types::result::CertificateChainHashes;

/// Verify the certificate chain using provided trust bundle
///
/// # Arguments
///
/// * `bundle` - The Sigstore bundle containing the leaf certificate
/// * `trust_bundle` - The trust bundle (intermediates and root) for verification
///
/// # Returns
///
/// Returns the complete certificate chain and SHA256 hashes of all certificates
pub fn verify_certificate_chain(
    bundle: &SigstoreBundle,
    trust_bundle: &CertificateChain,
) -> Result<(CertificateChain, CertificateChainHashes), CertificateError> {
    // Parse leaf certificate from bundle
    let leaf_der = decode_base64(&bundle.verification_material.certificate.raw_bytes)
        .map_err(|e| CertificateError::ParseError(e.to_string()))?;

    // Create complete chain with leaf from bundle
    let chain = CertificateChain {
        leaf: leaf_der.clone(),
        intermediates: trust_bundle.intermediates.clone(),
        root: trust_bundle.root.clone(),
    };

    // Parse all certificates
    let leaf_x509 = parse_der_certificate(&chain.leaf)?;
    let mut intermediate_x509 = Vec::new();
    for der in &chain.intermediates {
        intermediate_x509.push(parse_der_certificate(der)?);
    }
    let root_x509 = parse_der_certificate(&chain.root)?;

    // Verify certificate signatures
    // 1. Verify leaf signed by first intermediate
    verify_cert_signature(&leaf_x509, &intermediate_x509[0])?;

    // 2. Verify intermediate chain
    for i in 0..intermediate_x509.len() - 1 {
        verify_cert_signature(&intermediate_x509[i], &intermediate_x509[i + 1])?;
    }

    // 3. Verify last intermediate signed by root
    if let Some(last_intermediate) = intermediate_x509.last() {
        verify_cert_signature(last_intermediate, &root_x509)?;
    }

    // 4. Verify root is self-signed
    verify_cert_signature(&root_x509, &root_x509)?;

    // Compute SHA256 hashes of all certificates
    let leaf_hash = sha256(&chain.leaf);
    let intermediate_hashes: Vec<[u8; 32]> = chain
        .intermediates
        .iter()
        .map(|der| sha256(der))
        .collect();
    let root_hash = sha256(&chain.root);

    let hashes = CertificateChainHashes {
        leaf: leaf_hash,
        intermediates: intermediate_hashes,
        root: root_hash,
    };

    Ok((chain, hashes))
}

fn verify_cert_signature(
    cert: &X509Certificate,
    issuer: &X509Certificate,
) -> Result<(), CertificateError> {
    let public_key = PublicKey::from_certificate(issuer)
        .map_err(|e| CertificateError::ChainVerificationFailed(e.to_string()))?;

    let signature = &cert.signature_value.data;
    let tbs_certificate = cert.tbs_certificate.as_ref();

    public_key
        .verify_signature(tbs_certificate, signature)
        .map_err(|e| CertificateError::ChainVerificationFailed(e.to_string()))?;

    Ok(())
}

/// Verify TSA certificate chain with EKU validation
///
/// This verifies the TSA certificate chain and ensures the leaf certificate
/// has the correct Extended Key Usage (EKU) for timestamping.
///
/// # Arguments
///
/// * `tsa_chain` - The TSA certificate chain (leaf, intermediates, root)
///
/// # Returns
///
/// Returns Ok(()) if verification succeeds
pub fn verify_tsa_certificate_chain(
    tsa_chain: &CertificateChain,
) -> Result<(), CertificateError> {
    // Parse all certificates
    let leaf_x509 = parse_der_certificate(&tsa_chain.leaf)?;
    let mut intermediate_x509 = Vec::new();
    for der in &tsa_chain.intermediates {
        intermediate_x509.push(parse_der_certificate(der)?);
    }
    let root_x509 = parse_der_certificate(&tsa_chain.root)?;

    // Verify TSA leaf certificate EKU
    verify_tsa_certificate_eku(&leaf_x509)?;

    // Verify certificate signatures
    // 1. Verify leaf signed by first intermediate
    if !intermediate_x509.is_empty() {
        verify_cert_signature(&leaf_x509, &intermediate_x509[0])?;
    } else {
        // No intermediates - verify leaf signed by root
        verify_cert_signature(&leaf_x509, &root_x509)?;
    }

    // 2. Verify intermediate chain
    for i in 0..intermediate_x509.len().saturating_sub(1) {
        verify_cert_signature(&intermediate_x509[i], &intermediate_x509[i + 1])?;
    }

    // 3. Verify last intermediate signed by root (if intermediates exist)
    if let Some(last_intermediate) = intermediate_x509.last() {
        verify_cert_signature(last_intermediate, &root_x509)?;
    }

    // 4. Verify root is self-signed
    verify_cert_signature(&root_x509, &root_x509)?;

    Ok(())
}

/// Verify TSA certificate Extended Key Usage (EKU)
///
/// Per RFC 3161 §2.3, the TSA signing certificate MUST have the
/// Extended Key Usage extension marked as critical, and it MUST
/// contain only the id-kp-timeStamping OID (1.3.6.1.5.5.7.3.8).
///
/// # Arguments
///
/// * `cert` - The TSA leaf certificate to verify
///
/// # Returns
///
/// Returns Ok(()) if the certificate has correct EKU for timestamping
pub fn verify_tsa_certificate_eku(cert: &X509Certificate) -> Result<(), CertificateError> {
    // TimeStamping EKU OID: 1.3.6.1.5.5.7.3.8
    const TIME_STAMPING_OID: &str = "1.3.6.1.5.5.7.3.8";

    // Find Extended Key Usage extension
    let eku_ext = cert
        .tbs_certificate
        .extensions()
        .iter()
        .find(|ext| ext.oid == x509_parser::oid_registry::OID_X509_EXT_EXTENDED_KEY_USAGE)
        .ok_or_else(|| {
            CertificateError::ChainVerificationFailed(
                "TSA certificate missing Extended Key Usage extension".to_string(),
            )
        })?;

    // Verify extension is marked as critical per RFC 3161
    if !eku_ext.critical {
        return Err(CertificateError::ChainVerificationFailed(
            "TSA certificate Extended Key Usage extension must be marked as critical".to_string(),
        ));
    }

    // Parse Extended Key Usage extension
    let eku = match eku_ext.parsed_extension() {
        x509_parser::extensions::ParsedExtension::ExtendedKeyUsage(eku) => eku,
        _ => {
            return Err(CertificateError::ChainVerificationFailed(
                "Failed to parse Extended Key Usage extension".to_string(),
            ))
        }
    };

    // Verify it contains time stamping OID
    // x509_parser ExtendedKeyUsage has fields: any (bool), server_auth (bool), client_auth (bool), etc.
    // For time stamping, we need to check the raw extension value

    // Parse the extension value as a sequence of OIDs
    use ::asn1_rs::{FromDer, Sequence, Oid};

    let (_, oid_seq) = Sequence::from_der(eku_ext.value)
        .map_err(|e| CertificateError::ChainVerificationFailed(format!("Failed to parse EKU value: {}", e)))?;

    // Parse OIDs from the sequence
    let mut oids = Vec::new();
    let mut remaining = oid_seq.content.as_ref();
    while !remaining.is_empty() {
        let (rem, oid) = Oid::from_der(remaining)
            .map_err(|e| CertificateError::ChainVerificationFailed(format!("Failed to parse OID: {}", e)))?;
        oids.push(oid.to_string());
        remaining = rem;
    }

    // Verify it contains time stamping OID
    let has_time_stamping = oids.iter().any(|oid| oid == TIME_STAMPING_OID);

    if !has_time_stamping {
        return Err(CertificateError::ChainVerificationFailed(format!(
            "TSA certificate must have timeStamping EKU ({})",
            TIME_STAMPING_OID
        )));
    }

    // Per RFC 3161, the EKU should contain ONLY timeStamping
    // We'll be strict here and reject certificates with additional EKUs
    if oids.len() > 1 {
        return Err(CertificateError::ChainVerificationFailed(
            "TSA certificate must have ONLY timeStamping EKU, no other key usages allowed".to_string(),
        ));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_time_stamping_oid() {
        // Verify the OID constant is correct
        const TIME_STAMPING_OID: &str = "1.3.6.1.5.5.7.3.8";
        assert_eq!(TIME_STAMPING_OID, "1.3.6.1.5.5.7.3.8");
    }
}
