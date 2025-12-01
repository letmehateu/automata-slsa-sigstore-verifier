//SPDX-License-Identifier: MIT
pragma solidity >=0.8.0;

// =============================================================================
// Verification Result Data Structures
// =============================================================================
//
// These types represent the output of Sigstore bundle verification.
//
// DigestAlgorithm: Hash algorithm identifier
//   0 = Unknown, 1 = SHA256, 2 = SHA384
//
// TimestampProofType: Type of timestamp proof used
//   0 = None, 1 = RFC3161 (TSA), 2 = Rekor (transparency log)
//
// VerificationResult: Complete verification output including:
//   - Certificate chain hashes (signing cert chain)
//   - Subject digest (artifact hash from attestation)
//   - OIDC identity (from Fulcio certificate)
//   - Timestamp proof (RFC 3161 or Rekor)
//
// =============================================================================

error InvalidDataLength();
error InvalidCertificateHashesLength();
error InvalidTimestampProofType();

/// @notice Hash algorithm identifier
/// @dev 0 = Unknown, 1 = SHA256, 2 = SHA384
enum DigestAlgorithm {
    Unknown,
    Sha256,
    Sha384
}

/// @notice Timestamp proof type identifier
/// @dev 0 = None, 1 = RFC3161 (TSA), 2 = Rekor (transparency log)
enum TimestampProofType {
    None,
    Rfc3161,
    Rekor
}

struct VerificationResult {
    uint64 timestamp;
    TimestampProofType timestampProofType;
    bytes32[] certificateHashes; // [leaf, ...intermediates, root]
    bytes subjectDigest;
    DigestAlgorithm subjectDigestAlgorithm;
    string oidcIssuer;
    string oidcSubject;
    string oidcWorkflowRef;
    string oidcRepository;
    string oidcEventName;
    // RFC 3161 timestamp proof fields (populated when timestampProofType == Rfc3161)
    bytes32[] tsaChainHashes; // TSA certificate chain hashes
    DigestAlgorithm messageImprintAlgorithm;
    bytes messageImprint; // Hash of DSSE signature
    // Rekor timestamp proof fields (populated when timestampProofType == Rekor)
    bytes32 rekorLogId; // SHA256 of Rekor's public key
    uint64 rekorLogIndex; // Tree leaf index (for Merkle proof verification)
    uint64 rekorEntryIndex; // Entry index (for API queries)
}

library VerificationResultParser {
    function parseVerificationResultBytes(bytes memory data) internal pure returns (VerificationResult memory result) {
        // Validate minimum data length (8 bytes timestamp + 1 byte proof type + 32 byte tuple offset + ABI data)
        if (data.length < 73) revert InvalidDataLength();

        // Extract timestamp and proof type from header, then parse ABI data
        (result.timestamp, result.timestampProofType) = _parseHeader(data);
        bytes memory abiData = _extractAbiData(data);
        _decodeAbiData(abiData, result);

        // Validate certificate hashes (minimum 2: leaf + root)
        if (result.certificateHashes.length < 2) revert InvalidCertificateHashesLength();
    }

    function _parseHeader(bytes memory data) private pure returns (uint64 signingTime, TimestampProofType proofType) {
        uint8 proofTypeRaw;
        assembly ("memory-safe") {
            let rawData := mload(add(data, 32))
            signingTime := shr(192, rawData)
            proofTypeRaw := and(shr(184, rawData), 0xff)
        }

        if (proofTypeRaw == 0) {
            proofType = TimestampProofType.None;
        } else if (proofTypeRaw == 1) {
            proofType = TimestampProofType.Rfc3161;
        } else if (proofTypeRaw == 2) {
            proofType = TimestampProofType.Rekor;
        } else {
            revert InvalidTimestampProofType();
        }
    }

    function _extractAbiData(bytes memory data) private pure returns (bytes memory abiData) {
        // Skip: 9 bytes header (8 timestamp + 1 proof type) + 32 bytes tuple offset wrapper
        assembly ("memory-safe") {
            let abiLength := sub(mload(data), 41)
            abiData := mload(0x40)
            mstore(abiData, abiLength)
            let src := add(add(data, 32), 41)
            let dest := add(abiData, 32)
            let remaining := abiLength
            for {} gt(remaining, 0) {} {
                mstore(dest, mload(src))
                src := add(src, 32)
                dest := add(dest, 32)
                remaining := sub(remaining, 32)
            }
            mstore(0x40, add(abiData, add(32, abiLength)))
        }
    }

    function _decodeAbiData(bytes memory abiData, VerificationResult memory result) private pure {
        // Decode all fields
        (
            bytes32[] memory certHashes,
            bytes memory subjectDigest,
            uint8 subjectDigestAlgRaw,
            string memory oidcIssuer,
            string memory oidcSubject,
            string memory oidcWorkflowRef,
            string memory oidcRepository,
            string memory oidcEventName,
            bytes32[] memory tsaChainHashes,
            uint8 messageImprintAlgRaw,
            bytes memory messageImprint,
            bytes32 rekorLogId,
            uint64 rekorLogIndex,
            uint64 rekorEntryIndex
        ) = abi.decode(
            abiData,
            (
                bytes32[],
                bytes,
                uint8,
                string,
                string,
                string,
                string,
                string,
                bytes32[],
                uint8,
                bytes,
                bytes32,
                uint64,
                uint64
            )
        );

        // Assign to result struct
        result.certificateHashes = certHashes;
        result.subjectDigest = subjectDigest;
        result.oidcIssuer = oidcIssuer;
        result.oidcSubject = oidcSubject;
        result.oidcWorkflowRef = oidcWorkflowRef;
        result.oidcRepository = oidcRepository;
        result.oidcEventName = oidcEventName;
        result.tsaChainHashes = tsaChainHashes;
        result.messageImprint = messageImprint;
        result.rekorLogId = rekorLogId;
        result.rekorLogIndex = rekorLogIndex;
        result.rekorEntryIndex = rekorEntryIndex;
        result.subjectDigestAlgorithm = _toDigestAlgorithm(subjectDigestAlgRaw);
        result.messageImprintAlgorithm = _toDigestAlgorithm(messageImprintAlgRaw);
    }

    function _toDigestAlgorithm(uint8 value) private pure returns (DigestAlgorithm) {
        if (value == 1) return DigestAlgorithm.Sha256;
        if (value == 2) return DigestAlgorithm.Sha384;
        return DigestAlgorithm.Unknown;
    }
}
