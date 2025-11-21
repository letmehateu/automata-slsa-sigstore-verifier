use ::asn1_rs::{FromDer, OctetString};
use x509_parser::prelude::*;
use x509_parser::oid_registry::Oid;

use crate::error::CertificateError;
use crate::types::certificate::OidcIdentity;

// OIDC token claim OIDs (1.3.6.1.4.1.57264.1.x)
const OID_ISSUER: [u64; 9] = [1, 3, 6, 1, 4, 1, 57264, 1, 8]; // Issuer (v2)
const OID_SOURCE_REPOSITORY_URI: [u64; 9] = [1, 3, 6, 1, 4, 1, 57264, 1, 12];
const OID_SOURCE_REPOSITORY_REF: [u64; 9] = [1, 3, 6, 1, 4, 1, 57264, 1, 14];

// Legacy GitHub workflow OIDs (deprecated but still in use)
const OID_GITHUB_WORKFLOW_TRIGGER: [u64; 9] = [1, 3, 6, 1, 4, 1, 57264, 1, 2];
const OID_GITHUB_WORKFLOW_REPOSITORY: [u64; 9] = [1, 3, 6, 1, 4, 1, 57264, 1, 5];
const OID_GITHUB_WORKFLOW_REF: [u64; 9] = [1, 3, 6, 1, 4, 1, 57264, 1, 6];

/// Extract OIDC identity from Fulcio certificate extensions
pub fn extract_oidc_identity(cert: &X509Certificate) -> Result<OidcIdentity, CertificateError> {
    let mut identity = OidcIdentity {
        issuer: None,
        subject: None,
        workflow_ref: None,
        repository: None,
        event_name: None,
    };

    // Extract subject from SAN (Subject Alternative Name)
    if let Some(san_ext) = cert.subject_alternative_name().ok().and_then(|x| x) {
        // The subject is typically in the URI field
        for name in &san_ext.value.general_names {
            if let x509_parser::extensions::GeneralName::RFC822Name(email) = name {
                identity.subject = Some(email.to_string());
            } else if let x509_parser::extensions::GeneralName::URI(uri) = name {
                // For some OIDC providers, subject is in URI
                if identity.subject.is_none() {
                    identity.subject = Some(uri.to_string());
                }
            }
        }
    }

    // Extract custom Fulcio extensions
    for ext in cert.extensions() {
        let oid = &ext.oid;

        // Match against known OIDs
        if oid_equals(oid, &OID_ISSUER) {
            identity.issuer = extract_string_from_extension(ext)?;
        } else if oid_equals(oid, &OID_SOURCE_REPOSITORY_URI) || oid_equals(oid, &OID_GITHUB_WORKFLOW_REPOSITORY) {
            identity.repository = extract_string_from_extension(ext)?;
        } else if oid_equals(oid, &OID_SOURCE_REPOSITORY_REF) || oid_equals(oid, &OID_GITHUB_WORKFLOW_REF) {
            identity.workflow_ref = extract_string_from_extension(ext)?;
        } else if oid_equals(oid, &OID_GITHUB_WORKFLOW_TRIGGER) {
            identity.event_name = extract_string_from_extension(ext)?;
        }
    }

    Ok(identity)
}

fn oid_equals(oid: &Oid, expected: &[u64]) -> bool {
    if let Some(mut iter) = oid.iter() {
        for &expected_val in expected {
            match iter.next() {
                Some(val) if val == expected_val => continue,
                _ => return false,
            }
        }
        iter.next().is_none() // Ensure no extra components
    } else {
        false
    }
}

fn extract_string_from_extension(ext: &X509Extension) -> Result<Option<String>, CertificateError> {
    // The extension value is DER-encoded
    // For string values, it's typically an OCTET STRING containing UTF8String or IA5String
    match OctetString::from_der(ext.value) {
        Ok((_, octet_string)) => {
            let inner_bytes = octet_string.as_ref();

            // Try to parse as UTF8
            if let Ok(s) = std::str::from_utf8(inner_bytes) {
                return Ok(Some(s.to_string()));
            }

            // The inner content might be another DER-encoded string
            // Try parsing as an ASN.1 string type
            if inner_bytes.is_empty() {
                return Ok(None);
            }

            // Simple heuristic: if it starts with a string tag, extract it
            if inner_bytes.len() > 2 {
                let tag = inner_bytes[0];
                let len = inner_bytes[1] as usize;

                // UTF8String (0x0C) or IA5String (0x16) or PrintableString (0x13)
                if (tag == 0x0C || tag == 0x16 || tag == 0x13) && len + 2 <= inner_bytes.len() {
                    if let Ok(s) = std::str::from_utf8(&inner_bytes[2..2 + len]) {
                        return Ok(Some(s.to_string()));
                    }
                }
            }

            Ok(None)
        }
        Err(_) => Ok(None),
    }
}
