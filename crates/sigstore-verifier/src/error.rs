use thiserror::Error;

#[derive(Debug, Error)]
pub enum VerificationError {
    #[error("Bundle parsing error: {0}")]
    BundleParse(#[from] serde_json::Error),

    #[error("Certificate error: {0}")]
    Certificate(#[from] CertificateError),

    #[error("Signature error: {0}")]
    Signature(#[from] SignatureError),

    #[error("Timestamp error: {0}")]
    Timestamp(#[from] TimestampError),

    #[error("Transparency log error: {0}")]
    Transparency(#[from] TransparencyError),

    #[error("Subject digest is zero")]
    ZeroSubjectDigest,

    #[error("Subject digest mismatch: expected {expected}, got {actual}")]
    SubjectDigestMismatch { expected: String, actual: String },

    #[error("HTTP request failed: {0}")]
    HttpError(#[from] reqwest::Error),

    #[error("Base64 decode error: {0}")]
    Base64Decode(#[from] base64::DecodeError),

    #[error("Invalid bundle format: {0}")]
    InvalidBundleFormat(String),
}

#[derive(Debug, Error)]
pub enum CertificateError {
    #[error("Failed to parse certificate: {0}")]
    ParseError(String),

    #[error("Certificate chain verification failed: {0}")]
    ChainVerificationFailed(String),

    #[error("Certificate expired or not yet valid")]
    ValidityPeriod,

    #[error("Signing time outside certificate validity: signing_time={signing_time}, not_before={not_before}, not_after={not_after}")]
    SigningTimeOutsideValidity {
        signing_time: String,
        not_before: String,
        not_after: String,
    },

    #[error("Unknown issuer: {0}")]
    UnknownIssuer(String),

    #[error("Missing certificate in bundle")]
    MissingCertificate,

    #[error("Failed to fetch trust bundle: {0}")]
    TrustBundleFetch(String),

    #[error("Self-signed certificate verification failed")]
    SelfSignedVerificationFailed,
}

#[derive(Debug, Error)]
pub enum SignatureError {
    #[error("Unsupported signature algorithm: {0}")]
    UnsupportedAlgorithm(String),

    #[error("Invalid signature format: {0}")]
    InvalidFormat(String),

    #[error("Signature verification failed")]
    InvalidSignature,

    #[error("Failed to parse public key: {0}")]
    PublicKeyParse(String),

    #[error("DER encoding error: {0}")]
    DerError(String),
}

#[derive(Debug, Error)]
pub enum TimestampError {
    #[error("No timestamp found (neither RFC3161 nor integrated time)")]
    NoTimestamp,

    #[error("Bundle contains both RFC3161 timestamps and Rekor entries. Only one timestamp mechanism is allowed.")]
    BothTimestampMechanisms,

    #[error("RFC3161 timestamp verification is not yet supported. This bundle requires RFC3161 support. See RFC-3161.md for implementation details.")]
    Rfc3161NotSupported,

    #[error("Failed to parse RFC3161 timestamp: {0}")]
    Rfc3161Parse(String),

    #[error("RFC3161 timestamp signature verification failed")]
    Rfc3161SignatureInvalid,

    #[error("Message imprint mismatch: expected {expected}, got {actual}")]
    MessageImprintMismatch { expected: String, actual: String },

    #[error("Unsupported hash algorithm in RFC3161 timestamp: {0}")]
    UnsupportedHashAlgorithm(String),

    #[error("TSA certificate chain is required but not provided. RFC3161 timestamp does not contain embedded certificates.")]
    MissingTSAChain,

    #[error("Invalid TSA certificate: {0}")]
    InvalidTSACertificate(String),

    #[error("Invalid integrated time")]
    InvalidIntegratedTime,
}

#[derive(Debug, Error)]
pub enum TransparencyError {
    #[error("No Rekor entry found in bundle")]
    NoRekorEntry,

    #[error("Invalid entry hash")]
    InvalidEntryHash,

    #[error("Merkle tree inclusion proof verification failed")]
    InclusionProofFailed,

    #[error("Signed entry timestamp verification failed")]
    SignedEntryTimestampInvalid,
}
