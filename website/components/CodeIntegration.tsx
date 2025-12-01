import React, { useState } from 'react';
import { Code2 } from 'lucide-react';

const CodeIntegration: React.FC = () => {
  const [activeTab, setActiveTab] = useState<'rust' | 'solidity'>('rust');

  return (
    <div className="py-10 space-y-8">
      <div className="mb-8">
        <h3 className="text-2xl font-bold text-white mb-3 flex items-center gap-2">
          <Code2 className="w-6 h-6 text-orange-400" />
          Code Integration
        </h3>
        <p className="text-slate-400">
          Integrate the verifier into your Rust or Solidity projects.
        </p>
      </div>

      {/* Language Tabs */}
      <div className="flex gap-2 mb-6">
        <button
          onClick={() => setActiveTab('rust')}
          className={`px-4 py-2 rounded-lg font-medium transition-colors ${
            activeTab === 'rust'
              ? 'bg-orange-600 text-white'
              : 'bg-slate-800 text-slate-300 hover:bg-slate-700'
          }`}
        >
          Rust
        </button>
        <button
          onClick={() => setActiveTab('solidity')}
          className={`px-4 py-2 rounded-lg font-medium transition-colors ${
            activeTab === 'solidity'
              ? 'bg-orange-600 text-white'
              : 'bg-slate-800 text-slate-300 hover:bg-slate-700'
          }`}
        >
          Solidity
        </button>
      </div>

      {/* Rust Tab Content */}
      {activeTab === 'rust' && (
        <div className="space-y-6 animate-fade-in">
          <div className="bg-slate-950/50 rounded-xl p-6 border border-slate-800">
            <h4 className="text-lg font-semibold text-white mb-3">Rust Verifier Library</h4>
            <p className="text-slate-400 text-sm mb-4">
              Add the verifier library to your Cargo.toml and use it in your Rust project.
            </p>
            <div className="bg-slate-900 rounded-lg border border-slate-700 overflow-hidden">
              <div className="px-4 py-2 border-b border-slate-700 text-xs text-slate-400">
                Cargo.toml
              </div>
              <pre className="p-4 text-sm overflow-x-auto">
                <code className="text-orange-300">[dependencies]</code>
                {'\n'}
                <code className="text-slate-300">sigstore-verifier = </code>
                <code className="text-emerald-400">"0.1.0"</code>
              </pre>
            </div>
          </div>

          <div className="bg-slate-950/50 rounded-xl p-6 border border-slate-800">
            <h4 className="text-lg font-semibold text-white mb-3">Usage Example</h4>
            <div className="bg-slate-900 rounded-lg border border-slate-700 overflow-hidden">
              <div className="px-4 py-2 border-b border-slate-700 text-xs text-slate-400">
                main.rs
              </div>
              <pre className="p-4 text-sm overflow-x-auto">
                <code className="text-blue-400">use</code>
                <code className="text-slate-300"> sigstore_verifier::</code>
                <code className="text-amber-300">{'{'}</code>
                <code className="text-slate-300">verify_bundle, TrustRoot, Bundle</code>
                <code className="text-amber-300">{'}'}</code>
                <code className="text-slate-300">;</code>
                {'\n\n'}
                <code className="text-blue-400">fn</code>
                <code className="text-emerald-400"> main</code>
                <code className="text-slate-300">() {'{'}</code>
                {'\n'}
                <code className="text-slate-300">    </code>
                <code className="text-blue-400">let</code>
                <code className="text-slate-300"> bundle = Bundle::from_json(bundle_json);</code>
                {'\n'}
                <code className="text-slate-300">    </code>
                <code className="text-blue-400">let</code>
                <code className="text-slate-300"> trust_root = TrustRoot::from_json(trust_json);</code>
                {'\n\n'}
                <code className="text-slate-300">    </code>
                <code className="text-blue-400">let</code>
                <code className="text-slate-300"> result = verify_bundle(&bundle, &trust_root)</code>
                {'\n'}
                <code className="text-slate-300">        .expect(</code>
                <code className="text-emerald-400">"verification failed"</code>
                <code className="text-slate-300">);</code>
                {'\n\n'}
                <code className="text-slate-500">    // Access verified OIDC identity</code>
                {'\n'}
                <code className="text-slate-300">    println!(</code>
                <code className="text-emerald-400">"Repository: {'{}'}"</code>
                <code className="text-slate-300">, result.oidc_repository);</code>
                {'\n'}
                <code className="text-slate-300">{'}'}</code>
              </pre>
            </div>
          </div>
        </div>
      )}

      {/* Solidity Tab Content */}
      {activeTab === 'solidity' && (
        <div className="space-y-6 animate-fade-in">
          <div className="bg-slate-950/50 rounded-xl p-6 border border-slate-800">
            <h4 className="text-lg font-semibold text-white mb-3">Solidity Verifier Contracts</h4>
            <p className="text-slate-400 text-sm mb-4">
              Install the contracts package and configure the remapping in your Foundry project.
            </p>
            <div className="bg-slate-900 rounded-lg border border-slate-700 overflow-hidden">
              <div className="px-4 py-2 border-b border-slate-700 text-xs text-slate-400">
                Terminal
              </div>
              <pre className="p-4 text-sm overflow-x-auto">
                <code className="text-emerald-400">forge install</code>
                <code className="text-slate-300"> automata-network/automata-attest-build-verifier</code>
              </pre>
            </div>
          </div>

          <div className="bg-slate-950/50 rounded-xl p-6 border border-slate-800">
            <h4 className="text-lg font-semibold text-white mb-3">Remapping Configuration</h4>
            <div className="bg-slate-900 rounded-lg border border-slate-700 overflow-hidden">
              <div className="px-4 py-2 border-b border-slate-700 text-xs text-slate-400">
                remappings.txt
              </div>
              <pre className="p-4 text-sm overflow-x-auto">
                <code className="text-slate-300">@automata-network/automata-attest-build-verifier/=lib/automata-attest-build-verifier/contracts/src/</code>
              </pre>
            </div>
          </div>

          <div className="bg-slate-950/50 rounded-xl p-6 border border-slate-800">
            <h4 className="text-lg font-semibold text-white mb-3">Usage Example</h4>
            <div className="bg-slate-900 rounded-lg border border-slate-700 overflow-hidden">
              <div className="px-4 py-2 border-b border-slate-700 text-xs text-slate-400">
                MyContract.sol
              </div>
              <pre className="p-4 text-sm overflow-x-auto">
                <code className="text-slate-500">// SPDX-License-Identifier: MIT</code>
                {'\n'}
                <code className="text-blue-400">pragma solidity</code>
                <code className="text-slate-300"> ^0.8.0;</code>
                {'\n\n'}
                <code className="text-blue-400">import</code>
                <code className="text-slate-300"> {'{'}</code>
                {'\n'}
                <code className="text-slate-300">    </code>
                <code className="text-amber-300">ISigstoreAttestationVerifier</code>
                <code className="text-slate-300">,</code>
                {'\n'}
                <code className="text-slate-300">    </code>
                <code className="text-amber-300">ZkCoProcessorType</code>
                {'\n'}
                <code className="text-slate-300">{'}'} </code>
                <code className="text-blue-400">from</code>
                <code className="text-emerald-400"> "@automata-network/automata-attest-build-verifier/interfaces/ISigstoreAttestationVerifier.sol"</code>
                <code className="text-slate-300">;</code>
                {'\n'}
                <code className="text-blue-400">import</code>
                <code className="text-slate-300"> {'{'}</code>
                {'\n'}
                <code className="text-slate-300">    </code>
                <code className="text-amber-300">VerificationResult</code>
                <code className="text-slate-300">,</code>
                {'\n'}
                <code className="text-slate-300">    </code>
                <code className="text-amber-300">VerificationResultParser</code>
                {'\n'}
                <code className="text-slate-300">{'}'} </code>
                <code className="text-blue-400">from</code>
                <code className="text-emerald-400"> "@automata-network/automata-attest-build-verifier/Types.sol"</code>
                <code className="text-slate-300">;</code>
                {'\n\n'}
                <code className="text-blue-400">contract</code>
                <code className="text-amber-300"> MyContract </code>
                <code className="text-slate-300">{'{'}</code>
                {'\n'}
                <code className="text-slate-300">    </code>
                <code className="text-blue-400">using</code>
                <code className="text-slate-300"> VerificationResultParser </code>
                <code className="text-blue-400">for</code>
                <code className="text-slate-300"> </code>
                <code className="text-blue-400">bytes</code>
                <code className="text-slate-300">;</code>
                {'\n\n'}
                <code className="text-slate-300">    ISigstoreAttestationVerifier </code>
                <code className="text-blue-400">public</code>
                <code className="text-slate-300"> verifier;</code>
                {'\n\n'}
                <code className="text-slate-300">    </code>
                <code className="text-blue-400">constructor</code>
                <code className="text-slate-300">(</code>
                <code className="text-blue-400">address</code>
                <code className="text-slate-300"> _verifier) {'{'}</code>
                {'\n'}
                <code className="text-slate-300">        verifier = ISigstoreAttestationVerifier(_verifier);</code>
                {'\n'}
                <code className="text-slate-300">    {'}'}</code>
                {'\n\n'}
                <code className="text-slate-300">    </code>
                <code className="text-blue-400">function</code>
                <code className="text-emerald-400"> verifyAttestation</code>
                <code className="text-slate-300">(</code>
                {'\n'}
                <code className="text-slate-300">        </code>
                <code className="text-blue-400">bytes calldata</code>
                <code className="text-slate-300"> output,</code>
                {'\n'}
                <code className="text-slate-300">        </code>
                <code className="text-amber-300">ZkCoProcessorType</code>
                <code className="text-slate-300"> zkType,</code>
                {'\n'}
                <code className="text-slate-300">        </code>
                <code className="text-blue-400">bytes calldata</code>
                <code className="text-slate-300"> proofBytes</code>
                {'\n'}
                <code className="text-slate-300">    ) </code>
                <code className="text-blue-400">external</code>
                <code className="text-slate-300"> {'{'}</code>
                {'\n'}
                <code className="text-slate-500">        // Verify the ZK proof and get the parsed result</code>
                {'\n'}
                <code className="text-slate-300">        </code>
                <code className="text-amber-300">VerificationResult</code>
                <code className="text-slate-300"> </code>
                <code className="text-blue-400">memory</code>
                <code className="text-slate-300"> result = verifier.verifyAndAttestWithZKProof(</code>
                {'\n'}
                <code className="text-slate-300">            output, zkType, proofBytes</code>
                {'\n'}
                <code className="text-slate-300">        );</code>
                {'\n\n'}
                <code className="text-slate-500">        // Access verified OIDC identity and artifact data</code>
                {'\n'}
                <code className="text-slate-300">        </code>
                <code className="text-blue-400">string memory</code>
                <code className="text-slate-300"> repo = result.oidcRepository;</code>
                {'\n'}
                <code className="text-slate-300">        </code>
                <code className="text-blue-400">string memory</code>
                <code className="text-slate-300"> issuer = result.oidcIssuer;</code>
                {'\n'}
                <code className="text-slate-300">        </code>
                <code className="text-blue-400">bytes memory</code>
                <code className="text-slate-300"> digest = result.subjectDigest;</code>
                {'\n'}
                <code className="text-slate-300">        </code>
                <code className="text-blue-400">uint64</code>
                <code className="text-slate-300"> timestamp = result.timestamp;</code>
                {'\n\n'}
                <code className="text-slate-500">        // Use the verified data for access control, governance, etc.</code>
                {'\n'}
                <code className="text-slate-300">    {'}'}</code>
                {'\n\n'}
                <code className="text-slate-300">    </code>
                <code className="text-blue-400">function</code>
                <code className="text-emerald-400"> parseRawOutput</code>
                <code className="text-slate-300">(</code>
                <code className="text-blue-400">bytes memory</code>
                <code className="text-slate-300"> data) </code>
                <code className="text-blue-400">external pure</code>
                <code className="text-slate-300"> {'{'}</code>
                {'\n'}
                <code className="text-slate-500">        // Parse raw verification output bytes directly</code>
                {'\n'}
                <code className="text-slate-300">        </code>
                <code className="text-amber-300">VerificationResult</code>
                <code className="text-slate-300"> </code>
                <code className="text-blue-400">memory</code>
                <code className="text-slate-300"> result = data.parseVerificationResultBytes();</code>
                {'\n'}
                <code className="text-slate-300">    {'}'}</code>
                {'\n'}
                <code className="text-slate-300">{'}'}</code>
              </pre>
            </div>
          </div>
        </div>
      )}
    </div>
  );
};

export default CodeIntegration;
