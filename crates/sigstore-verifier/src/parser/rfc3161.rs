use chrono::{DateTime, TimeZone, Utc};
use cms::content_info::ContentInfo;
use cms::signed_data::SignedData;
use der::{Decode, Encode};
use sha2::{Digest, Sha256, Sha384};

use crate::error::TimestampError;

/// Hash algorithm used in message imprint
#[derive(Debug, Clone, PartialEq)]
pub enum HashAlgorithm {
    Sha256,
    Sha384,
}

impl HashAlgorithm {
    /// Hash the given data using this algorithm
    pub fn hash(&self, data: &[u8]) -> Vec<u8> {
        match self {
            HashAlgorithm::Sha256 => Sha256::digest(data).to_vec(),
            HashAlgorithm::Sha384 => Sha384::digest(data).to_vec(),
        }
    }
}

/// Message imprint from TSTInfo
#[derive(Debug, Clone)]
pub struct MessageImprint {
    pub hash_algorithm: HashAlgorithm,
    pub hashed_message: Vec<u8>,
}

/// Parsed RFC 3161 timestamp information
#[derive(Debug, Clone)]
pub struct TSTInfo {
    pub gen_time: DateTime<Utc>,
    pub message_imprint: MessageImprint,
}

/// Parsed RFC 3161 timestamp token with optional embedded certificates
#[derive(Debug, Clone)]
pub struct Rfc3161Timestamp {
    pub tst_info: TSTInfo,
    pub certificates: Option<Vec<Vec<u8>>>, // DER-encoded certificates
    pub signed_data: Vec<u8>,                // Raw SignedData for signature verification
}

/// Parse an RFC 3161 timestamp token from DER-encoded bytes
///
/// This parses the CMS ContentInfo structure and extracts:
/// - TSTInfo (timestamp information including signing time and message imprint)
/// - Embedded certificates (if present)
/// - SignedData for later signature verification
pub fn parse_rfc3161_timestamp(der: &[u8]) -> Result<Rfc3161Timestamp, TimestampError> {
    // Parse the ContentInfo structure
    let content_info = ContentInfo::from_der(der)
        .map_err(|e| TimestampError::Rfc3161Parse(format!("Failed to parse ContentInfo: {}", e)))?;

    // Extract SignedData from content
    let signed_data_bytes = content_info
        .content
        .to_der()
        .map_err(|e| TimestampError::Rfc3161Parse(format!("Failed to encode SignedData: {}", e)))?;

    let signed_data = SignedData::from_der(&signed_data_bytes)
        .map_err(|e| TimestampError::Rfc3161Parse(format!("Failed to parse SignedData: {}", e)))?;

    // Extract TSTInfo from encapsulated content
    let tst_info = parse_tstinfo(&signed_data)?;

    // Extract embedded certificates if present
    let certificates = extract_tsa_certificates(&signed_data);

    Ok(Rfc3161Timestamp {
        tst_info,
        certificates,
        signed_data: signed_data_bytes,
    })
}

/// Parse TSTInfo from SignedData
///
/// TSTInfo is the encapsulated content in the SignedData structure
fn parse_tstinfo(signed_data: &SignedData) -> Result<TSTInfo, TimestampError> {
    // Get encapsulated content (TSTInfo)
    let encap_content = signed_data
        .encap_content_info
        .econtent
        .as_ref()
        .ok_or_else(|| TimestampError::Rfc3161Parse("No encapsulated content in SignedData".to_string()))?;

    let tstinfo_der = encap_content.value();

    // Parse TSTInfo structure manually using asn1-rs
    parse_tstinfo_asn1(tstinfo_der)
}

/// Parse TSTInfo ASN.1 structure
///
/// TSTInfo ::= SEQUENCE {
///   version INTEGER,
///   policy TSAPolicyId,
///   messageImprint MessageImprint,
///   serialNumber INTEGER,
///   genTime GeneralizedTime,
///   ...
/// }
fn parse_tstinfo_asn1(der: &[u8]) -> Result<TSTInfo, TimestampError> {
    use asn1_rs::{FromDer, Integer, Sequence, Any};

    let (rem, tstinfo_seq) = Sequence::from_der(der)
        .map_err(|e| TimestampError::Rfc3161Parse(format!("Failed to parse TSTInfo sequence: {}", e)))?;

    // Parse the sequence contents manually
    let content = tstinfo_seq.content.as_ref();

    // Skip version (INTEGER)
    let (rem, _version) = Integer::from_der(content)
        .map_err(|e| TimestampError::Rfc3161Parse(format!("Failed to parse version: {}", e)))?;

    // Skip policy (OID)
    let (rem, _policy) = Any::from_der(rem)
        .map_err(|e| TimestampError::Rfc3161Parse(format!("Failed to parse policy: {}", e)))?;

    // Parse messageImprint (SEQUENCE)
    let (rem, message_imprint_obj) = Sequence::from_der(rem)
        .map_err(|e| TimestampError::Rfc3161Parse(format!("Failed to parse messageImprint: {}", e)))?;

    let message_imprint = parse_message_imprint_from_sequence(&message_imprint_obj)
        .map_err(|e| TimestampError::Rfc3161Parse(format!("Failed to parse MessageImprint: {}", e)))?;

    // Skip serialNumber (INTEGER)
    let (rem, _serial) = Integer::from_der(rem)
        .map_err(|e| TimestampError::Rfc3161Parse(format!("Failed to parse serialNumber: {}", e)))?;

    // Parse genTime (GeneralizedTime)
    let (_, gen_time_obj) = Any::from_der(rem)
        .map_err(|e| TimestampError::Rfc3161Parse(format!("Failed to parse genTime: {}", e)))?;

    let gen_time = parse_generalized_time(gen_time_obj.as_bytes())
        .map_err(|e| TimestampError::Rfc3161Parse(format!("Failed to parse GeneralizedTime: {}", e)))?;

    Ok(TSTInfo {
        gen_time,
        message_imprint,
    })
}

/// Parse MessageImprint from Sequence object
fn parse_message_imprint_from_sequence(seq: &asn1_rs::Sequence) -> Result<MessageImprint, String> {
    use asn1_rs::{FromDer, Sequence, OctetString};

    let content = seq.content.as_ref();

    // Parse hashAlgorithm (AlgorithmIdentifier - SEQUENCE)
    let (rem, hash_alg_seq) = Sequence::from_der(content)
        .map_err(|e| format!("Failed to parse hashAlgorithm: {}", e))?;

    let hash_algorithm = parse_hash_algorithm_from_sequence(&hash_alg_seq)?;

    // Parse hashedMessage (OCTET STRING)
    let (_, hashed_message_octets) = OctetString::from_der(rem)
        .map_err(|e| format!("Failed to parse hashedMessage: {}", e))?;

    let hashed_message = hashed_message_octets.as_cow().to_vec();

    Ok(MessageImprint {
        hash_algorithm,
        hashed_message,
    })
}

/// Parse AlgorithmIdentifier from Sequence to extract hash algorithm
fn parse_hash_algorithm_from_sequence(seq: &asn1_rs::Sequence) -> Result<HashAlgorithm, String> {
    use asn1_rs::{FromDer, Oid};

    let content = seq.content.as_ref();

    // Parse algorithm OID
    let (_, oid) = Oid::from_der(content)
        .map_err(|e| format!("Failed to parse algorithm OID: {}", e))?;

    // OID for SHA-256: 2.16.840.1.101.3.4.2.1
    // OID for SHA-384: 2.16.840.1.101.3.4.2.2
    match oid.to_string().as_str() {
        "2.16.840.1.101.3.4.2.1" => Ok(HashAlgorithm::Sha256),
        "2.16.840.1.101.3.4.2.2" => Ok(HashAlgorithm::Sha384),
        other => Err(format!("Unsupported hash algorithm OID: {}", other)),
    }
}

/// Parse GeneralizedTime to DateTime<Utc>
///
/// Format: YYYYMMDDHHMMSSsZ or YYYYMMDDHHMMSS.fffZ
fn parse_generalized_time(der: &[u8]) -> Result<DateTime<Utc>, String> {
    use asn1_rs::{FromDer, GeneralizedTime};

    let (_, gen_time) = GeneralizedTime::from_der(der)
        .map_err(|e| format!("Failed to parse GeneralizedTime: {}", e))?;

    // Convert ASN.1 GeneralizedTime to chrono DateTime
    let time_str = gen_time.to_string();

    // Parse format: YYYYMMDDHHMMSSZ or YYYYMMDDHHMMSS.fffZ
    let time_str_clean = time_str.trim_end_matches('Z');

    // Handle fractional seconds if present
    let (date_time_part, _frac) = if let Some(pos) = time_str_clean.find('.') {
        time_str_clean.split_at(pos)
    } else {
        (time_str_clean, "")
    };

    if date_time_part.len() < 14 {
        return Err(format!("Invalid GeneralizedTime format: {}", time_str));
    }

    let year: i32 = date_time_part[0..4].parse()
        .map_err(|e| format!("Failed to parse year: {}", e))?;
    let month: u32 = date_time_part[4..6].parse()
        .map_err(|e| format!("Failed to parse month: {}", e))?;
    let day: u32 = date_time_part[6..8].parse()
        .map_err(|e| format!("Failed to parse day: {}", e))?;
    let hour: u32 = date_time_part[8..10].parse()
        .map_err(|e| format!("Failed to parse hour: {}", e))?;
    let minute: u32 = date_time_part[10..12].parse()
        .map_err(|e| format!("Failed to parse minute: {}", e))?;
    let second: u32 = date_time_part[12..14].parse()
        .map_err(|e| format!("Failed to parse second: {}", e))?;

    Utc.with_ymd_and_hms(year, month, day, hour, minute, second)
        .single()
        .ok_or_else(|| format!("Invalid date/time values: {}", time_str))
}

/// Extract TSA certificates from SignedData if present
///
/// Returns None if no certificates are embedded, otherwise returns
/// a vector of DER-encoded certificates
pub fn extract_tsa_certificates(signed_data: &SignedData) -> Option<Vec<Vec<u8>>> {
    signed_data.certificates.as_ref().map(|cert_set| {
        cert_set
            .0
            .iter()
            .filter_map(|cert_choice| {
                use cms::cert::CertificateChoices;
                // Extract the certificate bytes - only handle Certificate variant
                match cert_choice {
                    CertificateChoices::Certificate(cert) => {
                        cert.to_der().ok()
                    }
                    _ => None,  // Skip other certificate types
                }
            })
            .collect()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_algorithm_sha256() {
        let alg = HashAlgorithm::Sha256;
        let data = b"test data";
        let hash = alg.hash(data);

        // Verify it's 32 bytes (256 bits)
        assert_eq!(hash.len(), 32);
    }

    #[test]
    fn test_hash_algorithm_sha384() {
        let alg = HashAlgorithm::Sha384;
        let data = b"test data";
        let hash = alg.hash(data);

        // Verify it's 48 bytes (384 bits)
        assert_eq!(hash.len(), 48);
    }
}
