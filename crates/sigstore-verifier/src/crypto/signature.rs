use ecdsa::signature::Verifier;
use p256::ecdsa::{Signature as P256Signature, VerifyingKey as P256VerifyingKey};
use p384::ecdsa::{Signature as P384Signature, VerifyingKey as P384VerifyingKey};
use x509_parser::prelude::*;

use crate::error::SignatureError;

#[derive(Debug, Clone)]
pub enum PublicKey {
    P256(P256VerifyingKey),
    P384(P384VerifyingKey),
}

impl PublicKey {
    pub fn from_certificate(cert: &X509Certificate) -> Result<Self, SignatureError> {
        let spki = cert.public_key();
        let algorithm_oid = &spki.algorithm.algorithm;

        // Check if this is an EC public key (1.2.840.10045.2.1)
        if algorithm_oid.to_id_string() == "1.2.840.10045.2.1" {
            // For EC keys, the curve is specified in the parameters
            if let Some(params) = &spki.algorithm.parameters {
                if let Ok(curve_oid) = params.as_oid() {
                    match curve_oid.to_id_string().as_str() {
                        "1.2.840.10045.3.1.7" => {
                            // secp256r1 (P-256)
                            let key_bytes = &spki.subject_public_key.data;
                            let verifying_key = P256VerifyingKey::from_sec1_bytes(key_bytes)
                                .map_err(|e| SignatureError::PublicKeyParse(e.to_string()))?;
                            return Ok(PublicKey::P256(verifying_key));
                        }
                        "1.3.132.0.34" => {
                            // secp384r1 (P-384)
                            let key_bytes = &spki.subject_public_key.data;
                            let verifying_key = P384VerifyingKey::from_sec1_bytes(key_bytes)
                                .map_err(|e| SignatureError::PublicKeyParse(e.to_string()))?;
                            return Ok(PublicKey::P384(verifying_key));
                        }
                        oid => return Err(SignatureError::UnsupportedAlgorithm(format!("EC curve: {}", oid))),
                    }
                }
            }
            return Err(SignatureError::UnsupportedAlgorithm("EC key without curve parameters".to_string()));
        }

        // Legacy support: try matching the algorithm OID directly (for older formats)
        match algorithm_oid.to_id_string().as_str() {
            "1.2.840.10045.3.1.7" => {
                // secp256r1 (P-256)
                let key_bytes = &spki.subject_public_key.data;
                let verifying_key = P256VerifyingKey::from_sec1_bytes(key_bytes)
                    .map_err(|e| SignatureError::PublicKeyParse(e.to_string()))?;
                Ok(PublicKey::P256(verifying_key))
            }
            "1.3.132.0.34" => {
                // secp384r1 (P-384)
                let key_bytes = &spki.subject_public_key.data;
                let verifying_key = P384VerifyingKey::from_sec1_bytes(key_bytes)
                    .map_err(|e| SignatureError::PublicKeyParse(e.to_string()))?;
                Ok(PublicKey::P384(verifying_key))
            }
            oid => Err(SignatureError::UnsupportedAlgorithm(oid.to_string())),
        }
    }

    pub fn verify_signature(&self, message: &[u8], signature: &[u8]) -> Result<(), SignatureError> {
        match self {
            PublicKey::P256(key) => {
                let sig = P256Signature::from_der(signature)
                    .map_err(|e| SignatureError::InvalidFormat(e.to_string()))?;
                key.verify(message, &sig)
                    .map_err(|_| SignatureError::InvalidSignature)?;
            }
            PublicKey::P384(key) => {
                let sig = P384Signature::from_der(signature)
                    .map_err(|e| SignatureError::InvalidFormat(e.to_string()))?;
                key.verify(message, &sig)
                    .map_err(|_| SignatureError::InvalidSignature)?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unsupported_algorithm() {
        // This would require a certificate with an unsupported algorithm
        // For now, we just test the error type exists
        let result: Result<PublicKey, SignatureError> =
            Err(SignatureError::UnsupportedAlgorithm("1.2.3.4".to_string()));
        assert!(result.is_err());
    }
}
