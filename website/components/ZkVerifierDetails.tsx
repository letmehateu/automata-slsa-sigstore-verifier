import React, { useState } from 'react';
import { Cpu, Terminal, ChevronRight, FileJson, CheckCircle2, Box } from 'lucide-react';
import { VerificationOutputField } from '../types';

const OUTPUT_FIELDS: VerificationOutputField[] = [
    { id: 1, label: "Fulcio Certificates Hash Chain", description: "Array of hashes, leaf first, root last.", technicalKey: "fulcio_certs" },
    { id: 2, label: "Subject Digest", description: "The hash of the build artifact.", technicalKey: "subject_digest" },
    { id: 3, label: "OIDC Issuer", description: "The identity provider URL (e.g., https://token.actions.githubusercontent.com).", technicalKey: "oidc_issuer" },
    { id: 4, label: "OIDC Workflow Ref", description: "Reference to the workflow file.", technicalKey: "workflow_ref" },
    { id: 5, label: "OIDC Repository", description: "The repo where the action ran.", technicalKey: "repository" },
    { id: 6, label: "OIDC Event Name", description: "Trigger event (e.g., 'push', 'release').", technicalKey: "event_name" },
    { id: 7, label: "TSA Certificate Hash Chain", description: "RFC 3161 only. Sorted leaf first.", technicalKey: "tsa_certs" },
    { id: 8, label: "Message Imprint", description: "RFC 3161 only. Hash of DSSE Payload.", technicalKey: "message_imprint" },
    { id: 9, label: "Rekor Log ID", description: "Unique ID of the transparency log.", technicalKey: "log_id" },
    { id: 10, label: "Rekor Entry Index", description: "Position in the log.", technicalKey: "log_index" },
    { id: 11, label: "Rekor Integrated Time", description: "When the entry was persisted.", technicalKey: "integrated_time" },
];

const ZKVM_COMMANDS = {
  RISC0: {
    build: 'cargo build -p risc0-host --release',
    prove: 'cargo run -p risc0-host --release -- prove \\\n  --bundle ./attestation.json \\\n  --trust-roots ./trusted-root.json \\\n --output ./output/boundless-proof.json \\\n  boundless',
    imageId: 'cargo run -p risc0-host -- image-id',
  },
  SP1: {
    build: 'cargo build -p sp1-host --release',
    prove: 'cargo run -p sp1-host --release -- prove \\\n  --bundle ./attestation.json \\\n  --trust-roots ./trusted-root.json \\\n --output ./output/boundless-proof.json \\\n  network',
    imageId: 'cargo run -p sp1-host -- verifying-key',
  },
  Pico: {
    build: 'cargo build -p pico-host --release',
    prove: 'cargo run -p pico-host --release -- prove \\\n  --bundle ./attestation.json \\\n  --trust-roots ./trusted-root.json',
    imageId: 'cargo run -p pico-host -- program-id',
  },
};

const ZkVerifierDetails: React.FC = () => {
  const [activeTab, setActiveTab] = useState<'input' | 'process' | 'output'>('input');
  const [selectedVm, setSelectedVm] = useState<'RISC0' | 'SP1' | 'Pico'>('RISC0');

  return (
    <div className="bg-slate-900 border border-slate-800 rounded-xl overflow-hidden shadow-2xl">
      <div className="flex border-b border-slate-800 bg-slate-950">
        <button
          onClick={() => setActiveTab('input')}
          className={`px-6 py-4 flex items-center gap-2 font-medium transition-colors ${activeTab === 'input' ? 'text-blue-400 border-b-2 border-blue-400 bg-slate-900' : 'text-slate-400 hover:text-slate-200'}`}
        >
          <FileJson size={18} /> Inputs
        </button>
        <button
          onClick={() => setActiveTab('process')}
          className={`px-6 py-4 flex items-center gap-2 font-medium transition-colors ${activeTab === 'process' ? 'text-purple-400 border-b-2 border-purple-400 bg-slate-900' : 'text-slate-400 hover:text-slate-200'}`}
        >
          <Cpu size={18} /> zkVM Process
        </button>
        <button
          onClick={() => setActiveTab('output')}
          className={`px-6 py-4 flex items-center gap-2 font-medium transition-colors ${activeTab === 'output' ? 'text-emerald-400 border-b-2 border-emerald-400 bg-slate-900' : 'text-slate-400 hover:text-slate-200'}`}
        >
          <Terminal size={18} /> Outputs
        </button>
      </div>

      <div className="p-8 min-h-[500px]">
        {activeTab === 'input' && (
            <div className="animate-fade-in">
                <h3 className="text-xl font-bold text-white mb-4">Required Verification Artifacts</h3>
                <p className="text-slate-400 mb-6">
                    The verifier requires two inputs: a Sigstore bundle (v0.3+) and trust roots.
                    Trust roots should be refreshed every 60-90 days using <code className="bg-slate-800 px-1 py-0.5 rounded text-slate-200">gh attestation trusted-root</code> due to certificate rotation.
                </p>
                
                <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
                    <div className="bg-slate-950 p-5 rounded-lg border border-slate-800 font-mono text-sm overflow-hidden">
                        <div className="text-blue-400 font-bold mb-2">// 1. Attestation Bundle</div>
                        <pre className="text-slate-500 overflow-x-auto whitespace-pre">{`{
  "mediaType": "application/vnd.dev.sigstore.bundle+json;version=0.3",
  "verificationMaterial": { ... },
  "dsseEnvelope": {
    "payload": "...",
    "signatures": [ ... ]
  }
}`}</pre>
                        <p className="mt-4 text-xs text-slate-400 font-sans">
                            Generated directly by the <code className="bg-slate-800 px-1 py-0.5 rounded text-slate-200">attest-build-provenance</code> GitHub Action.
                        </p>
                    </div>

                    <div className="bg-slate-950 p-5 rounded-lg border border-slate-800 font-mono text-sm overflow-hidden">
                        <div className="text-purple-400 font-bold mb-2">// 2. Trust Roots</div>
                        <pre className="text-slate-500 overflow-x-auto whitespace-pre">{`{
  "mediaType": "application/vnd.dev.sigstore.trustedroot+json;version=0.1",
  "certificateAuthorities": [ ... ],
  "tlogs": [ ... ],
  "timestampAuthorities": [ ... ]
}`}</pre>
                        <p className="mt-4 text-xs text-slate-400 font-sans">
                            Obtained via CLI: <code className="bg-slate-800 px-1 py-0.5 rounded text-slate-200">gh attestation trusted-root</code>
                        </p>
                    </div>
                </div>
            </div>
        )}

        {activeTab === 'process' && (
            <div className="animate-fade-in space-y-8">
                <div>
                    <h3 className="text-xl font-bold text-white mb-2">Supported Environments</h3>
                    <p className="text-slate-400 mb-6">Our Rust library is optimized for compilation to RISC-V and other zkVM targets. All produce Groth16 SNARK proofs for on-chain verification.</p>
                    <div className="flex flex-wrap gap-3 mb-6">
                        {(['RISC0', 'SP1', 'Pico'] as const).map(vm => (
                            <button
                                key={vm}
                                onClick={() => setSelectedVm(vm)}
                                className={`flex items-center gap-2 px-4 py-2 rounded-full border font-semibold transition-all ${
                                    selectedVm === vm
                                        ? 'bg-indigo-600 border-indigo-500 text-white'
                                        : 'bg-slate-800 border-slate-700 text-slate-200 hover:border-slate-600'
                                }`}
                            >
                                <Box size={16} /> {vm}
                            </button>
                        ))}
                    </div>

                    <div className="bg-slate-950 rounded-lg border border-slate-800 overflow-hidden">
                        <div className="px-4 py-3 border-b border-slate-800 flex items-center gap-2">
                            <Terminal size={16} className="text-slate-400" />
                            <span className="text-sm font-medium text-slate-300">{selectedVm} Commands</span>
                        </div>
                        <div className="p-4 space-y-4">
                            <div>
                                <div className="text-xs text-slate-500 mb-1">Build</div>
                                <pre className="text-sm text-emerald-400 overflow-x-auto whitespace-pre">{ZKVM_COMMANDS[selectedVm].build}</pre>
                            </div>
                            <div>
                                <div className="text-xs text-slate-500 mb-1">Get Program ID</div>
                                <pre className="text-sm text-blue-400 overflow-x-auto whitespace-pre">{ZKVM_COMMANDS[selectedVm].imageId}</pre>
                            </div>
                            <div>
                                <div className="text-xs text-slate-500 mb-1">Generate Proof</div>
                                <pre className="text-sm text-purple-400 overflow-x-auto whitespace-pre">{ZKVM_COMMANDS[selectedVm].prove}</pre>
                            </div>
                        </div>
                    </div>
                </div>

                <div className="relative pl-8 border-l-2 border-slate-800 space-y-8">
                    <div className="relative">
                        <span className="absolute -left-[41px] bg-slate-900 border-2 border-slate-700 w-6 h-6 rounded-full flex items-center justify-center text-xs text-slate-400">1</span>
                        <h4 className="text-lg font-semibold text-white">Execution</h4>
                        <p className="text-slate-400 text-sm">
                            The Rust verifier runs inside the zkVM guest program. It parses the Sigstore bundle, verifies the DSSE signature,
                            validates the certificate chain (leaf → intermediates → root), verifies the timestamp proof (RFC3161 or Rekor),
                            and extracts OIDC identity from certificate extensions.
                        </p>
                    </div>
                    <div className="relative">
                        <span className="absolute -left-[41px] bg-slate-900 border-2 border-purple-500 w-6 h-6 rounded-full flex items-center justify-center text-xs text-purple-400">2</span>
                        <h4 className="text-lg font-semibold text-white">Proof Generation</h4>
                        <p className="text-slate-400 text-sm">
                            A Groth16 ZK-SNARK proof is generated, cryptographically proving the verification succeeded.
                            The journal (public output) contains the VerificationResult with certificate hashes, OIDC identity, and timestamp data.
                        </p>
                    </div>
                     <div className="relative">
                        <span className="absolute -left-[41px] bg-slate-900 border-2 border-emerald-500 w-6 h-6 rounded-full flex items-center justify-center text-xs text-emerald-400">3</span>
                        <h4 className="text-lg font-semibold text-white">On-Chain Verification</h4>
                        <p className="text-slate-400 text-sm">
                            The proof and journal are submitted to a smart contract. The contract verifies the ZK proof,
                            parses the VerificationResult, and emits an attestation event. Other contracts can then use the verified data for access control, governance, or compliance.
                        </p>
                    </div>
                </div>
            </div>
        )}

        {activeTab === 'output' && (
            <div className="animate-fade-in space-y-8">
                <div>
                    <h3 className="text-xl font-bold text-white mb-3">Verification Data Structure</h3>
                    <p className="text-slate-400 text-sm mb-6">
                        The VerificationResult contains all cryptographic evidence and identity information, organized into categories for easy access.
                    </p>

                    {/* Artifact Info */}
                    <div className="mb-6">
                        <h4 className="text-md font-semibold text-blue-300 mb-3 flex items-center gap-2">
                            <CheckCircle2 size={16} /> Artifact Information
                        </h4>
                        <div className="grid grid-cols-1 md:grid-cols-2 gap-3">
                            <div className="bg-slate-950/50 p-3 rounded-lg border border-slate-800 hover:border-slate-600 transition-colors group">
                                <div className="flex justify-between items-start mb-1">
                                    <h5 className="font-semibold text-slate-200 text-sm">Subject Digest</h5>
                                    <code className="text-[10px] text-slate-500 bg-slate-900 px-1 py-0.5 rounded opacity-0 group-hover:opacity-100 transition-opacity">subjectDigest</code>
                                </div>
                                <p className="text-xs text-slate-400">Hash of the build artifact (SHA-256 or SHA-384)</p>
                            </div>
                        </div>
                    </div>

                    {/* Certificate Chain */}
                    <div className="mb-6">
                        <h4 className="text-md font-semibold text-purple-300 mb-3 flex items-center gap-2">
                            <FileJson size={16} /> Certificate Chains
                        </h4>
                        <div className="grid grid-cols-1 md:grid-cols-2 gap-3">
                            <div className="bg-slate-950/50 p-3 rounded-lg border border-slate-800 hover:border-slate-600 transition-colors group">
                                <div className="flex justify-between items-start mb-1">
                                    <h5 className="font-semibold text-slate-200 text-sm">Fulcio Certificate Hashes</h5>
                                    <code className="text-[10px] text-slate-500 bg-slate-900 px-1 py-0.5 rounded opacity-0 group-hover:opacity-100 transition-opacity">certificateHashes</code>
                                </div>
                                <p className="text-xs text-slate-400">SHA-256 hashes of the signing certificate chain [leaf, ...intermediates, root]</p>
                            </div>
                            <div className="bg-slate-950/50 p-3 rounded-lg border border-slate-800 hover:border-slate-600 transition-colors group">
                                <div className="flex justify-between items-start mb-1">
                                    <h5 className="font-semibold text-slate-200 text-sm">TSA Certificate Hashes</h5>
                                    <code className="text-[10px] text-slate-500 bg-slate-900 px-1 py-0.5 rounded opacity-0 group-hover:opacity-100 transition-opacity">tsaChainHashes</code>
                                </div>
                                <p className="text-xs text-slate-400">RFC 3161 only. Timestamp Authority certificate chain hashes</p>
                            </div>
                        </div>
                    </div>

                    {/* OIDC Identity */}
                    <div className="mb-6">
                        <h4 className="text-md font-semibold text-emerald-300 mb-3 flex items-center gap-2">
                            <Terminal size={16} /> OIDC Identity
                        </h4>
                        <div className="grid grid-cols-1 md:grid-cols-2 gap-3">
                            {OUTPUT_FIELDS.filter(f => [3, 4, 5, 6].includes(f.id)).map((field) => (
                                <div key={field.id} className="bg-slate-950/50 p-3 rounded-lg border border-slate-800 hover:border-slate-600 transition-colors group">
                                    <div className="flex justify-between items-start mb-1">
                                        <h5 className="font-semibold text-slate-200 text-sm">{field.label}</h5>
                                        <code className="text-[10px] text-slate-500 bg-slate-900 px-1 py-0.5 rounded opacity-0 group-hover:opacity-100 transition-opacity">{field.technicalKey}</code>
                                    </div>
                                    <p className="text-xs text-slate-400">{field.description}</p>
                                </div>
                            ))}
                        </div>
                    </div>

                    {/* Timestamp Proof */}
                    <div className="mb-6">
                        <h4 className="text-md font-semibold text-orange-300 mb-3 flex items-center gap-2">
                            <Cpu size={16} /> Timestamp Proof
                        </h4>
                        <div className="grid grid-cols-1 md:grid-cols-2 gap-3">
                            {OUTPUT_FIELDS.filter(f => [8, 9, 10, 11].includes(f.id)).map((field) => (
                                <div key={field.id} className="bg-slate-950/50 p-3 rounded-lg border border-slate-800 hover:border-slate-600 transition-colors group">
                                    <div className="flex justify-between items-start mb-1">
                                        <h5 className="font-semibold text-slate-200 text-sm">{field.label}</h5>
                                        <code className="text-[10px] text-slate-500 bg-slate-900 px-1 py-0.5 rounded opacity-0 group-hover:opacity-100 transition-opacity">{field.technicalKey}</code>
                                    </div>
                                    <p className="text-xs text-slate-400">{field.description}</p>
                                </div>
                            ))}
                        </div>
                    </div>
                </div>

                {/* On-Chain Use Cases */}
                <div className="border-t border-slate-800 pt-8">
                    <h3 className="text-xl font-bold text-white mb-3">On-Chain Use Cases</h3>
                    <p className="text-slate-400 text-sm mb-6">
                        Smart contracts can leverage the verified data for various security and governance applications.
                    </p>

                    <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                        <div className="bg-gradient-to-br from-blue-900/20 to-blue-800/10 p-5 rounded-xl border border-blue-500/20">
                            <h4 className="text-lg font-semibold text-blue-200 mb-2">Supply Chain Security</h4>
                            <p className="text-sm text-slate-400">
                                Verify artifacts were built by authorized CI/CD pipelines before deployment.
                                Check <code className="bg-slate-900 px-1 rounded text-xs">oidcRepository</code> and{' '}
                                <code className="bg-slate-900 px-1 rounded text-xs">oidcWorkflowRef</code> match your security policy.
                            </p>
                        </div>

                        <div className="bg-gradient-to-br from-purple-900/20 to-purple-800/10 p-5 rounded-xl border border-purple-500/20">
                            <h4 className="text-lg font-semibold text-purple-200 mb-2">Governance</h4>
                            <p className="text-sm text-slate-400">
                                Time-lock contract execution based on build timestamps.
                                Require artifacts built after security audit completion using the{' '}
                                <code className="bg-slate-900 px-1 rounded text-xs">timestamp</code> field.
                            </p>
                        </div>

                        <div className="bg-gradient-to-br from-emerald-900/20 to-emerald-800/10 p-5 rounded-xl border border-emerald-500/20">
                            <h4 className="text-lg font-semibold text-emerald-200 mb-2">Access Control</h4>
                            <p className="text-sm text-slate-400">
                                Grant permissions only to artifacts from specific GitHub repositories.
                                Validate OIDC identity fields match organization requirements before executing privileged operations.
                            </p>
                        </div>

                        <div className="bg-gradient-to-br from-orange-900/20 to-orange-800/10 p-5 rounded-xl border border-orange-500/20">
                            <h4 className="text-lg font-semibold text-orange-200 mb-2">Artifact Registry</h4>
                            <p className="text-sm text-slate-400">
                                Maintain on-chain directory of verified builds.
                                Query by certificate hashes, repository, or timestamp for trustless artifact discovery and provenance tracking.
                            </p>
                        </div>
                    </div>
                </div>
            </div>
        )}
      </div>
    </div>
  );
};

export default ZkVerifierDetails;
