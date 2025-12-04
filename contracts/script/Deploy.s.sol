// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import {Script} from "forge-std/Script.sol";
import {console2} from "forge-std/console2.sol";
import {Config} from "forge-std/Config.sol";
import {stdJson} from "forge-std/StdJson.sol";

import {SigstoreAttestationVerifier} from "../src/SigstoreAttestationVerifier.sol";
import {ZkCoProcessorType} from "../src/interfaces/ISigstoreAttestationVerifier.sol";

/// @title Deploy
/// @notice Multi-chain deployment script for SigstoreAttestationVerifier
/// @dev Uses forge-std Config for TOML-based configuration management
///
/// Usage:
///   Single chain:  forge script script/Deploy.s.sol --sig "runSingle()" --rpc-url $RPC_URL --broadcast
///   Multi-chain:   forge script script/Deploy.s.sol --sig "run()" --broadcast
contract Deploy is Script, Config {
    bytes32 constant SIGSTORE_ATTESTATION_VERIFIER_SALT = keccak256(bytes("SIGSTORE_ATTESTATION_VERIFIER"));

    using stdJson for string;

    // ============================================================
    //                          ERRORS
    // ============================================================

    error InvalidOwner();
    error InvalidVerifierAddress();
    error InvalidProgramId();

    // ============================================================
    //                        STRUCTURES
    // ============================================================

    /// @notice Global configuration (same across all chains)
    struct GlobalConfig {
        bytes32 riscZeroImageId;
        bytes32 sp1Vkey;
        bytes32 picoVkey;
    }

    /// @notice Per-chain deployment configuration
    struct ChainConfig {
        address owner;
        address riscZeroVerifier;
        address sp1Verifier;
        address picoVerifier;
        bool enableRiscZero;
        bool enableSp1;
        bool enablePico;
    }

    /// @notice Deployment result for a single chain
    struct DeploymentResult {
        uint256 chainId;
        address verifier;
        bool riscZeroConfigured;
        bool sp1Configured;
        bool picoConfigured;
    }

    // ============================================================
    //                          STORAGE
    // ============================================================

    string internal constant CONFIG_PATH = "./script/config/deployment.toml";
    string internal constant DEPLOYMENTS_DIR = "./deployments/";

    GlobalConfig internal globalConfig;
    DeploymentResult[] internal deploymentResults;

    // ============================================================
    //                    CONFIGURATION LOADING
    // ============================================================

    /// @notice Loads global configuration (program IDs) from environment variables
    /// @dev Reads RISC_ZERO_IMAGE_ID, SP1_VKEY, and PICO_VKEY from env
    function _loadGlobalConfig() internal {
        globalConfig.riscZeroImageId = vm.envOr("RISC_ZERO_IMAGE_ID", bytes32(0));
        globalConfig.sp1Vkey = vm.envOr("SP1_VKEY", bytes32(0));
        globalConfig.picoVkey = vm.envOr("PICO_VKEY", bytes32(0));

        console2.log("Global config loaded from environment:");
        console2.log("  RiscZero Image ID:", vm.toString(globalConfig.riscZeroImageId));
        console2.log("  SP1 VKey:", vm.toString(globalConfig.sp1Vkey));
        console2.log("  Pico VKey:", vm.toString(globalConfig.picoVkey));
    }

    /// @notice Loads chain-specific configuration from TOML via StdConfig
    /// @param chainId The chain ID to load configuration for
    /// @return cfg The parsed chain configuration
    function _loadChainConfig(uint256 chainId) internal view returns (ChainConfig memory cfg) {
        cfg.owner = config.get(chainId, "owner").toAddress();
        cfg.enableRiscZero = config.get(chainId, "enable_risc_zero").toBool();
        cfg.enableSp1 = config.get(chainId, "enable_sp1").toBool();
        cfg.enablePico = config.get(chainId, "enable_pico").toBool();

        if (cfg.enableRiscZero) {
            cfg.riscZeroVerifier = config.get(chainId, "risc_zero_verifier").toAddress();
        }
        if (cfg.enableSp1) {
            cfg.sp1Verifier = config.get(chainId, "sp1_verifier").toAddress();
        }
        if (cfg.enablePico) {
            cfg.picoVerifier = config.get(chainId, "pico_verifier").toAddress();
        }
    }

    // ============================================================
    //                        VALIDATION
    // ============================================================

    /// @notice Validates the chain configuration before deployment
    function _validateConfig(ChainConfig memory cfg) internal view {
        if (cfg.owner == address(0)) {
            revert InvalidOwner();
        }

        if (cfg.enableRiscZero) {
            if (cfg.riscZeroVerifier == address(0)) {
                revert InvalidVerifierAddress();
            }
            if (globalConfig.riscZeroImageId == bytes32(0)) {
                revert InvalidProgramId();
            }
        }

        if (cfg.enableSp1) {
            if (cfg.sp1Verifier == address(0)) {
                revert InvalidVerifierAddress();
            }
            if (globalConfig.sp1Vkey == bytes32(0)) {
                revert InvalidProgramId();
            }
        }

        if (cfg.enablePico) {
            if (cfg.picoVerifier == address(0)) {
                revert InvalidVerifierAddress();
            }
            if (globalConfig.picoVkey == bytes32(0)) {
                revert InvalidProgramId();
            }
        }
    }

    // ============================================================
    //                      DEPLOYMENT LOGIC
    // ============================================================

    /// @notice Deploys SigstoreAttestationVerifier and configures ZK co-processors
    function _deployAndConfigure(ChainConfig memory cfg)
        internal
        returns (address verifier, DeploymentResult memory result)
    {
        uint256 chainId = block.chainid;

        console2.log("Deploying SigstoreAttestationVerifier on chain", chainId);
        console2.log("  Owner:", cfg.owner);

        SigstoreAttestationVerifier attestationVerifier =
            new SigstoreAttestationVerifier{salt: SIGSTORE_ATTESTATION_VERIFIER_SALT}(cfg.owner);
        verifier = address(attestationVerifier);

        console2.log("  Deployed at:", verifier);

        result.chainId = chainId;
        result.verifier = verifier;

        // Configure RiscZero if enabled
        if (cfg.enableRiscZero) {
            console2.log("  Configuring RiscZero...");
            console2.log("    Verifier:", cfg.riscZeroVerifier);
            console2.log("    Image ID:", vm.toString(globalConfig.riscZeroImageId));

            attestationVerifier.setZkCoProcessorConfig(
                ZkCoProcessorType.RiscZero, globalConfig.riscZeroImageId, cfg.riscZeroVerifier
            );

            result.riscZeroConfigured = true;
        }

        // Configure SP1 (Succinct) if enabled
        if (cfg.enableSp1) {
            console2.log("  Configuring SP1 (Succinct)...");
            console2.log("    Verifier:", cfg.sp1Verifier);
            console2.log("    VKey:", vm.toString(globalConfig.sp1Vkey));

            attestationVerifier.setZkCoProcessorConfig(
                ZkCoProcessorType.Succinct, globalConfig.sp1Vkey, cfg.sp1Verifier
            );

            result.sp1Configured = true;
        }

        // Configure Pico if enabled
        if (cfg.enablePico) {
            console2.log("  Configuring Pico...");
            console2.log("    Verifier:", cfg.picoVerifier);
            console2.log("    VKey:", vm.toString(globalConfig.picoVkey));

            attestationVerifier.setZkCoProcessorConfig(ZkCoProcessorType.Pico, globalConfig.picoVkey, cfg.picoVerifier);

            result.picoConfigured = true;
        }
    }

    // ============================================================
    //                     OUTPUT MANAGEMENT
    // ============================================================

    /// @notice Writes deployment result to a JSON file
    function _writeDeploymentOutput(DeploymentResult memory result) internal {
        string memory chainIdStr = vm.toString(result.chainId);
        string memory outputPath = string.concat(DEPLOYMENTS_DIR, chainIdStr, ".json");

        if (!vm.exists(DEPLOYMENTS_DIR)) {
            vm.createDir(DEPLOYMENTS_DIR, true);
        }

        string memory root = "deployment";
        vm.serializeAddress(root, "SigstoreAttestationVerifier", result.verifier);
        vm.serializeBool(root, "riscZeroConfigured", result.riscZeroConfigured);
        vm.serializeBool(root, "sp1Configured", result.sp1Configured);
        string memory finalJson = vm.serializeBool(root, "picoConfigured", result.picoConfigured);

        vm.writeJson(finalJson, outputPath);
        console2.log("  Output written to:", outputPath);
    }

    /// @notice Prints a summary of all deployments
    function _printSummary() internal view {
        console2.log("");
        console2.log("========================================");
        console2.log("        DEPLOYMENT SUMMARY");
        console2.log("========================================");

        for (uint256 i = 0; i < deploymentResults.length; i++) {
            DeploymentResult memory r = deploymentResults[i];
            console2.log("");
            console2.log("Chain ID:", r.chainId);
            console2.log("  Verifier:", r.verifier);
            console2.log("  RiscZero:", r.riscZeroConfigured ? "Configured" : "Not configured");
            console2.log("  SP1:", r.sp1Configured ? "Configured" : "Not configured");
            console2.log("  Pico:", r.picoConfigured ? "Configured" : "Not configured");
        }

        console2.log("");
        console2.log("========================================");
    }

    // ============================================================
    //                      ENTRY POINTS
    // ============================================================

    /// @notice Multi-chain deployment entry point
    /// @dev Deploys to all chains configured in deployment.toml
    function run() external {
        // Load global config first (program IDs)
        _loadGlobalConfig();

        // Load chain configs and create forks
        _loadConfigAndForks(CONFIG_PATH, true);

        console2.log("");
        console2.log("========================================");
        console2.log("  SIGSTORE ATTESTATION VERIFIER DEPLOY");
        console2.log("========================================");
        console2.log("");

        uint256[] memory chains = config.getChainIds();
        console2.log("Deploying to", chains.length, "chain(s)");
        console2.log("");

        for (uint256 i = 0; i < chains.length; i++) {
            uint256 chainId = chains[i];

            console2.log("----------------------------------------");
            console2.log("Processing chain ID:", chainId);
            console2.log("----------------------------------------");

            vm.selectFork(forkOf[chainId]);

            ChainConfig memory cfg = _loadChainConfig(chainId);
            _validateConfig(cfg);

            vm.startBroadcast();
            (, DeploymentResult memory result) = _deployAndConfigure(cfg);
            vm.stopBroadcast();

            deploymentResults.push(result);
            _writeDeploymentOutput(result);

            console2.log("");
        }

        _printSummary();
    }

    /// @notice Single chain deployment entry point
    /// @dev Use with --rpc-url flag to deploy to a specific chain
    function runSingle() external {
        // Load global config first (program IDs)
        _loadGlobalConfig();

        // Load config without creating forks (use current RPC)
        _loadConfig(CONFIG_PATH, true);

        uint256 chainId = block.chainid;

        console2.log("========================================");
        console2.log("  SINGLE CHAIN DEPLOYMENT");
        console2.log("  Chain ID:", chainId);
        console2.log("========================================");
        console2.log("");

        ChainConfig memory cfg = _loadChainConfig(chainId);
        _validateConfig(cfg);

        vm.startBroadcast();
        (, DeploymentResult memory result) = _deployAndConfigure(cfg);
        vm.stopBroadcast();

        _writeDeploymentOutput(result);

        console2.log("");
        console2.log("Deployment complete!");
    }
}
