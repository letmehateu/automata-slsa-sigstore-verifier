//! RISC0 zkVM prover implementation
//!
//! Implements the ZkVmProver trait for RISC0, providing proof generation
//! capabilities for Sigstore attestation verification.

use crate::config::{ProvingStrategy, Risc0Config};
use crate::proving::boundless::prove_with_boundless;
use async_trait::async_trait;
use risc0_zkvm::{compute_image_id, default_executor, ExecutorEnv};
use sigstore_risc0_methods::SIGSTORE_RISC0_GUEST_ELF;
use sigstore_zkvm_traits::error::ZkVmError;
use sigstore_zkvm_traits::traits::ZkVmProver;
use sigstore_zkvm_traits::types::ProverInput;

pub struct Risc0Prover {
    elf: &'static [u8],
}

#[async_trait]
impl ZkVmProver for Risc0Prover {
    type Config = Risc0Config;

    fn new() -> Result<Self, ZkVmError> {
        Ok(Risc0Prover {
            elf: SIGSTORE_RISC0_GUEST_ELF,
        })
    }

    async fn prove(
        &self,
        config: &Self::Config,
        input: &ProverInput,
    ) -> Result<(Vec<u8>, Vec<u8>), ZkVmError> {
        // Serialize input to bytes
        let input_bytes = input.encode_input()
            .map_err(|e| ZkVmError::InvalidInput(format!("Failed to encode ProverInput: {}", e)))?;

        // Log image ID
        let image_id = compute_image_id(self.elf)
            .map_err(|e| ZkVmError::ProofGenerationError(format!("Failed to compute image ID: {}", e)))?;
        println!("Image ID: {}", image_id.to_string());
        println!("RISC0 Version: {}", Self::circuit_version());

        // Execute locally to get journal
        let env = ExecutorEnv::builder()
            .write_slice(&input_bytes)
            .build()
            .map_err(|e| ZkVmError::ProofGenerationError(format!("Failed to build executor env: {}", e)))?;

        let session_info = default_executor()
            .execute(env, self.elf)
            .map_err(|e| ZkVmError::ProofGenerationError(format!("Failed to execute guest program: {}", e)))?;

        let journal = session_info.journal.bytes.to_vec();

        // Check for DEV_MODE
        if std::env::var("DEV_MODE").is_ok() || std::env::var("RISC0_DEV_MODE").is_ok() {
            println!("⚠ Running in DEV_MODE - no proof will be generated");
            return Ok((journal, vec![]));
        }

        // Generate proof based on strategy
        let seal = match config.proving_strategy {
            ProvingStrategy::Local => {
                return Err(ZkVmError::ProofGenerationError(
                    "Local proving is not yet supported. Use Boundless or set DEV_MODE=1 for testing.".to_string()
                ));
            }
            ProvingStrategy::Boundless => {
                let boundless_config = config.boundless.as_ref()
                    .ok_or_else(|| ZkVmError::InvalidInput("Boundless config required".to_string()))?;

                prove_with_boundless(self.elf, &input_bytes, boundless_config)
                    .await
                    .map_err(|e| ZkVmError::ProofGenerationError(format!("Boundless proving failed: {}", e)))?
            }
        };

        Ok((journal, seal))
    }

    fn program_identifier(&self) -> Result<String, ZkVmError> {
        let image_id = compute_image_id(self.elf)
            .map_err(|e| ZkVmError::ProofGenerationError(format!("Failed to compute image ID: {}", e)))?;
        Ok(image_id.to_string())
    }

    fn circuit_version() -> String {
        risc0_zkvm::VERSION.to_string()
    }

    fn elf(&self) -> &'static [u8] {
        self.elf
    }
}
