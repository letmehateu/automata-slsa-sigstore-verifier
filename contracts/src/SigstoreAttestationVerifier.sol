// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "./interfaces/ISigstoreAttestationVerifier.sol";
import {VerificationResultParser} from "./Types.sol";
import {Ownable} from "solady/auth/Ownable.sol";

// ZK-Coprocessor imports:
import {IRiscZeroVerifier} from "risc0/IRiscZeroVerifier.sol";
import {ISP1Verifier} from "@sp1-contracts/ISP1Verifier.sol";

contract SigstoreAttestationVerifier is ISigstoreAttestationVerifier, Ownable {
    mapping(ZkCoProcessorType => ZkCoProcessorConfig) _zkConfig;

    // 20b15e84
    error InvalidZkCoProcessorType();
    // a18c0a0a
    error MissingZkVerifier();
    // bfec3ebd
    error MissingZkProgramId();

    event AttestationSubmitted(ZkCoProcessorType verifierType, bytes output);
    event ZkCoProcessorUpdated(ZkCoProcessorType indexed zkCoProcessor, bytes32 programIdentifier, address zkVerifier);

    constructor(address owner) {
        _initializeOwner(owner);
    }

    function setZkCoProcessorConfig(ZkCoProcessorType _zkCoProcessor, bytes32 _programIdentifier, address _zkVerifier)
        external
        onlyOwner
    {
        _noneZkConfigCheck(_zkCoProcessor);
        _zkConfig[_zkCoProcessor] = ZkCoProcessorConfig(_programIdentifier, _zkVerifier);
        emit ZkCoProcessorUpdated(_zkCoProcessor, _programIdentifier, _zkVerifier);
    }

    function programIdentifier(ZkCoProcessorType zkCoProcessorType) external view override returns (bytes32) {
        return _zkConfig[zkCoProcessorType].programIdentifier;
    }

    function zkVerifier(ZkCoProcessorType zkCoProcessorType) external view override returns (address) {
        return _zkConfig[zkCoProcessorType].zkVerifier;
    }

    function verifyAndAttestWithZKProof(
        bytes calldata output,
        ZkCoProcessorType zkCoProcessor,
        bytes calldata proofBytes
    ) external returns (VerificationResult memory verifiedOutput) {
        _noneZkConfigCheck(zkCoProcessor);
        ZkCoProcessorConfig memory config = _zkConfig[zkCoProcessor];

        require(config.zkVerifier != address(0), MissingZkVerifier());
        require(config.programIdentifier != bytes32(0), MissingZkProgramId());

        if (zkCoProcessor == ZkCoProcessorType.RiscZero) {
            IRiscZeroVerifier(config.zkVerifier).verify(proofBytes, config.programIdentifier, sha256(output));
        } else if (zkCoProcessor == ZkCoProcessorType.Succinct) {
            ISP1Verifier(config.zkVerifier).verifyProof(config.programIdentifier, output, proofBytes);
        } else {
            revert InvalidZkCoProcessorType();
        }

        emit AttestationSubmitted(zkCoProcessor, output);
        verifiedOutput = VerificationResultParser.parseVerificationResultBytes(output);
    }

    function _noneZkConfigCheck(ZkCoProcessorType zkCoProcessor) private pure {
        require(zkCoProcessor != ZkCoProcessorType.None, InvalidZkCoProcessorType());
    }
}
