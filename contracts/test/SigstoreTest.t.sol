// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.0;

import {Test} from "forge-std/Test.sol";

import {SigstoreAttestationVerifier} from "../src/SigstoreAttestationVerifier.sol";
import {ZkCoProcessorType} from "../src/interfaces/ISigstoreAttestationVerifier.sol";
import {RiscZeroGroth16Verifier} from "risc0/groth16/RiscZeroGroth16Verifier.sol";
import {ControlID} from "risc0/groth16/ControlID.sol";
import {SP1Verifier} from "@sp1-contracts/v5.0.0/SP1VerifierGroth16.sol";

contract SigstoreTest is Test {

    RiscZeroGroth16Verifier risc0Verifier;
    SP1Verifier sp1Verifier;
    SigstoreAttestationVerifier sigstoreVerifier;
    address admin = address(1);

    bytes32 constant RISC0_IMAGE_ID = 0x59c443f59e07f88155939288024ac7eaae58db4db826600f08318e9441795121;
    bytes32 constant SP1_VKEY = 0x001bf0267a7348bf563502165c5cd9c5c3fd107e34d63b09509fb4ba3b8df48b;

    event AttestationSubmitted(ZkCoProcessorType verifierType, bytes output);

    function setUp() public {
        vm.startPrank(admin);

        risc0Verifier = new RiscZeroGroth16Verifier(ControlID.CONTROL_ROOT, ControlID.BN254_CONTROL_ID);
        sp1Verifier = new SP1Verifier();

        sigstoreVerifier = new SigstoreAttestationVerifier(admin);
        sigstoreVerifier.setZkCoProcessorConfig(
            ZkCoProcessorType.RiscZero,
            RISC0_IMAGE_ID,
            address(risc0Verifier)
        );
        sigstoreVerifier.setZkCoProcessorConfig(
            ZkCoProcessorType.Succinct,
            SP1_VKEY,
            address(sp1Verifier)
        );

        vm.stopPrank();
    }

    function testRiscZeroProofVerification() public {
        string memory path = string.concat(
            vm.projectRoot(),
            "/",
            "test",
            "/",
            "fixtures",
            "/",
            "boundless-public.json"
        );

        (bytes memory output, bytes memory proof) = _readFixture(path);

        vm.expectEmit(false, false, false, true);
        emit AttestationSubmitted(ZkCoProcessorType.RiscZero, output);
        sigstoreVerifier.verifyAndAttestWithZKProof(
            output,
            ZkCoProcessorType.RiscZero,
            proof
        );
    }

    function testSp1ProofVerification() public {
        string memory path = string.concat(
            vm.projectRoot(),
            "/",
            "test",
            "/",
            "fixtures",
            "/",
            "sp1_github.json"
        );

        (bytes memory output, bytes memory proof) = _readFixture(path);

        vm.expectEmit(false, false, false, true);
        emit AttestationSubmitted(ZkCoProcessorType.Succinct, output);
        sigstoreVerifier.verifyAndAttestWithZKProof(
            output,
            ZkCoProcessorType.Succinct,
            proof
        );
    }

    function _readFixture(string memory path) private view returns (
        bytes memory output,
        bytes memory proof
    ) {
        string memory json = vm.readFile(path);
        output = abi.decode(
            vm.parseJson(json, ".journal"),
            (bytes)
        );
        proof = abi.decode(
            vm.parseJson(json, ".proof"),
            (bytes)
        );
    }

}
