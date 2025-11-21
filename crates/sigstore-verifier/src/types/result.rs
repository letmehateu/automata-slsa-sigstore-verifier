use chrono::{DateTime, Utc};

use super::certificate::OidcIdentity;

#[derive(Debug, Clone)]
pub struct VerificationResult {
    pub certificate_hashes: CertificateChainHashes,
    pub signing_time: DateTime<Utc>,
    pub subject_digest: Vec<u8>,
    pub oidc_identity: Option<OidcIdentity>,
}

#[derive(Debug, Clone)]
pub struct CertificateChainHashes {
    pub leaf: [u8; 32],
    pub intermediates: Vec<[u8; 32]>,
    pub root: [u8; 32],
}

impl CertificateChainHashes {
    pub fn as_tuple(&self) -> ([u8; 32], Vec<[u8; 32]>, [u8; 32]) {
        (self.leaf, self.intermediates.clone(), self.root)
    }
}

#[derive(Debug, Clone, Default)]
pub struct VerificationOptions {
    /// Optional expected digest to verify against the subject digest in the attestation
    pub expected_digest: Option<Vec<u8>>,

    /// Allow certificates without valid SCT (Signed Certificate Timestamp) verification
    ///
    /// **NOT YET IMPLEMENTED** - This field is reserved for future Certificate Transparency verification.
    ///
    /// When implemented, this will control whether to require valid SCTs embedded in the
    /// Fulcio certificate. SCTs prove that the certificate was logged in Certificate
    /// Transparency logs, providing an additional audit trail.
    ///
    /// - `false` (recommended): Require valid SCT verification (stricter security policy)
    /// - `true`: Skip SCT verification (more permissive, current behavior)
    pub allow_insecure_sct: bool,

    /// Optional expected OIDC issuer (e.g., "https://token.actions.githubusercontent.com")
    pub expected_issuer: Option<String>,

    /// Optional expected OIDC subject (e.g., "repo:owner/repo:ref:refs/heads/main")
    pub expected_subject: Option<String>,
}
