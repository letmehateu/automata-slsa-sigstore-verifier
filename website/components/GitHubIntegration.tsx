import React from 'react';
import { GitBranch, Lock, Globe, Clock, FileCheck, Download, Key, Shield } from 'lucide-react';
import CodeExample from './CodeExample';

const GitHubIntegration: React.FC = () => {
  return (
    <div className="py-10 space-y-12">

      {/* How to Generate Attestations */}
      <div className="bg-slate-950/30 rounded-xl p-6 border border-slate-800">
        <h3 className="text-2xl font-bold text-white mb-3 flex items-center gap-2">
          <Shield className="w-6 h-6 text-orange-400" />
          Step 1: Generate Attestations
        </h3>
        <p className="text-slate-400 mb-4">
          Use GitHub Actions to automatically generate Sigstore attestations during your build process.
        </p>
        <CodeExample
          language="yaml"
          title="GitHub Actions workflow example"
          code={`name: Build and Attest
on: [push]

jobs:
  build:
    runs-on: ubuntu-latest
    permissions:
      contents: read
      attestations: write
      id-token: write

    steps:
      - uses: actions/checkout@v5

      - name: Build artifact
        run: |
          # Your build commands
          echo "built" > artifact.txt

      - name: Attest build provenance
        uses: actions/attest-build-provenance@v3
        with:
          subject-path: artifact.txt`}
        />
      </div>

      {/* How to Download Bundles */}
      <div className="bg-slate-950/30 rounded-xl p-6 border border-slate-800">
        <h3 className="text-2xl font-bold text-white mb-3 flex items-center gap-2">
          <Download className="w-6 h-6 text-blue-400" />
          Step 2: Download Bundles
        </h3>
        <p className="text-slate-400 mb-4">
          After GitHub Actions generates the attestation, download the Sigstore bundle using the <a href="https://cli.github.com" target="_blank" rel="noopener noreferrer" className="text-orange-400 underline"> GitHub CLI </a>.
        </p>
        <div className="space-y-3">
          <CodeExample
            language="bash"
            title="Using local artifact path"
            code={`gh attestation download local/path/to/artifact -R OWNER/REPO`}
          />

          <p> OR </p>

          <CodeExample
            language="bash"
            title="Using container image URI"
            code={`gh attestation download oci://<image-uri> -R OWNER/REPO`}
          />

          <p> OR </p>

          <CodeExample
            language="bash"
            title="Using CURL directly from GitHub"
            code={`curl https://github.com/OWNER/REPO/attestations/<attestation-id>/download > bundle.json`}
          />
          
        </div>
      </div>

      {/* Bundle Types */}
      <div>
        <h3 className="text-2xl font-bold text-white mb-4">Bundle Types</h3>
        <p className="text-slate-400 mb-6">
          GitHub generates two different types of Sigstore bundles depending on repository visibility.
        </p>
        <div className="grid grid-cols-1 md:grid-cols-2 gap-8">
        
        {/* Public Repos */}
        <div className="bg-slate-900/50 border border-slate-700 rounded-2xl p-6 hover:border-emerald-500/50 transition-colors">
            <div className="flex items-center gap-3 mb-6">
                <div className="p-3 bg-emerald-500/10 rounded-lg text-emerald-400">
                    <Globe size={24} />
                </div>
                <div>
                    <h3 className="text-xl font-bold text-white">Public Repositories</h3>
                    <p className="text-sm text-emerald-400">Public Good Instance</p>
                </div>
            </div>
            
            <ul className="space-y-4">
                <li className="flex items-start gap-3">
                    <FileCheck className="w-5 h-5 text-slate-400 mt-1" />
                    <div>
                        <span className="text-slate-200 font-medium">Fulcio Issuer</span>
                        <p className="text-sm text-slate-400">Public Good Sigstore CA</p>
                    </div>
                </li>
                <li className="flex items-start gap-3">
                    <Clock className="w-5 h-5 text-slate-400 mt-1" />
                    <div>
                        <span className="text-slate-200 font-medium">Timestamping</span>
                        <p className="text-sm text-slate-400">Rekor Transparency Log with Merkle inclusion proof</p>
                    </div>
                </li>
                <li className="flex items-start gap-3">
                    <DatabaseIcon className="w-5 h-5 text-slate-400 mt-1" />
                    <div>
                        <span className="text-slate-200 font-medium">Inclusion Proof</span>
                        <p className="text-sm text-slate-400">Cryptographic proof that the cert exists in the public log.</p>
                    </div>
                </li>
            </ul>
        </div>

        {/* Private Repos */}
        <div className="bg-slate-900/50 border border-slate-700 rounded-2xl p-6 hover:border-orange-500/50 transition-colors">
            <div className="flex items-center gap-3 mb-6">
                <div className="p-3 bg-orange-500/10 rounded-lg text-orange-400">
                    <Lock size={24} />
                </div>
                <div>
                    <h3 className="text-xl font-bold text-white">Private Repositories</h3>
                    <p className="text-sm text-orange-400">GitHub Internal CA</p>
                </div>
            </div>
            
            <ul className="space-y-4">
                <li className="flex items-start gap-3">
                    <FileCheck className="w-5 h-5 text-slate-400 mt-1" />
                    <div>
                        <span className="text-slate-200 font-medium">Fulcio Issuer</span>
                        <p className="text-sm text-slate-400">GitHub's own CA</p>
                    </div>
                </li>
                <li className="flex items-start gap-3">
                    <Clock className="w-5 h-5 text-slate-400 mt-1" />
                    <div>
                        <span className="text-slate-200 font-medium">Timestamping</span>
                        <p className="text-sm text-slate-400">RFC 3161 Timestamp Authority (TSA) with certificate chain</p>
                    </div>
                </li>
                <li className="flex items-start gap-3">
                    <GitBranch className="w-5 h-5 text-slate-400 mt-1" />
                    <div>
                        <span className="text-slate-200 font-medium">Privacy</span>
                        <p className="text-sm text-slate-400">Metadata is kept private; not posted to public logs.</p>
                    </div>
                </li>
            </ul>
        </div>
      </div>
      </div>

      {/* How to Get Trust Roots */}
      <div className="bg-slate-950/30 rounded-xl p-6 border border-slate-800">
        <h3 className="text-2xl font-bold text-white mb-3 flex items-center gap-2">
          <Key className="w-6 h-6 text-orange-400" />
          Step 3: Get Trust Roots
        </h3>
        <p className="text-slate-400 mb-4">
          Trust roots contain the Fulcio CA certificates and Timestamp Authority certificates needed to verify the attestation.
          Recommended to refresh every 60-90 days due to certificate rotation.
        </p>
        <div className="space-y-3">
          <CodeExample
            language="bash"
            title="Fetch all trust roots using GitHub CLI (recommended)"
            code={`gh attestation trusted-root > trusted_root.jsonl`}
          />
          <CodeExample
            language="bash"
            title="Fetch GitHub Fulcio trust bundle directly"
            code={`curl https://fulcio.githubapp.com/api/v2/trustBundle > github_trust.json`}
          />
          <CodeExample
            language="bash"
            title="Fetch public Sigstore trust bundle"
            code={`curl https://fulcio.sigstore.dev/api/v2/trustBundle > sigstore_trust.json`}
          />
        </div>
      </div>
    </div>
  );
};

// Helper icon
const DatabaseIcon = ({ className }: { className?: string }) => (
    <svg 
      xmlns="http://www.w3.org/2000/svg" 
      width="24" height="24" viewBox="0 0 24 24" 
      fill="none" stroke="currentColor" strokeWidth="2" 
      strokeLinecap="round" strokeLinejoin="round" 
      className={className}
    >
      <ellipse cx="12" cy="5" rx="9" ry="3"></ellipse>
      <path d="M21 12c0 1.66-4 3-9 3s-9-1.34-9-3"></path>
      <path d="M3 5v14c0 1.66 4 3 9 3s9-1.34 9-3V5"></path>
    </svg>
);

export default GitHubIntegration;
