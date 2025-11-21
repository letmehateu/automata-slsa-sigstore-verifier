use base64::{engine::general_purpose::STANDARD as BASE64, Engine};
use chrono::{DateTime, Utc};
use x509_parser::prelude::*;

use crate::error::{CertificateError, TimestampError};
use crate::parser::{parse_integrated_time, rfc3161::parse_rfc3161_timestamp};
use crate::types::{SigstoreBundle, TransparencyLogEntry};

/// Extract signing time from RFC 3161 timestamp
pub fn get_rfc3161_time(bundle: &SigstoreBundle) -> Result<DateTime<Utc>, TimestampError> {
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

    Ok(parsed_timestamp.tst_info.gen_time)
}

pub fn get_integrated_time(entry: &TransparencyLogEntry) -> Result<DateTime<Utc>, TimestampError> {
    parse_integrated_time(&entry.integrated_time)
}

pub fn verify_signing_time_in_validity(
    signing_time: &DateTime<Utc>,
    cert: &X509Certificate,
) -> Result<(), CertificateError> {
    let validity = cert.validity();
    let not_before = validity.not_before.timestamp();
    let not_after = validity.not_after.timestamp();
    let signing_timestamp = signing_time.timestamp();

    if signing_timestamp < not_before || signing_timestamp > not_after {
        return Err(CertificateError::SigningTimeOutsideValidity {
            signing_time: signing_time.to_rfc3339(),
            not_before: validity.not_before.to_string(),
            not_after: validity.not_after.to_string(),
        });
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_integrated_time() {
        let entry = TransparencyLogEntry {
            log_index: Some("123".to_string()),
            log_id: None,
            kind_version: None,
            integrated_time: "1732068373".to_string(),
            inclusion_promise: None,
            inclusion_proof: None,
            canonicalized_body: String::new(),
        };

        let result = get_integrated_time(&entry);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().timestamp(), 1732068373);
    }
}
