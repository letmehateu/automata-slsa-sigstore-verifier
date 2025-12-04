// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import {Script} from "forge-std/Script.sol";
import {console2} from "forge-std/console2.sol";
import {Config} from "forge-std/Config.sol";

import {SigstoreAttestationVerifier} from "../src/SigstoreAttestationVerifier.sol";
import {ZkCoProcessorType} from "../src/interfaces/ISigstoreAttestationVerifier.sol";

/// @title ConfigScript
/// @notice Script for updating ZK co-processor configuration on deployed SigstoreAttestationVerifier
/// @dev Reads verifier addresses from deployment.toml, contract address from deployments/{chainId}.json,
///      and program IDs from environment variables
///
/// Usage:
///   RiscZero:  forge script script/Config.s.sol --sig "configureRiscZero()" --rpc-url $RPC_URL --broadcast
///   SP1:       forge script script/Config.s.sol --sig "configureSp1()" --rpc-url $RPC_URL --broadcast
///   Pico:      forge script script/Config.s.sol --sig "configurePico()" --rpc-url $RPC_URL --broadcast
///
/// Required env vars:
///   RISC_ZERO_IMAGE_ID - RiscZero program image ID (for configureRiscZero)
///   SP1_VKEY           - SP1 verification key (for configureSp1)
///   PICO_VKEY          - Pico verification key (for configurePico)
contract ConfigScript is Script, Config {
    string internal constant CONFIG_PATH = "./script/config/deployment.toml";
    string internal constant DEPLOYMENTS_PATH = "./deployments/";

    error DeploymentNotFound(uint256 chainId);

    error ZkCoProcessorNotEnabled(string name);
    error MissingProgramId(string name);
    error MissingVerifierAddress();

    /// @notice Get the deployed contract address from deployments/{chainId}.json
    function _getDeployedContract(uint256 chainId) internal view returns (address) {
        string memory path = string.concat(DEPLOYMENTS_PATH, vm.toString(chainId), ".json");
        if (!vm.exists(path)) {
            revert DeploymentNotFound(chainId);
        }
        string memory json = vm.readFile(path);
        return vm.parseJsonAddress(json, ".SigstoreAttestationVerifier");
    }

    /// @notice Configure RiscZero on the deployed contract
    function configureRiscZero() external {
        _loadConfig(CONFIG_PATH, false);
        uint256 chainId = block.chainid;

        // 1. Check if RiscZero is enabled
        bool enabled = config.get(chainId, "enable_risc_zero").toBool();
        if (!enabled) {
            revert ZkCoProcessorNotEnabled("RiscZero");
        }

        // 2. Read program ID from env
        bytes32 imageId = vm.envOr("RISC_ZERO_IMAGE_ID", bytes32(0));
        if (imageId == bytes32(0)) {
            revert MissingProgramId("RISC_ZERO_IMAGE_ID");
        }

        // Read verifier address from config
        address verifierAddr = config.get(chainId, "risc_zero_verifier").toAddress();
        if (verifierAddr == address(0)) {
            revert MissingVerifierAddress();
        }

        // Read deployed contract address from deployment file
        address contractAddr = _getDeployedContract(chainId);

        console2.log("Configuring RiscZero on chain", chainId);
        console2.log("  Contract:", contractAddr);
        console2.log("  Verifier:", verifierAddr);
        console2.log("  Image ID:", vm.toString(imageId));

        // 3. Send the transaction
        vm.startBroadcast();
        SigstoreAttestationVerifier(contractAddr)
            .setZkCoProcessorConfig(ZkCoProcessorType.RiscZero, imageId, verifierAddr);
        vm.stopBroadcast();

        console2.log("RiscZero configured successfully!");
    }

    /// @notice Configure SP1 (Succinct) on the deployed contract
    function configureSp1() external {
        _loadConfig(CONFIG_PATH, false);
        uint256 chainId = block.chainid;

        // 1. Check if SP1 is enabled
        bool enabled = config.get(chainId, "enable_sp1").toBool();
        if (!enabled) {
            revert ZkCoProcessorNotEnabled("SP1");
        }

        // 2. Read program ID from env
        bytes32 vkey = vm.envOr("SP1_VKEY", bytes32(0));
        if (vkey == bytes32(0)) {
            revert MissingProgramId("SP1_VKEY");
        }

        // Read verifier address from config
        address verifierAddr = config.get(chainId, "sp1_verifier").toAddress();
        if (verifierAddr == address(0)) {
            revert MissingVerifierAddress();
        }

        // Read deployed contract address from deployment file
        address contractAddr = _getDeployedContract(chainId);

        console2.log("Configuring SP1 on chain", chainId);
        console2.log("  Contract:", contractAddr);
        console2.log("  Verifier:", verifierAddr);
        console2.log("  VKey:", vm.toString(vkey));

        // 3. Send the transaction
        vm.startBroadcast();
        SigstoreAttestationVerifier(contractAddr).setZkCoProcessorConfig(ZkCoProcessorType.Succinct, vkey, verifierAddr);
        vm.stopBroadcast();

        console2.log("SP1 configured successfully!");
    }

    /// @notice Configure Pico on the deployed contract
    function configurePico() external {
        _loadConfig(CONFIG_PATH, false);
        uint256 chainId = block.chainid;

        // 1. Check if Pico is enabled
        bool enabled = config.get(chainId, "enable_pico").toBool();
        if (!enabled) {
            revert ZkCoProcessorNotEnabled("Pico");
        }

        // 2. Read program ID from env
        bytes32 vkey = vm.envOr("PICO_VKEY", bytes32(0));
        if (vkey == bytes32(0)) {
            revert MissingProgramId("PICO_VKEY");
        }

        // Read verifier address from config
        address verifierAddr = config.get(chainId, "pico_verifier").toAddress();
        if (verifierAddr == address(0)) {
            revert MissingVerifierAddress();
        }

        // Read deployed contract address from deployment file
        address contractAddr = _getDeployedContract(chainId);

        console2.log("Configuring Pico on chain", chainId);
        console2.log("  Contract:", contractAddr);
        console2.log("  Verifier:", verifierAddr);
        console2.log("  VKey:", vm.toString(vkey));

        // 3. Send the transaction
        vm.startBroadcast();
        SigstoreAttestationVerifier(contractAddr).setZkCoProcessorConfig(ZkCoProcessorType.Pico, vkey, verifierAddr);
        vm.stopBroadcast();

        console2.log("Pico configured successfully!");
    }

    /// @notice Configure all enabled ZK co-processors on the deployed contract
    function configureAll() external {
        _loadConfig(CONFIG_PATH, false);
        uint256 chainId = block.chainid;
        address contractAddr = _getDeployedContract(chainId);

        console2.log("Configuring all enabled ZK co-processors on chain", chainId);
        console2.log("  Contract:", contractAddr);

        vm.startBroadcast();

        // Configure RiscZero if enabled
        if (config.get(chainId, "enable_risc_zero").toBool()) {
            bytes32 imageId = vm.envOr("RISC_ZERO_IMAGE_ID", bytes32(0));
            address verifierAddr = config.get(chainId, "risc_zero_verifier").toAddress();

            if (imageId != bytes32(0) && verifierAddr != address(0)) {
                console2.log("  Configuring RiscZero...");
                SigstoreAttestationVerifier(contractAddr)
                    .setZkCoProcessorConfig(ZkCoProcessorType.RiscZero, imageId, verifierAddr);
            }
        }

        // Configure SP1 if enabled
        if (config.get(chainId, "enable_sp1").toBool()) {
            bytes32 vkey = vm.envOr("SP1_VKEY", bytes32(0));
            address verifierAddr = config.get(chainId, "sp1_verifier").toAddress();

            if (vkey != bytes32(0) && verifierAddr != address(0)) {
                console2.log("  Configuring SP1...");
                SigstoreAttestationVerifier(contractAddr)
                    .setZkCoProcessorConfig(ZkCoProcessorType.Succinct, vkey, verifierAddr);
            }
        }

        // Configure Pico if enabled
        if (config.get(chainId, "enable_pico").toBool()) {
            bytes32 vkey = vm.envOr("PICO_VKEY", bytes32(0));
            address verifierAddr = config.get(chainId, "pico_verifier").toAddress();

            if (vkey != bytes32(0) && verifierAddr != address(0)) {
                console2.log("  Configuring Pico...");
                SigstoreAttestationVerifier(contractAddr)
                    .setZkCoProcessorConfig(ZkCoProcessorType.Pico, vkey, verifierAddr);
            }
        }

        vm.stopBroadcast();

        console2.log("Configuration complete!");
    }
}
