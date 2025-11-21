use x509_parser::prelude::*;

use crate::error::CertificateError;
use crate::types::certificate::FulcioInstance;

pub fn parse_der_certificate(der: &[u8]) -> Result<X509Certificate, CertificateError> {
    let (_, cert) = X509Certificate::from_der(der)
        .map_err(|e| CertificateError::ParseError(e.to_string()))?;
    Ok(cert)
}

pub fn parse_pem_certificate(pem_str: &str) -> Result<Vec<u8>, CertificateError> {
    let parsed = ::pem::parse(pem_str.as_bytes())
        .map_err(|e| CertificateError::ParseError(e.to_string()))?;

    if parsed.tag() != "CERTIFICATE" {
        return Err(CertificateError::ParseError(format!(
            "Expected CERTIFICATE tag, got {}",
            parsed.tag()
        )));
    }

    Ok(parsed.into_contents())
}

pub fn extract_issuer_cn(cert: &X509Certificate) -> Result<String, CertificateError> {
    let issuer = cert.issuer();

    for rdn in issuer.iter() {
        for attr in rdn.iter() {
            if attr.attr_type() == &oid_registry::OID_X509_COMMON_NAME {
                // Try as_str() first (for UTF8String), fall back to manual conversion
                // This handles both PrintableString (Tag 19) and UTF8String (Tag 12)
                return attr
                    .as_str()
                    .map(|s| s.to_string())
                    .or_else(|_| {
                        // If as_str() fails, try to convert the raw bytes to UTF-8
                        let bytes = attr.as_slice();
                        std::str::from_utf8(bytes)
                            .map(|s| s.to_string())
                            .map_err(|e| CertificateError::ParseError(e.to_string()))
                    })
                    .map_err(|e| CertificateError::ParseError(e.to_string()));
            }
        }
    }

    Err(CertificateError::ParseError(
        "Common Name not found in issuer".to_string(),
    ))
}

pub fn determine_fulcio_instance(cert: &X509Certificate) -> Result<FulcioInstance, CertificateError> {
    let issuer_cn = extract_issuer_cn(cert)?;
    FulcioInstance::from_issuer_cn(&issuer_cn)
        .ok_or_else(|| CertificateError::UnknownIssuer(issuer_cn))
}

pub fn extract_subject_public_key_info<'a>(cert: &'a X509Certificate) -> &'a SubjectPublicKeyInfo<'a> {
    cert.public_key()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_pem_certificate() {
        let pem = "-----BEGIN CERTIFICATE-----\nMIIBkTCCATigAwIBAgIJAKHHCgVZU6luMAoGCCqGSM49BAMCMA0xCzAJBgNVBAMM\nAkNBMB4XDTI0MDEwMTAwMDAwMFoXDTI1MDEwMTAwMDAwMFowDTELMAkGA1UEAwwC\nQ0EwWTATBgcqhkjOPQIBBggqhkjOPQMBBwNCAATMOCJCdPYpnFCL1qDYnXpnTwxk\nplBFjZmluX8Q2Jz1KqTJqYbPJPHCNmIVnGGpEUxZ0AY5V0VpfHQ4OvZs0gKEo1Mw\nUTAdBgNVHQ4EFgQUl9BhUDLVP7qCJLWqKJWGHQqQVJ4wHwYDVR0jBBgwFoAUl9Bh\nUDLVP7qCJLWqKJWGHQqQVJ4wDwYDVR0TAQH/BAUwAwEB/zAKBggqhkjOPQQDAgNH\nADBEAiBS2gL+3hKqFJKAJRJH9V+CfKPCqB7C5sBXGBqKQDVLUAIgH9xm+MZMoAYl\n3SQJqPHK0yLCt0mXVKCWH3ypVxD7QQE=\n-----END CERTIFICATE-----";

        let result = parse_pem_certificate(pem);
        assert!(result.is_ok());
    }
}
