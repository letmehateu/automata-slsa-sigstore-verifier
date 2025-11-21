use crate::error::CertificateError;
use crate::parser::certificate::parse_pem_certificate;
use crate::types::certificate::{CertificateChain, FulcioInstance, TrustBundle};

pub fn fetch_trust_bundle(instance: &FulcioInstance) -> Result<CertificateChain, CertificateError> {
    let url = instance.trust_bundle_url();

    let response = reqwest::blocking::get(url)
        .map_err(|e| CertificateError::TrustBundleFetch(e.to_string()))?;

    if !response.status().is_success() {
        return Err(CertificateError::TrustBundleFetch(format!(
            "HTTP error: {}",
            response.status()
        )));
    }

    let bundle: TrustBundle = response
        .json()
        .map_err(|e| CertificateError::TrustBundleFetch(e.to_string()))?;

    if bundle.chains.is_empty() {
        return Err(CertificateError::TrustBundleFetch(
            "No certificate chains in trust bundle".to_string(),
        ));
    }

    // Get the first chain (there should typically be only one)
    let chain = &bundle.chains[0];

    if chain.certificates.is_empty() {
        return Err(CertificateError::TrustBundleFetch(
            "Empty certificate chain".to_string(),
        ));
    }

    // Parse all certificates from PEM to DER
    let mut der_certs = Vec::new();
    for pem_cert in &chain.certificates {
        let der = parse_pem_certificate(pem_cert)?;
        der_certs.push(der);
    }

    // The chain structure is: [intermediate(s)..., root]
    // We need at least 2 certificates (intermediate + root)
    if der_certs.len() < 2 {
        return Err(CertificateError::TrustBundleFetch(
            "Certificate chain too short".to_string(),
        ));
    }

    let root = der_certs.pop().unwrap();
    let intermediates = der_certs;

    // If there are multiple intermediates, the first one is closest to the leaf
    // The structure is: leaf (from bundle) -> intermediate[0] -> ... -> intermediate[n] -> root

    Ok(CertificateChain {
        leaf: Vec::new(), // Leaf will be set by caller
        intermediates,
        root,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore] // Requires network access
    fn test_fetch_github_trust_bundle() {
        let result = fetch_trust_bundle(&FulcioInstance::GitHub);
        assert!(result.is_ok());

        let chain = result.unwrap();
        assert!(!chain.intermediates.is_empty());
        assert!(!chain.root.is_empty());
    }

    #[test]
    #[ignore] // Requires network access
    fn test_fetch_public_trust_bundle() {
        let result = fetch_trust_bundle(&FulcioInstance::PublicGood);
        assert!(result.is_ok());

        let chain = result.unwrap();
        assert!(!chain.intermediates.is_empty());
        assert!(!chain.root.is_empty());
    }
}
