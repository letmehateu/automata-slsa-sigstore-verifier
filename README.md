# Automata SLSA Attestation with Sigstore

A Rust-based verification system for [Sigstore](https://www.sigstore.dev/) attestation bundles that generates zero-knowledge proofs using multiple zkVM backends (RISC0, SP1, and Pico). This enables trustless on-chain verification of software build provenance.

## Overview

This project provides cryptographic verification of SLSA (Supply-chain Levels for Software Artifacts) attestations signed via Sigstore. The verification process validates:

- **DSSE Envelope Signatures** - Cryptographic signatures over the attestation payload
- **X.509 Certificate Chains** - Fulcio-issued certificates linking to trusted roots
- **RFC 3161 Timestamps** - Timestamp authority proofs establishing signing time
- **Rekor Transparency Logs** - Inclusion proofs in the immutable transparency log
- **OIDC Identity Claims** - Issuer and subject identity from the signing certificate

The verified result can be proven in zero-knowledge using any of three supported zkVM backends, enabling on-chain verification without revealing the full attestation data.

### Supported Sigstore Instances

| Instance | Fulcio CA | Use Case |
|----------|-----------|----------|
| GitHub Actions | `fulcio.githubapp.com` | Artifact attestations from GitHub Actions workflows |
| Public Good | `fulcio.sigstore.dev` | General-purpose Sigstore signing |

## Project Structure

```
automata-attest-build-verifier/
├── crates/
│   ├── sigstore-verifier/       # Core verification library
│   ├── sigstore-zkvm-traits/    # Trait abstractions for zkVM provers
│   ├── sp1-host/                # SP1 zkVM host (CLI + prover)
│   ├── sp1/                     # SP1 guest program
│   ├── risc0-host/              # RISC0 zkVM host (CLI + prover)
│   ├── risc0/                   # RISC0 guest program
│   ├── pico-host/               # Pico zkVM host (CLI + prover)
│   └── pico/                    # Pico guest program
├── contracts/                   # Solidity contracts for on-chain verification
└── samples/                     # Example attestation bundles and trusted roots
```

### Crate Descriptions

| Crate | Description |
|-------|-------------|
| `sigstore-verifier` | Standalone library for parsing and verifying Sigstore attestation bundles. Handles certificate validation, signature verification, timestamp proofs, and transparency log inclusion. |
| `sigstore-zkvm-traits` | Defines the `ZkVmProver` trait and common types (`ProverInput`, `ProofArtifact`) shared across all zkVM implementations. |
| `sp1-host` | Host program and CLI for generating proofs using the SP1 zkVM. Supports compressed, Groth16, and Plonk proof modes via SP1 Network. |
| `sp1` | Guest program that runs inside SP1, executing the attestation verification logic. |
| `risc0-host` | Host program and CLI for generating proofs using RISC0 zkVM. Supports proving via Boundless network. |
| `risc0` | Guest program that runs inside RISC0, executing the attestation verification logic. |
| `pico-host` | Host program and CLI for generating proofs using Pico zkVM. Supports KoalaBear and BabyBear field types. |
| `pico` | Guest program that runs inside Pico, executing the attestation verification logic. |

## Commands

### Getting Program Identifiers

Each zkVM has a unique program identifier needed for on-chain verification:

```bash
# SP1 - Get verifying key hash
cargo run -p sp1-host -- verifying-key

# RISC0 - Get image ID
cargo run -p risc0-host -- image-id

# Pico - Get program ID
cargo run -p pico-host -- program-id
```

### Generating Proofs

#### SP1

```bash
cargo run -p sp1-host -- prove \
    --bundle <BUNDLE_PATH> \
    --trust-roots <TRUSTED_ROOT_PATH> \
    --output <OUTPUT_PATH> \
    --network-private-key <HEX_PRIVATE_KEY> \
    --mode groth16
```

**Options:**
- `--mode`: `compressed`, `groth16`, or `plonk` (use `groth16` for on-chain verification)
- `--network-private-key`: SP1 Network wallet key (or set `SP1_NETWORK_PRIVATE_KEY` env var)

#### RISC0

```bash
cargo run -p risc0-host -- prove \
    --bundle <BUNDLE_PATH> \
    --trust-roots <TRUSTED_ROOT_PATH> \
    --output <OUTPUT_PATH> \
    prove boundless \
        --boundless-rpc-url <RPC_URL> \
        --boundless-private-key <HEX_PRIVATE_KEY>
```

**Options:**
- `--boundless-rpc-url`: Boundless proving network RPC endpoint
- `--boundless-private-key`: Wallet key for Boundless network
- `--proof-type`: `groth16` or `merkle` (optional)
- `--min-price`, `--max-price`: Price bounds in wei (optional)
- `--timeout`: Proof generation timeout in seconds (optional)

#### Pico

```bash
cargo run -p pico-host -- prove \
    --bundle <BUNDLE_PATH> \
    --trust-roots <TRUSTED_ROOT_PATH> \
    --artifacts <ARTIFACTS_DIR> \
    --field-type kb \
    --output <OUTPUT_PATH>
```

**Options:**
- `--artifacts`: Directory for proof artifacts (created if doesn't exist)
- `--field-type`: `kb` (KoalaBear, default) or `bb` (BabyBear)

### Development Mode

For testing without generating real proofs, set the dev mode environment variable:

```bash
# SP1
SP1_DEV_MODE=1 cargo run -p sp1-host -- prove ...

# RISC0
RISC0_DEV_MODE=1 cargo run -p risc0-host -- prove ...
```

### Example with Sample Data

```bash
# Verify a GitHub Actions attestation bundle
cargo run -p sp1-host -- prove \
    --bundle samples/example.sigstore.json \
    --trust-roots samples/trusted_root.jsonl \
    --output proof.json \
    --network-private-key $SP1_NETWORK_PRIVATE_KEY \
    --mode groth16
```

## Code Integration

### Adding Dependencies

```toml
[dependencies]
sigstore-verifier = { git = "https://github.com/automata-network/automata-slsa-sigstore-verifier" }

# For zkVM proof generation (choose one or more):
sigstore-zkvm-traits = { git = "https://github.com/automata-network/automata-slsa-sigstore-verifier" }
sp1-host = { git = "https://github.com/automata-network/automata-slsa-sigstore-verifier" }
risc0-host = { git = "https://github.com/automata-network/automata-slsa-sigstore-verifier" }
pico-host = { git = "https://github.com/automata-network/automata-slsa-sigstore-verifier" }
```

### Basic Verification

```rust
use sigstore_verifier::{
    AttestationVerifier,
    types::{certificate::CertificateChain, result::VerificationOptions},
};
use std::path::Path;

fn verify_attestation(
    bundle_path: &Path,
    trust_bundle: &CertificateChain,
    tsa_chain: Option<&CertificateChain>,
) -> Result<(), Box<dyn std::error::Error>> {
    let verifier = AttestationVerifier::new();

    let options = VerificationOptions {
        expected_digest: None,
        expected_issuer: None,
        expected_subject: None,
    };

    let result = verifier.verify_bundle(
        bundle_path,
        options,
        trust_bundle,
        tsa_chain,
    )?;

    println!("Verification succeeded!");
    println!("Signing time: {}", result.signing_time);
    println!("Subject digest: {}", hex::encode(&result.subject_digest));

    if let Some(identity) = &result.oidc_identity {
        println!("OIDC Issuer: {:?}", identity.issuer);
        println!("OIDC Subject: {:?}", identity.subject);
    }

    Ok(())
}
```

### Generating ZK Proofs

```rust
use sigstore_zkvm_traits::{
    traits::ZkVmProver,
    workflow::prepare_guest_input_local,
    utils::write_proof_artifact,
};
use sp1_host::{prover::Sp1Prover, config::{Sp1Config, ProvingMode}};
use std::path::Path;

async fn generate_sp1_proof(
    bundle_path: &Path,
    trust_root_path: &Path,
    output_path: &Path,
    private_key: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // Prepare input from files
    let prover_input = prepare_guest_input_local(
        bundle_path,
        trust_root_path,
        Default::default(),
    )?;

    // Create prover and config
    let prover = Sp1Prover::new()?;
    let config = Sp1Config {
        proving_mode: ProvingMode::Groth16,
        private_key: private_key.to_string(),
    };

    // Generate proof
    let (journal, proof_bytes) = prover.prove(&config, &prover_input).await?;

    // Create proof artifact
    let artifact = sigstore_zkvm_traits::utils::create_proof_artifact(
        "sp1",
        &prover.program_identifier()?,
        &Sp1Prover::circuit_version(),
        &journal,
        &proof_bytes,
    );

    // Write to file
    write_proof_artifact(output_path, &artifact)?;

    Ok(())
}
```

### Key Types

```rust
// Verification input options
pub struct VerificationOptions {
    pub expected_digest: Option<String>,   // Expected artifact digest
    pub expected_issuer: Option<String>,   // Expected OIDC issuer
    pub expected_subject: Option<String>,  // Expected OIDC subject
}

// Verification output
pub struct VerificationResult {
    pub signing_time: DateTime<Utc>,
    pub subject_digest: Vec<u8>,
    pub subject_digest_algorithm: DigestAlgorithm,
    pub certificate_hashes: CertificateChainHashes,
    pub oidc_identity: Option<OidcIdentity>,
    pub timestamp_proof: Option<TimestampProof>,
}

// Proof artifact for on-chain submission
pub struct ProofArtifact {
    pub zkvm: String,           // "risc0", "sp1", or "pico"
    pub program_id: String,     // Program identifier for the zkVM
    pub circuit_version: String,
    pub journal: String,        // Hex-encoded public output
    pub proof: String,          // Hex-encoded proof bytes
}
```

## Learn More

For comprehensive documentation, tutorials, and API references, visit the project site:

**[https://www.proofofbuild.xyz/](https://www.proofofbuild.xyz/)**

## License

MIT
