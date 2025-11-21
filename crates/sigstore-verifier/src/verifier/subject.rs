use crate::crypto::hash::hex_decode;
use crate::error::VerificationError;
use crate::types::dsse::Statement;

pub fn verify_subject_digest(
    statement: &Statement,
    expected_digest: Option<&[u8]>,
) -> Result<Vec<u8>, VerificationError> {
    // Get SHA256 digest from subject
    let digest_hex = statement
        .get_subject_digest("sha256")
        .ok_or_else(|| {
            VerificationError::InvalidBundleFormat("No sha256 digest in subject".to_string())
        })?;

    // Decode hex digest
    let digest = hex_decode(&digest_hex)
        .map_err(|e| VerificationError::InvalidBundleFormat(format!("Invalid digest hex: {}", e)))?;

    // Check digest is not all zeros
    if digest.iter().all(|&b| b == 0) {
        return Err(VerificationError::ZeroSubjectDigest);
    }

    // If expected digest provided, verify it matches
    if let Some(expected) = expected_digest {
        if digest != expected {
            return Err(VerificationError::SubjectDigestMismatch {
                expected: hex::encode(expected),
                actual: digest_hex,
            });
        }
    }

    Ok(digest)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::dsse::Subject;
    use std::collections::HashMap;

    #[test]
    fn test_verify_subject_digest_success() {
        let mut digest_map = HashMap::new();
        digest_map.insert(
            "sha256".to_string(),
            "658913cfebe8a49165264e2b5e54ad99b3bdbfbc8cd281b3cfaa949a21588f18".to_string(),
        );

        let statement = Statement {
            statement_type: "test".to_string(),
            subject: vec![Subject {
                name: "artifact".to_string(),
                digest: digest_map,
            }],
            predicate_type: "test".to_string(),
            predicate: serde_json::Value::Null,
        };

        let result = verify_subject_digest(&statement, None);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 32);
    }

    #[test]
    fn test_verify_subject_digest_zero() {
        let mut digest_map = HashMap::new();
        digest_map.insert(
            "sha256".to_string(),
            "0000000000000000000000000000000000000000000000000000000000000000".to_string(),
        );

        let statement = Statement {
            statement_type: "test".to_string(),
            subject: vec![Subject {
                name: "artifact".to_string(),
                digest: digest_map,
            }],
            predicate_type: "test".to_string(),
            predicate: serde_json::Value::Null,
        };

        let result = verify_subject_digest(&statement, None);
        assert!(matches!(result, Err(VerificationError::ZeroSubjectDigest)));
    }

    #[test]
    fn test_verify_subject_digest_mismatch() {
        let mut digest_map = HashMap::new();
        digest_map.insert(
            "sha256".to_string(),
            "658913cfebe8a49165264e2b5e54ad99b3bdbfbc8cd281b3cfaa949a21588f18".to_string(),
        );

        let statement = Statement {
            statement_type: "test".to_string(),
            subject: vec![Subject {
                name: "artifact".to_string(),
                digest: digest_map,
            }],
            predicate_type: "test".to_string(),
            predicate: serde_json::Value::Null,
        };

        let expected = vec![0u8; 32];
        let result = verify_subject_digest(&statement, Some(&expected));
        assert!(matches!(
            result,
            Err(VerificationError::SubjectDigestMismatch { .. })
        ));
    }
}
