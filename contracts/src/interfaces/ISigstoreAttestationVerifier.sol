//SPDX-License-Identifier: MIT
pragma solidity >=0.8.0;

import {VerificationResult} from "../Types.sol";

enum ZkCoProcessorType {
    // if the ZkCoProcessorType is included as None in the AttestationSubmitted event log
    // it indicates that the attestation of the DCAP quote is executed entirely on-chain
    None,
    RiscZero,
    Succinct
}

/**
 * @title ZK Co-Processor Configuration Object
 * @param programIdentifier - This is the identifier of the ZK Program, required for
 * verification
 * @param zkVerifier - Points to the address of the ZK Verifier contract. Ideally
 * this should be pointing to a universal verifier, that may support multiple proof types and/or versions.
 */
struct ZkCoProcessorConfig {
    bytes32 programIdentifier;
    address zkVerifier;
}

interface ISigstoreAttestationVerifier {
    /**
     * @param zkCoProcessorType 1 - RiscZero, 2 - Succinct... etc.
     * @return this returns the latest DCAP program identifier for the specified ZK Co-processor
     */
    function programIdentifier(ZkCoProcessorType zkCoProcessorType) external view returns (bytes32);

    /**
     * @notice gets the default (universal) ZK verifier for the provided ZK Co-processor
     */
    function zkVerifier(ZkCoProcessorType zkCoProcessorType) external view returns (address);

    function verifyAndAttestWithZKProof(
        bytes calldata output,
        ZkCoProcessorType zkCoProcessor,
        bytes calldata proofBytes
    ) external returns (VerificationResult memory verifiedOutput);
}
