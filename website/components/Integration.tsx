import React, { useState } from 'react';
import CodeBlock from './ui/CodeBlock';
import { Code2 } from 'lucide-react';

const Integration: React.FC = () => {
  const [lang, setLang] = useState<'rust' | 'solidity'>('rust');

  return (
    <section id="integration" className="py-16 md:py-24 border-b border-zinc-900 bg-black overflow-hidden">
      <div className="max-w-7xl mx-auto px-4 md:px-6">

        <div className="flex items-center gap-2 md:gap-3 mb-6 md:mb-8 text-orange-500">
          <Code2 className="w-6 h-6 md:w-8 md:h-8 flex-shrink-0" />
          <h2 className="text-2xl md:text-4xl font-bold text-white">Code Integration</h2>
        </div>

        <div className="flex gap-1 border-b border-zinc-800 mb-6 md:mb-8">
          <button
            onClick={() => setLang('rust')}
            className={`px-4 md:px-8 py-2 md:py-3 text-sm font-medium transition-all ${
              lang === 'rust'
                ? 'bg-zinc-900 text-white border-t border-x border-zinc-800 rounded-t'
                : 'text-zinc-500 hover:text-zinc-300'
            }`}
          >
            Rust
          </button>
          <button
            onClick={() => setLang('solidity')}
            className={`px-4 md:px-8 py-2 md:py-3 text-sm font-medium transition-all ${
              lang === 'solidity'
                ? 'bg-zinc-900 text-white border-t border-x border-zinc-800 rounded-t'
                : 'text-zinc-500 hover:text-zinc-300'
            }`}
          >
            Solidity
          </button>
        </div>

        <div className="min-h-[400px] md:min-h-[500px]">
          {lang === 'rust' ? (
             <div className="animate-in fade-in duration-300">
               <div className="mb-6 md:mb-8">
                 <h3 className="text-lg md:text-xl text-white mb-2">Rust Verifier Library</h3>
                 <p className="text-sm md:text-base text-zinc-400 mb-4">Add the verifier library to your Cargo.toml and use it in your Rust project.</p>
                 <CodeBlock
                  language="toml"
                  title="Cargo.toml"
                  code={`[dependencies]
sigstore-verifier = { git = "https://github.com/automata-network/automata-slsa-sigstore-verifier" }`}
                 />
               </div>
               <div>
                 <h3 className="text-lg md:text-xl text-white mb-2">Usage Example</h3>
                 <CodeBlock
                  language="rust"
                  title="main.rs"
                  code={`use std::path::Path;
use sigstore_verifier::AttestationVerifier;
use sigstore_verifier::fetcher::jsonl::parser::{
    load_trusted_root_from_jsonl, select_certificate_authority, select_timestamp_authority,
};
use sigstore_verifier::types::certificate::CertificateChain;
use sigstore_verifier::types::certificate::FulcioInstance;
use sigstore_verifier::types::result::VerificationOptions;

fn main() {
    let verifier = AttestationVerifier::new();

    // Load your trust bundle (Fulcio CA chain)
    let trust_roots_content = fs::read_to_string("/path/to/trusted-root.jsonl");
    let trust_roots = load_trusted_root_from_jsonl(&trust_roots_content)
        .expect("Failed to parse trusted root JSONL");

    // Auto-detect Fulcio instance from bundle
    let bundle_path = "path/to/bundle.json";
    let bundle_json = std::fs::read_to_string(&bundle_path).expect("Failed to read bundle");
    let fulcio_instance =
        FulcioInstance::from_bundle_json(&bundle_json).expect("Failed to detect Fulcio instance");

    // Parse the Sigstore bundle
    let bundle = parse_bundle_from_path(&path).expect("Failed to parse bundle");

    // Extract timestamp from the bundle
    let timestamp = extract_bundle_timestamp(&bundle).expect("Failed to extract timestamp");

    let fulcio_chain = select_certificate_authority(&trust_roots, &fulcio_instance, timestamp)
        .expect("Failed to select certificate authority");
    let tsa_chain = select_timestamp_authority(&trust_roots, &fulcio_instance, timestamp)
        .expect("Failed to select timestamp authority");

    let result = verifier.verify_bundle(
        Path::new(&bundle_path),
        VerificationOptions::default(),
        &fulcio_chain,
        Some(&tsa_chain), // Optional TSA chain for RFC 3161
    ).expect("verification failed");

    // Access verified OIDC identity
    if let Some(ref oidc) = result.oidc_identity {
        println!("Repository: {:?}", oidc.repository);
        println!("Issuer: {:?}", oidc.issuer);
    }
}`}
                 />
               </div>
             </div>
          ) : (
            <div className="animate-in fade-in duration-300">
              <div className="mb-6 md:mb-8">
                 <h3 className="text-lg md:text-xl text-white mb-2">Solidity Verifier Contracts</h3>
                 <p className="text-sm md:text-base text-zinc-400 mb-4">Install the contracts package and configure the remapping in your Foundry project.</p>
                 <CodeBlock
                  language="bash"
                  title="Terminal"
                  code={`forge install automata-network/automata-slsa-sigstore-verifier`}
                 />
               </div>
               <div className="mb-6 md:mb-8">
                 <h3 className="text-lg md:text-xl text-white mb-2">Remapping Configuration</h3>
                 <CodeBlock
                  language="text"
                  title="remappings.txt"
                  code={`@automata-network/automata-slsa-sigstore-verifier/=lib/automata-slsa-sigstore-verifier/contracts/src/`}
                 />
               </div>
               <div>
                 <h3 className="text-lg md:text-xl text-white mb-2">Usage Example</h3>
                 <CodeBlock
                  language="solidity"
                  title="MyContract.sol"
                  code={`// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import {
    ISigstoreAttestationVerifier,
    ZkCoProcessorType
} from "@automata-network/automata-slsa-sigstore-verifier/interfaces/ISigstoreAttestationVerifier.sol";
import {
    VerificationResult,
    VerificationResultParser
} from "@automata-network/automata-slsa-sigstore-verifier/Types.sol";

contract MyContract {
    using VerificationResultParser for bytes;

    ISigstoreAttestationVerifier public verifier;

    constructor(address _verifier) {
        verifier = ISigstoreAttestationVerifier(_verifier);
    }

    function verifyAttestation(
        bytes calldata output,
        ZkCoProcessorType zkType,
        bytes calldata proofBytes
    ) external {
        // Verify the ZK proof and get the parsed result
        VerificationResult memory result = verifier.verifyAndAttestWithZKProof(
            output, zkType, proofBytes
        );

        // Access verified OIDC identity and artifact data
        string memory repo = result.oidcRepository;
        string memory issuer = result.oidcIssuer;
        bytes memory digest = result.subjectDigest;
        uint64 timestamp = result.timestamp;

        // Use the verified data for access control, governance, etc.
    }

    function parseRawOutput(bytes memory data) external pure {
        // Parse raw verification output bytes directly
        VerificationResult memory result = data.parseVerificationResultBytes();
    }
}`}
                 />
               </div>
            </div>
          )}
        </div>

      </div>
    </section>
  );
};

export default Integration;
