use crate::parser::bundle::{decode_base64, parse_bundle_from_str};
use crate::parser::certificate::{determine_fulcio_instance, parse_der_certificate};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CertificateChain {
    pub leaf: Vec<u8>,               // DER-encoded
    pub intermediates: Vec<Vec<u8>>, // DER-encoded
    pub root: Vec<u8>,               // DER-encoded
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrustBundle {
    pub chains: Vec<CertChain>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CertChain {
    pub certificates: Vec<String>, // PEM-encoded certificates
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FulcioInstance {
    GitHub,
    PublicGood,
}

impl FulcioInstance {
    pub fn trust_bundle_url(&self) -> &'static str {
        match self {
            FulcioInstance::GitHub => "https://fulcio.githubapp.com/api/v2/trustBundle",
            FulcioInstance::PublicGood => "https://fulcio.sigstore.dev/api/v2/trustBundle",
        }
    }

    pub fn from_issuer_cn(cn: &str) -> Option<Self> {
        match cn {
            "Fulcio Intermediate l2" => Some(FulcioInstance::GitHub),
            "sigstore-intermediate" => Some(FulcioInstance::PublicGood),
            _ => None,
        }
    }

    /// Detect Fulcio instance from bundle JSON
    ///
    /// Parses the bundle and extracts the leaf certificate to determine
    /// which Fulcio instance signed it (GitHub or PublicGood).
    ///
    /// # Arguments
    ///
    /// * `bundle_json` - Raw JSON bytes of the sigstore bundle
    ///
    /// # Returns
    ///
    /// Returns the detected `FulcioInstance` or an error if detection fails
    ///
    /// # Example
    ///
    /// ```ignore
    /// # #[cfg(feature = "fetcher")]
    /// # fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// use sigstore_verifier::types::certificate::FulcioInstance;
    /// use sigstore_verifier::fetcher::trust_bundle::fetch_fulcio_trust_bundle;
    ///
    /// let bundle_json = std::fs::read_to_string("bundle.sigstore.json")?;
    /// let instance = FulcioInstance::from_bundle_json(&bundle_json)?;
    /// let trust_bundle = fetch_fulcio_trust_bundle(&instance)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn from_bundle_json(bundle_json: &str) -> Result<Self, String> {
        let bundle = parse_bundle_from_str(bundle_json)
            .map_err(|e| format!("Failed to parse bundle: {}", e))?;

        let leaf_der = decode_base64(&bundle.verification_material.certificate.raw_bytes)
            .map_err(|e| format!("Failed to decode certificate: {}", e))?;

        let leaf_cert = parse_der_certificate(&leaf_der)
            .map_err(|e| format!("Failed to parse certificate: {}", e))?;

        determine_fulcio_instance(&leaf_cert)
            .map_err(|e| format!("Failed to determine Fulcio instance: {}", e))
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OidcIdentity {
    pub issuer: Option<String>,
    pub subject: Option<String>,
    pub workflow_ref: Option<String>,
    pub repository: Option<String>,
    pub event_name: Option<String>,
}
