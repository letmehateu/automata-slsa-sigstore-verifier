# Sigstore Verifier

A Rust library for verifying software build attestations according to the Sigstore specification.

## Features

- Verifies Sigstore bundles (format v0.3+)
- Supports both GitHub Fulcio and public Sigstore instances
- Validates DSSE envelope signatures with ECDSA (P-256, P-384)
- Verifies certificate chains (user must provide trust bundles)
- Supports RFC 3161 timestamps with full TSA chain verification
- Supports Rekor integrated time with Merkle tree inclusion proof verification
- Extracts and validates OIDC identity from certificate extensions
- Returns SHA-256 hashes of the entire certificate chain
- Optional trust bundle fetcher utility (behind `fetcher` feature flag)

## Verification Workflow

The library performs the following verification steps in order:

1. **Subject Digest Validation**: Checks that the attestation subject digest is not zero and optionally matches an expected value
2. **Timestamp Extraction**: Extracts signing time from either RFC 3161 timestamps OR Rekor integrated time (mutually exclusive)
3. **Certificate Chain Verification**: Verifies the entire chain from leaf → intermediates → root, ensuring each certificate is signed by its parent and the root is self-signed
4. **Signing Time Validation**: Verifies the signing time falls within the certificate's validity period
5. **DSSE Signature Verification**: Verifies the DSSE envelope signature using the public key from the leaf certificate
6. **Timestamp Mechanism Verification**:
   - For RFC 3161: Verifies TSA certificate chain, Extended Key Usage, message imprint, and PKCS#7 signature
   - For Rekor: Verifies Merkle tree inclusion proof
7. **OIDC Identity Extraction**: Extracts and optionally validates OIDC identity from certificate extensions

## Usage

### Basic Example

```rust
use std::path::Path;
use sigstore_verifier::{AttestationVerifier, VerificationOptions, CertificateChain};

let verifier = AttestationVerifier;

// Prepare Fulcio certificate chain
let fulcio_ca_chain = CertificateChain {
    leaf: leaf_cert_der,
    intermediates: vec![intermediate_cert_der],
    root: root_cert_der,
};

// Prepare TSA chain if using RFC 3161 timestamps (optional)
let tsa_chain = Some(CertificateChain {
    leaf: tsa_leaf_der,
    intermediates: vec![],
    root: tsa_root_der,
});

let options = VerificationOptions {
    expected_digest: None,
    expected_issuer: Some("https://token.actions.githubusercontent.com".to_string()),
    expected_subject: Some("repo:owner/repo:ref:refs/heads/main".to_string()),
};

let result = verifier.verify_bundle(
    Path::new("path/to/bundle.sigstore.json"),
    &options,
    &felco_ca_chain,
    tsa_chain.as_ref(),
)?;

println!("Leaf cert hash: {}", hex::encode(&result.certificate_hashes.leaf));
println!("Root cert hash: {}", hex::encode(&result.certificate_hashes.root));
println!("Signing time: {}", result.signing_time);
println!("Subject digest: {}", hex::encode(&result.subject_digest));

if let Some(identity) = &result.oidc_identity {
    println!("OIDC Issuer: {}", identity.issuer);
    println!("OIDC Subject: {}", identity.subject);
}
```

### Using the Trust Bundle Fetcher (Optional)

Enable the `fetcher` feature in `Cargo.toml`:

```toml
[dependencies]
sigstore-verifier = { version = "0.1", features = ["fetcher"] }
```

Then use the fetcher to download trust bundles:

```rust
use sigstore_verifier::fetcher::fetch_fulcio_trust_bundle;
use sigstore_verifier::types::certificate::FulcioInstance;

// Fetch from GitHub Fulcio instance
let felco_ca_chain = fetch_fulcio_trust_bundle(FulcioInstance::Github).await?;

// Or from public Sigstore instance
let felco_ca_chain = fetch_fulcio_trust_bundle(FulcioInstance::PublicGood).await?;
```

### Fetching Trust Bundles Directly From Github (Optional)

Install the [GitHub CLI](https://github.com/cli/cli#installation), and run:

```bash
gh attestation trusted-root > path/to/trusted_root.jsonl
```

Then, load the JSONL trust bundle:

```rust
use sigstore_verifier::fetcher::jsonl::parser::{
    load_trusted_root_from_jsonl, select_certificate_authority
};
use sigstore_verifier::types::certificate::FulcioInstance;

let trusted_root_path = "path/to/trusted_root.jsonl";
let trusted_root_content = std::fs::read_to_string(&trusted_root_path)?;
let trust_roots = load_trusted_root_from_jsonl(&trusted_root_content)?;

let fulcio_instance = FulcioInstance::Github; // Or PublicGood
let felco_ca_chain = select_certificate_authority(&trust_roots, &fulcio_instance, timestamp)?;
// Set `None` for FulcioInstance::PublicGood
let tsa_chain = select_timestamp_authority(&trust_roots, &fulcio_instance, timestamp)?;
```

## Return Value

On successful verification, the library returns a `VerificationResult` containing:

```rust
pub struct VerificationResult {
    pub certificate_hashes: CertificateChainHashes,
    pub signing_time: DateTime<Utc>,
    pub subject_digest: Vec<u8>,
    pub oidc_identity: Option<OidcIdentity>,
}

pub struct CertificateChainHashes {
    pub leaf: [u8; 32],
    pub intermediates: Vec<[u8; 32]>,
    pub root: [u8; 32],
}
```

The certificate hashes can be used to verify the trust chain and track which certificates were used for signing.

## Supported Signature Algorithms

### DSSE Envelope Signatures
- ECDSA with secp256r1 (P-256)
- ECDSA with secp384r1 (P-384)

### RFC 3161 Timestamp Signatures
- RSA with SHA-256
- RSA with SHA-384
- ECDSA with P-256
- ECDSA with P-384

The library automatically detects the signature algorithm from the certificate's Subject Public Key Info.

## OIDC Identity Verification

The library can extract and validate the following OIDC identity fields from certificate extensions:

- **Issuer**: Extracted from OID 1.3.6.1.4.1.57264.1.8
- **Subject**: Extracted from Subject Alternative Name (SAN)
- **Repository URI**: Extracted from OID 1.3.6.1.4.1.57264.1.12 (optional)
- **Workflow Reference**: Extracted from OID 1.3.6.1.4.1.57264.1.14 (optional)
- **Event Name**: Extracted from legacy GitHub OID 1.3.6.1.4.1.57264.1.2 (optional)

You can optionally validate the expected issuer and subject by setting `expected_issuer` and `expected_subject` in `VerificationOptions`.

## Limitations

- **Certificate revocation checking**: Not implemented (no CRL or OCSP validation)
- **SCT verification**: Not implemented (no Signed Certificate Timestamp validation)
- **Rekor signed entry timestamp verification**: Entry existence is checked but signature validation is not fully implemented
- **RSA DSSE signatures**: Only ECDSA (P-256, P-384) is supported for DSSE envelope signatures
- **Single signature verification**: Only the first signature in the DSSE envelope is verified
- **Ed25519 support**: Not implemented, limited to ECDSA curves
- **Embedded TSA certificate extraction**: While supported, some RFC 3161 timestamp responses may require external TSA chains

## Testing

Run the test suite:

```bash
cargo test --package sigstore-verifier
```

Run integration tests (requires network access):

```bash
cargo test --package sigstore-verifier -- --ignored
```

## License

See the repository root for license information.
