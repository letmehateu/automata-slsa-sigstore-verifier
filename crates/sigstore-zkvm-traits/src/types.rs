use serde::{Deserialize, Serialize};
use sigstore_verifier::types::result::VerificationOptions;
use sigstore_verifier::types::certificate::CertificateChain;

/// Input data for the zkVM prover
///
/// This structure contains all the necessary data for the guest program
/// to perform sigstore bundle verification.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProverInput {
    /// Sigstore attestation bundle in JSON format
    pub bundle_json: Vec<u8>,

    /// Options for verification (expected digest, issuer, subject, etc.)
    pub verification_options: VerificationOptions,

    /// Trust bundle containing Fulcio certificate chain in PEM format
    pub trust_bundle: CertificateChain,

    /// Optional TSA certificate chain in PEM format for RFC3161 timestamp verification
    pub tsa_cert_chain: Option<CertificateChain>,
}

impl ProverInput {
    /// Create a new ProverInput with the given parameters
    pub fn new(
        bundle_json: Vec<u8>,
        verification_options: VerificationOptions,
        trust_bundle: CertificateChain,
        tsa_cert_chain: Option<CertificateChain>,
    ) -> Self {
        Self {
            bundle_json,
            verification_options,
            trust_bundle,
            tsa_cert_chain,
        }
    }

    /// Encode the ProverInput to bytes for host-to-guest communication
    ///
    /// This method serializes the ProverInput using bincode for efficient
    /// binary encoding to be passed from the host to the guest program.
    pub fn encode_input(&self) -> Result<Vec<u8>, String> {
        bincode::serialize(self)
            .map_err(|e| format!("Failed to serialize ProverInput: {}", e))
    }

    /// Parse ProverInput from bytes in the guest program
    ///
    /// This method deserializes the ProverInput from the bincode format
    /// created by encode_input().
    pub fn parse_input(bytes: &[u8]) -> Result<Self, String> {
        bincode::deserialize(bytes)
            .map_err(|e| format!("Failed to deserialize ProverInput: {}", e))
    }
}
