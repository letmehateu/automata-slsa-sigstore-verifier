import React, { useState } from 'react';
import CodeBlock from './ui/CodeBlock';
import { FileJson, Cpu, Terminal } from 'lucide-react';

const ZkVerification: React.FC = () => {
  const [activeTab, setActiveTab] = useState<'inputs' | 'process' | 'outputs'>('inputs');
  const [vm, setVm] = useState<'risc0' | 'sp1' | 'pico'>('risc0');

  return (
    <section id="verifier" className="py-16 md:py-24 border-b border-zinc-900 bg-zinc-950 relative overflow-hidden">

      {/* Decorative gradient */}
      <div className="absolute top-0 right-0 w-[300px] md:w-[500px] h-[300px] md:h-[500px] bg-orange-900/10 blur-[100px] rounded-full pointer-events-none" />

      <div className="max-w-7xl mx-auto px-4 md:px-6 relative z-10">

        <div className="mb-8 md:mb-12">
          <h2 className="text-2xl md:text-4xl font-bold text-white mb-4 md:mb-6">Zero-Knowledge Verification</h2>
          <p className="text-zinc-400 max-w-3xl leading-relaxed">
            We built a portable Rust library that verifies Sigstore bundles within Zero-Knowledge Virtual Machines (RISC0, SP1, and Pico).
            The zkVM verifier generates Groth16 SNARK proofs enabling <span className="text-orange-500 font-medium">Proof of Provenance</span> on Ethereum.
          </p>
        </div>

        {/* Tab Header */}
        <div className="flex flex-wrap border-b border-zinc-800 mb-6 md:mb-8 -mx-4 md:mx-0 px-4 md:px-0 overflow-x-auto">
          {[
            { id: 'inputs', label: 'Inputs', icon: FileJson },
            { id: 'process', label: 'zkVM', icon: Cpu },
            { id: 'outputs', label: 'Outputs', icon: Terminal },
          ].map((tab) => (
            <button
              key={tab.id}
              onClick={() => setActiveTab(tab.id as any)}
              className={`flex items-center gap-1.5 md:gap-2 px-3 md:px-6 py-3 md:py-4 font-mono text-xs md:text-sm uppercase tracking-wide transition-colors border-b-2 whitespace-nowrap ${
                activeTab === tab.id
                  ? 'border-orange-500 text-white bg-zinc-900/50'
                  : 'border-transparent text-zinc-500 hover:text-zinc-300'
              }`}
            >
              <tab.icon size={14} className="md:w-4 md:h-4" />
              {tab.label}
            </button>
          ))}
        </div>

        {/* Tab Content */}
        <div className="min-h-[400px]">
          
          {/* TAB 1: INPUTS */}
          {activeTab === 'inputs' && (
            <div className="animate-in fade-in slide-in-from-bottom-2 duration-300">
              <div className="mb-8">
                <h3 className="text-xl font-bold text-white mb-2">Required Verification Artifacts</h3>
                <p className="text-zinc-400">
                  The verifier requires two inputs: a Sigstore bundle (v0.3+) and trust roots.
                  Trust roots should be refreshed every 60-90 days using <code className="text-orange-400 bg-zinc-900 px-1">gh attestation trusted-root</code> due to certificate rotation.
                </p>
              </div>
              <div className="grid lg:grid-cols-2 gap-8">
                <div>
                  <h4 className="text-white font-bold mb-2">Input 1: Attestation Bundle</h4>
                  <p className="text-sm text-zinc-500 mb-4">Generated directly by the attest-build-provenance GitHub Action.</p>
                  <CodeBlock
                    language="json"
                    code={`{
  "mediaType": "application/vnd.dev.sigstore.bundle+json;version=0.3",
  "verificationMaterial": { ... }, // RFC 3161 or Rekor Logs
  "dsseEnvelope": {
    "payload": "...", // in-toto statement
    "signatures": [ ... ]
  }
}`}
                  />
                </div>
                <div>
                  <h4 className="text-white font-bold mb-2">Input 2: Trust Roots</h4>
                  <p className="text-sm text-zinc-500 mb-4">Obtained via CLI: <code className="text-orange-400">gh attestation trusted-root</code></p>
                  <CodeBlock
                    language="json"
                    code={`{
  "mediaType": "application/vnd.dev.sigstore.trustedroot+json;version=0.1",
  "certificateAuthorities": [ ... ],
  "tlogs": [ ... ],
  "timestampAuthorities": [ ... ]
}`}
                  />
                </div>
              </div>
            </div>
          )}

          {/* TAB 2: PROCESS */}
          {activeTab === 'process' && (
            <div className="animate-in fade-in slide-in-from-bottom-2 duration-300">

              <div className="mb-6">
                <h3 className="text-xl font-bold text-white mb-2">Supported Environments</h3>
                <p className="text-zinc-400">
                  Our Rust Verifier Library compiles to RISC-V to support zkVMs. All produce Groth16 SNARK proofs for on-chain verification.
                </p>
              </div>

              <div className="flex gap-2 md:gap-4 mb-6 overflow-x-auto pb-2">
                {['risc0', 'sp1', 'pico'].map((v) => (
                  <button
                    key={v}
                    onClick={() => setVm(v as any)}
                    className={`px-3 md:px-4 py-2 text-xs md:text-sm font-mono border whitespace-nowrap ${
                      vm === v
                        ? 'border-white text-white'
                        : 'border-zinc-800 text-zinc-500 hover:border-zinc-600'
                    }`}
                  >
                    {v.toUpperCase()}
                  </button>
                ))}
              </div>

              <div className="bg-zinc-900/30 border border-zinc-800 p-4 md:p-6 mb-6 md:mb-8">
                {vm === 'risc0' && (
                  <CodeBlock
                    language="bash"
                    title="RISC0 Commands"
                    code={`# Build
cargo build -p risc0-host --release

# Get Program ID
cargo run -p risc0-host -- image-id

# Generate Proof
cargo run -p risc0-host --release -- prove --bundle ./attestation.json --trust-roots ./trusted-root.json --output ./output/boundless-proof.json boundless`}
                  />
                )}
                {vm === 'sp1' && (
                  <CodeBlock
                    language="bash"
                    title="SP1 Commands"
                    code={`# Build
cargo build -p sp1-host --release

# Get Program ID
cargo run -p sp1-host -- verifying-key

# Generate Proof
cargo run -p sp1-host --release -- prove --bundle ./attestation.json --trust-roots ./trusted-root.json --output ./output/boundless-proof.json`}
                  />
                )}
                {vm === 'pico' && (
                  <CodeBlock
                    language="bash"
                    title="Pico Commands"
                    code={`# Build
cargo build -p pico-host --release

# Get Program ID
cargo run -p pico-host -- program-id

# Generate Proof
cargo run -p pico-host --release -- prove --bundle ./attestation.json --trust-roots ./trusted-root.json`}
                  />
                )}
              </div>

              <div className="grid md:grid-cols-3 gap-4 md:gap-6 border-t border-zinc-800 pt-6 md:pt-8">
                <div>
                  <div className="text-orange-500 font-mono text-xs mb-2">01 EXECUTION</div>
                  <p className="text-xs md:text-sm text-zinc-400">The Rust verifier runs inside the zkVM guest program. It parses the Sigstore bundle, verifies the DSSE signature, validates the certificate chain, and extracts OIDC identity.</p>
                </div>
                <div>
                  <div className="text-orange-500 font-mono text-xs mb-2">02 PROOF GENERATION</div>
                  <p className="text-xs md:text-sm text-zinc-400">A Groth16 ZK-SNARK proof is generated, proving verification succeeded. The journal contains certificate hashes, OIDC identity, and timestamp data.</p>
                </div>
                <div>
                  <div className="text-orange-500 font-mono text-xs mb-2">03 ON-CHAIN VERIFICATION</div>
                  <p className="text-xs md:text-sm text-zinc-400">The proof is submitted to a smart contract. The contract verifies it and emits an attestation event for downstream logic.</p>
                </div>
              </div>
            </div>
          )}

          {/* TAB 3: OUTPUTS */}
          {activeTab === 'outputs' && (
            <div className="animate-in fade-in slide-in-from-bottom-2 duration-300">
              <div className="mb-6 md:mb-8">
                <h3 className="text-lg md:text-xl font-bold text-white mb-2">Verification Data Structure</h3>
                <p className="text-sm md:text-base text-zinc-400 mb-4 md:mb-6">
                  The VerificationResult contains all cryptographic evidence and identity information.
                </p>
                <div className="overflow-x-auto -mx-4 md:mx-0 px-4 md:px-0">
                  <table className="w-full text-left text-xs md:text-sm border-collapse min-w-[500px]">
                    <thead>
                      <tr className="border-b border-zinc-800 text-zinc-500 font-mono text-xs uppercase">
                        <th className="py-3 pr-4">Category</th>
                        <th className="py-3 pr-4">Key</th>
                        <th className="py-3">Description</th>
                      </tr>
                    </thead>
                    <tbody className="divide-y divide-zinc-800/50 text-zinc-300">
                      {/* Artifact Information */}
                      <tr>
                        <td className="py-3 pr-4 font-semibold text-white">Artifact</td>
                        <td className="py-3 pr-4 font-mono text-orange-400">subject_digest</td>
                        <td className="py-3 text-zinc-400">The hash of the build artifact.</td>
                      </tr>
                      {/* Certificate Chain */}
                      <tr>
                        <td className="py-3 pr-4 font-semibold text-white">Certificate Chain</td>
                        <td className="py-3 pr-4 font-mono text-orange-400">fulcio_certs</td>
                        <td className="py-3 text-zinc-400">Array of SHA256 certificate hashes, leaf first, root last.</td>
                      </tr>
                      {/* OIDC Identity */}
                      <tr>
                        <td className="py-3 pr-4 font-semibold text-white">OIDC Identity</td>
                        <td className="py-3 pr-4 font-mono text-orange-400">oidc_issuer</td>
                        <td className="py-3 text-zinc-400">The identity provider URL (e.g., https://token.actions.githubusercontent.com).</td>
                      </tr>
                      <tr>
                        <td className="py-3 pr-4"></td>
                        <td className="py-3 pr-4 font-mono text-orange-400">workflow_ref</td>
                        <td className="py-3 text-zinc-400">Reference to the workflow file.</td>
                      </tr>
                      <tr>
                        <td className="py-3 pr-4"></td>
                        <td className="py-3 pr-4 font-mono text-orange-400">repository</td>
                        <td className="py-3 text-zinc-400">The repo where the action ran.</td>
                      </tr>
                      <tr>
                        <td className="py-3 pr-4"></td>
                        <td className="py-3 pr-4 font-mono text-orange-400">event_name</td>
                        <td className="py-3 text-zinc-400">Trigger event (e.g., 'push', 'release').</td>
                      </tr>
                      {/* Timestamp Proof */}
                      <tr>
                        <td className="py-3 pr-4 font-semibold text-white">Timestamp Proof</td>
                        <td className="py-3 pr-4 font-mono text-orange-400">signing_time</td>
                        <td className="py-3 text-zinc-400">The UNIX timestamp in seconds when the DSSE Payload is signed.</td>
                      </tr>
                      <tr>
                        <td className="py-3 pr-4"></td>
                        <td className="py-3 pr-4 font-mono text-orange-400">tsa_certs</td>
                        <td className="py-3 text-zinc-400">RFC 3161 only. Array of SHA256 certificate hashes, leaf first, root last.</td>
                      </tr>
                      <tr>
                        <td className="py-3 pr-4"></td>
                        <td className="py-3 pr-4 font-mono text-orange-400">message_imprint</td>
                        <td className="py-3 text-zinc-400">RFC 3161 only. Hash of DSSE Signature.</td>
                      </tr>
                      <tr>
                        <td className="py-3 pr-4"></td>
                        <td className="py-3 pr-4 font-mono text-orange-400">log_id</td>
                        <td className="py-3 text-zinc-400">Rekor only. The SHA256 hash of Rekor's public key (identifies the log instance).</td>
                      </tr>
                      <tr>
                        <td className="py-3 pr-4"></td>
                        <td className="py-3 pr-4 font-mono text-orange-400">log_index</td>
                        <td className="py-3 text-zinc-400">Rekor only. The tree leaf index (for Merkle proof verification).</td>
                      </tr>
                      <tr>
                        <td className="py-3 pr-4"></td>
                        <td className="py-3 pr-4 font-mono text-orange-400">entry_index</td>
                        <td className="py-3 text-zinc-400">Rekor only. The entry index (for API queries to fetch the full entry).</td>
                      </tr>
                    </tbody>
                  </table>
                </div>
              </div>

              {/* On-Chain Use Cases */}
              <div>
                <h4 className="text-base md:text-lg font-bold text-white mb-2">On-Chain Use Cases</h4>
                <p className="text-zinc-400 text-xs md:text-sm mb-4">
                  Smart contracts can leverage the verified data for various security and governance applications.
                </p>
                <div className="grid md:grid-cols-2 gap-3 md:gap-4">
                  <div className="p-3 md:p-4 border border-zinc-800 bg-zinc-900/20">
                    <h5 className="font-bold text-white text-sm md:text-base mb-1 md:mb-2">Supply Chain Security</h5>
                    <p className="text-xs md:text-sm text-zinc-400">Verify artifacts were built by authorized CI/CD pipelines before deployment.</p>
                  </div>
                  <div className="p-3 md:p-4 border border-zinc-800 bg-zinc-900/20">
                    <h5 className="font-bold text-white text-sm md:text-base mb-1 md:mb-2">Governance</h5>
                    <p className="text-xs md:text-sm text-zinc-400">Time-lock contract execution based on build timestamps.</p>
                  </div>
                  <div className="p-3 md:p-4 border border-zinc-800 bg-zinc-900/20">
                    <h5 className="font-bold text-white text-sm md:text-base mb-1 md:mb-2">Access Control</h5>
                    <p className="text-xs md:text-sm text-zinc-400">Grant permissions only to artifacts from specific GitHub repositories.</p>
                  </div>
                  <div className="p-3 md:p-4 border border-zinc-800 bg-zinc-900/20">
                    <h5 className="font-bold text-white text-sm md:text-base mb-1 md:mb-2">Artifact Registry</h5>
                    <p className="text-xs md:text-sm text-zinc-400">Maintain on-chain directory of verified builds for trustless discovery.</p>
                  </div>
                </div>
              </div>
            </div>
          )}

        </div>
      </div>
    </section>
  );
};

export default ZkVerification;
