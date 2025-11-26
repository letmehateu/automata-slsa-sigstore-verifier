//! Pico zkVM prover implementation
//!
//! Implements the ZkVmProver trait for Pico, providing proof generation
//! capabilities for Sigstore attestation verification.

use crate::config::PicoConfig;
use async_trait::async_trait;
use p3_field::PrimeField;
use pico_sdk::client::{DefaultProverClient, KoalaBearProverClient};
use pico_sdk::HashableKey;
use sigstore_pico_methods::PICO_SIGSTORE_ELF;
use sigstore_zkvm_traits::error::ZkVmError;
use sigstore_zkvm_traits::traits::ZkVmProver;
use sigstore_zkvm_traits::types::ProverInput;

pub struct PicoProver {
    elf: &'static [u8],
}

#[async_trait]
impl ZkVmProver for PicoProver {
    type Config = PicoConfig;

    fn new() -> Result<Self, ZkVmError> {
        Ok(PicoProver {
            elf: PICO_SIGSTORE_ELF,
        })
    }

    async fn prove(
        &self,
        config: &Self::Config,
        input: &ProverInput,
    ) -> Result<(Vec<u8>, Vec<u8>), ZkVmError> {
        // Serialize input to bytes
        let input_bytes = input
            .encode_input()
            .map_err(|e| ZkVmError::InvalidInput(format!("Failed to encode ProverInput: {}", e)))?;

        // Log program identifier
        println!("Program ID: {}", self.program_identifier()?);
        println!("Pico Version: {}", Self::circuit_version());

        // Initialize the prover client
        let client = DefaultProverClient::new(self.elf);

        // Initialize new stdin builder
        let mut stdin_builder = client.new_stdin_builder();
        stdin_builder.write_slice(&input_bytes);

        // Emulate first to get public buffer
        println!("Emulating program...");
        let (reports, public_buffer) = client.emulate(stdin_builder.clone());
        let total_cycles: u64 = reports.iter().map(|r| r.current_cycle).sum();
        println!("Emulation cycles: {}", total_cycles);

        // Generate proof if not in dev mode
        if std::env::var("DEV_MODE").is_err() || std::env::var("DEV_MODE").unwrap().is_empty() {
            println!(
                "Begin proving with Pico zkVM (field: {})",
                config.field_type
            );

            // Check if trusted setup is needed (vm_pk exists)
            let proving_key_path = config.artifacts_path.join("vm_pk");
            let need_setup = !proving_key_path.exists();

            if need_setup {
                println!("Performing trusted setup (first time)...");
            } else {
                println!("Using existing proving key at {:?}", proving_key_path);
            }

            client
                .prove_evm(
                    stdin_builder,
                    need_setup,
                    config.artifacts_path.clone(),
                    &config.field_type,
                )
                .map_err(|e| {
                    ZkVmError::ProofGenerationError(format!("Failed to generate Pico proof: {}", e))
                })?;

            println!("Proof generated successfully");
        } else {
            println!("DEV_MODE enabled, skipping proof generation");
        }

        // Parse the journal (public buffer)
        let journal = public_buffer.to_vec();

        // Read and encode proof from proof.data
        let proof_data_path = config.artifacts_path.join("proof.data");
        let proof_bytes = if proof_data_path.exists() {
            let proof_data = std::fs::read_to_string(&proof_data_path).map_err(|e| {
                ZkVmError::ProofGenerationError(format!("Failed to read proof.data: {}", e))
            })?;

            // Parse comma-separated hex strings
            let hex_strings: Vec<&str> = proof_data.split(',').collect();

            if hex_strings.len() < 8 {
                return Err(ZkVmError::ProofGenerationError(format!(
                    "Invalid proof.data: expected at least 8 values, got {}",
                    hex_strings.len()
                )));
            }

            // Take first 8 values (the proof), last 2 are witness
            let proof_values = &hex_strings[0..8];

            // Encode as uint256[8]: just concatenate 8 * 32 bytes
            let mut encoded = Vec::with_capacity(8 * 32);

            // Concatenate the 8 proof values (each already 32 bytes)
            for hex_str in proof_values {
                let hex_str = hex_str.trim().trim_start_matches("0x");
                let bytes = hex::decode(hex_str).map_err(|e| {
                    ZkVmError::ProofGenerationError(format!(
                        "Failed to decode proof hex string: {}",
                        e
                    ))
                })?;

                if bytes.len() != 32 {
                    return Err(ZkVmError::ProofGenerationError(format!(
                        "Invalid proof value: expected 32 bytes, got {}",
                        bytes.len()
                    )));
                }

                encoded.extend_from_slice(&bytes);
            }

            encoded
        } else {
            println!("proof.data not found, returning empty proof");
            Vec::new()
        };

        Ok((journal, proof_bytes))
    }

    fn program_identifier(&self) -> Result<String, ZkVmError> {
        // Create KoalaBear client to compute VK
        let client = KoalaBearProverClient::new(self.elf);
        let vk = client.riscv_vk();
        let vk_digest_bn254 = vk.hash_bn254();

        // Convert to bytes using PrimeField trait
        let vk_bytes = vk_digest_bn254.as_canonical_biguint().to_bytes_be();

        // Pad to 32 bytes (vk_bytes is typically 31 bytes)
        let mut result = [0u8; 32];
        let offset = 32 - vk_bytes.len();
        result[offset..].copy_from_slice(&vk_bytes);

        // Return as hex string
        Ok(format!("0x{}", hex::encode(result)))
    }

    fn circuit_version() -> String {
        // As specified in https://github.com/brevis-network/pico/blob/main/Cargo.toml
        "v1.1.8".to_string()
    }

    fn elf(&self) -> &'static [u8] {
        self.elf
    }
}
