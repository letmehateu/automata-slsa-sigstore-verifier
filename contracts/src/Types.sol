//SPDX-License-Identifier: MIT
pragma solidity >=0.8.0;

error InvalidDataLength();
error InvalidCertificateHashesLength();

struct VerificationResult {
    uint64 timestamp;
    bytes32[] certificateHashes; // [leaf, ...intermediates, root]
    bytes subjectDigest;
    string oidcIssuer;
    string oidcSubject;
    string oidcWorkflowRef;
    string oidcRepository;
    string oidcEventName;
}

library VerificationResultParser {
    function parseVerificationResultBytes(bytes memory data)
        internal
        pure
        returns (VerificationResult memory)
    {
        // Validate minimum data length (8 bytes timestamp + 32 bytes wrapper)
        if (data.length < 40) revert InvalidDataLength();

        // Extract timestamp from first 8 bytes (big-endian uint64)
        uint64 signingTime;
        assembly {
            // Load 32 bytes starting from data + 32 (skip length prefix)
            // First 8 bytes contain the big-endian timestamp
            let rawData := mload(add(data, 32))
            // Shift right by 192 bits (24 bytes) to get the first 8 bytes
            signingTime := shr(192, rawData)
        }

        // Extract ABI-encoded data (after 8-byte timestamp + 32-byte wrapper pointer)
        bytes memory abiData;
        assembly {
            // Calculate length of ABI data (skip 8-byte timestamp + 32-byte wrapper)
            let abiLength := sub(mload(data), 40)
            // Allocate memory for ABI data
            abiData := mload(0x40)
            // Store length
            mstore(abiData, abiLength)
            // Copy data from position 40 onwards (8 timestamp + 32 wrapper)
            // Source: data + 32 (length) + 8 (timestamp) + 32 (wrapper)
            // Dest: abiData + 32 (after length field)
            let src := add(add(data, 32), 40)
            let dest := add(abiData, 32)
            // Copy in 32-byte chunks
            let remaining := abiLength
            for {} gt(remaining, 0) {} {
                mstore(dest, mload(src))
                src := add(src, 32)
                dest := add(dest, 32)
                remaining := sub(remaining, 32)
            }
            // Update free memory pointer
            mstore(0x40, add(abiData, add(32, abiLength)))
        }

        // Decode ABI-encoded data
        (
            bytes32[] memory certificateHashes,
            bytes memory subjectDigest,
            string memory oidcIssuer,
            string memory oidcSubject,
            string memory oidcWorkflowRef,
            string memory oidcRepository,
            string memory oidcEventName
        ) = abi.decode(
            abiData,
            (bytes32[], bytes, string, string, string, string, string)
        );

        // Validate certificate hashes (minimum 2: leaf + root)
        if (certificateHashes.length < 2) revert InvalidCertificateHashesLength();

        // Return populated struct
        return VerificationResult({
            timestamp: signingTime,
            certificateHashes: certificateHashes,
            subjectDigest: subjectDigest,
            oidcIssuer: oidcIssuer,
            oidcSubject: oidcSubject,
            oidcWorkflowRef: oidcWorkflowRef,
            oidcRepository: oidcRepository,
            oidcEventName: oidcEventName
        });
    }
}
