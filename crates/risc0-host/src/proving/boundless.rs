//! Boundless network proving integration
//!
//! Provides functionality to generate proofs using the Boundless proving network.

use crate::cli::BoundlessProofType;
use crate::config::BoundlessConfig;
use anyhow::{Context, Result};
use boundless_market::{
    alloy::{
        primitives::U256,
        providers::{Provider, ProviderBuilder},
        signers::local::PrivateKeySigner,
        transports::http::reqwest::Url,
    },
    client::Client,
    request_builder::OfferParams,
    storage::storage_provider_from_env,
    Deployment,
};
use std::time::Duration;

/// Generate a proof using the Boundless proving network
///
/// # Arguments
///
/// * `elf` - The guest program ELF binary
/// * `input_bytes` - Serialized input data for the guest program
/// * `config` - Boundless configuration (RPC URL, private key, etc.)
///
/// # Returns
///
/// Returns the proof seal bytes on success.
///
/// # Errors
///
/// Returns an error if:
/// - RPC URL or private key is missing/invalid
/// - Boundless deployment is not found for the chain
/// - Proof request submission fails
/// - Proof generation times out
pub async fn prove_with_boundless(
    elf: &'static [u8],
    input_bytes: &[u8],
    config: &BoundlessConfig,
) -> Result<Vec<u8>> {
    println!("🔗 Connecting to Boundless network...");

    // Parse RPC URL and get chain ID
    let rpc_url_parsed: Url = config
        .rpc_url
        .parse()
        .context("Failed to parse Boundless RPC URL")?;

    // Build provider and get chain ID
    let provider = ProviderBuilder::new()
        .connect_http(rpc_url_parsed.clone());

    let chain_id = provider
        .get_chain_id()
        .await
        .context("Failed to get chain ID from RPC")?;

    println!("📡 Connected to chain ID: {}", chain_id);

    // Get deployment for chain
    let deployment = Deployment::from_chain_id(chain_id).with_context(|| {
        format!(
            "No Boundless deployment found for chain {}. Is this a supported network?",
            chain_id
        )
    })?;

    // Parse private key
    let private_key_bytes = hex::decode(&config.private_key)
        .context("Failed to decode private key (must be hex-encoded)")?;

    let private_key = PrivateKeySigner::from_slice(&private_key_bytes)
        .context("Failed to parse private key")?;

    println!("💰 Wallet address: {:?}", private_key.address());

    // Get storage provider from environment
    let storage_provider = storage_provider_from_env()
        .context("Failed to get storage provider from environment (check BOUNDLESS_STORAGE_* env vars)")?;

    println!("🔑 Building Boundless client...");

    // Build client
    let client = Client::builder()
        .with_rpc_url(rpc_url_parsed)
        .with_deployment(deployment)
        .with_storage_provider(Some(storage_provider))
        .with_private_key(private_key)
        .build()
        .await
        .context("Failed to build Boundless client")?;

    println!("📝 Creating proof request...");

    // Build request
    let mut request_builder = client.new_request().with_stdin(input_bytes);

    // Set program (either URL or ELF)
    if let Some(ref program_url) = config.program_url {
        println!("📦 Using program URL: {}", program_url);
        request_builder = request_builder
            .with_program_url(program_url.as_str())
            .context("Failed to set program URL")?;
    } else {
        println!("📦 Using embedded ELF ({} bytes)", elf.len());
        request_builder = request_builder.with_program(elf.to_vec());
    }

    // Set proof type
    match config.proof_type {
        BoundlessProofType::Groth16 => {
            println!("🔐 Proof type: Groth16");
            request_builder = request_builder.with_groth16_proof();
        }
        BoundlessProofType::Merkle => {
            println!("🌳 Proof type: Merkle");
            // Merkle is the default, no special flag needed
        }
    }

    // Set offer params if any are provided
    if config.min_price.is_some()
        || config.max_price.is_some()
        || config.timeout.is_some()
        || config.ramp_up_period.is_some()
    {
        let mut offer_builder = OfferParams::builder();

        if let Some(min_price) = config.min_price {
            println!("💰 Min price: {} wei", min_price);
            offer_builder.min_price(U256::from(min_price));
        }

        if let Some(max_price) = config.max_price {
            println!("💰 Max price: {} wei", max_price);
            offer_builder.max_price(U256::from(max_price));
        }

        if let Some(timeout) = config.timeout {
            println!("⏱️  Timeout: {} seconds", timeout);
            offer_builder.timeout(timeout);
        }

        if let Some(ramp_up_period) = config.ramp_up_period {
            println!("📈 Ramp-up period: {} seconds", ramp_up_period);
            offer_builder.ramp_up_period(ramp_up_period);
        }

        request_builder = request_builder.with_offer(offer_builder);
    }

    println!("🚀 Submitting proof request to Boundless...");

    // Submit request
    let (request_id, expires_at) = client
        .submit_onchain(request_builder)
        .await
        .map_err(|e| anyhow::anyhow!("Failed to submit proof request to Boundless: {:?}", e))?;

    println!("✓ Request submitted! ID: {:x}", request_id);
    println!("⏳ Waiting for proof generation...");

    // Wait for fulfillment
    let fulfillment = client
        .wait_for_request_fulfillment(request_id, Duration::from_secs(5), expires_at)
        .await
        .context("Failed to wait for proof fulfillment")?;

    println!("✓ Proof generated successfully!");

    Ok(fulfillment.seal.to_vec())
}
