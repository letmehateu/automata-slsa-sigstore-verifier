import React, { useState, useEffect } from 'react';
import CodeBlock from './ui/CodeBlock';
import { FileJson, Key, Shield, Clock, Lock, Globe, FileBadge, ChevronLeft, ChevronRight, ChevronDown, GitBranch } from 'lucide-react';

type SubsectionId = 'slsa' | 'oidc-fulcio' | 'timestamping';

interface Subsection {
  id: SubsectionId;
  anchorId: string;
  icon: React.ReactNode;
  title: string;
  shortTitle: string;
}

const subsections: Subsection[] = [
  { id: 'slsa', anchorId: 'specification-slsa', icon: <FileJson className="w-5 h-5" />, title: 'SLSA Provenance v1 Bundle', shortTitle: 'SLSA Provenance' },
  { id: 'oidc-fulcio', anchorId: 'specification-oidc-fulcio', icon: <FileBadge className="w-5 h-5" />, title: 'OIDC Authentication & Fulcio CA', shortTitle: 'OIDC & Fulcio' },
  { id: 'timestamping', anchorId: 'specification-signature-timestamping', icon: <Clock className="w-5 h-5" />, title: 'Signature Timestamping', shortTitle: 'Timestamping' },
];

const Specification: React.FC = () => {
  const [activeSection, setActiveSection] = useState<SubsectionId | null>(null);

  // Listen for hash changes to auto-expand subsections from navigation
  useEffect(() => {
    const handleHashChange = () => {
      const hash = window.location.hash.slice(1); // Remove the '#'
      const matchedSection = subsections.find(s => s.anchorId === hash);
      if (matchedSection) {
        setActiveSection(matchedSection.id);
      }
    };

    // Check on mount
    handleHashChange();

    // Listen for hash changes
    window.addEventListener('hashchange', handleHashChange);
    return () => window.removeEventListener('hashchange', handleHashChange);
  }, []);

  const currentIndex = activeSection ? subsections.findIndex(s => s.id === activeSection) : -1;

  const navigatePrev = () => {
    if (currentIndex > 0) {
      setActiveSection(subsections[currentIndex - 1].id);
    } else if (currentIndex === -1) {
      setActiveSection(subsections[subsections.length - 1].id);
    }
  };

  const navigateNext = () => {
    if (currentIndex < subsections.length - 1 && currentIndex !== -1) {
      setActiveSection(subsections[currentIndex + 1].id);
    } else if (currentIndex === -1) {
      setActiveSection(subsections[0].id);
    }
  };

  const toggleSection = (id: SubsectionId) => {
    setActiveSection(activeSection === id ? null : id);
  };

  return (
    <section id="specification" className="py-16 md:py-24 border-b border-zinc-900 bg-black">
      <div className="max-w-7xl mx-auto px-4 md:px-6">

        {/* Section Header */}
        <div className="mb-12 md:mb-16">
          <div className="flex items-center gap-3 mb-4">
            <div className="p-2 bg-orange-500/10 border border-orange-500/20 rounded-lg">
              <FileJson className="w-5 h-5 text-orange-500" />
            </div>
            <span className="text-sm font-mono text-orange-500 uppercase tracking-wider">Specification</span>
          </div>
          <h2 className="text-2xl md:text-4xl font-bold text-white mb-4 md:mb-6">Protocol Deep Dive</h2>
          <p className="text-zinc-400 max-w-3xl leading-relaxed">
            In-depth technical details about the attestation format, identity verification, and timestamping mechanisms that power Sigstore.
          </p>
        </div>

        {/* Navigation Controls */}
        <div className="flex items-center justify-between mb-8 pb-4 border-b border-zinc-800">
          {/* Left/Right Navigation */}
          <div className="flex items-center gap-2">
            <button
              onClick={navigatePrev}
              disabled={currentIndex === 0}
              className={`p-2 border rounded-lg transition-all duration-200 ${
                currentIndex === 0
                  ? 'border-zinc-800 text-zinc-600 cursor-not-allowed'
                  : 'border-zinc-700 text-zinc-400 hover:text-white hover:border-zinc-500 hover:bg-zinc-900'
              }`}
              aria-label="Previous section"
            >
              <ChevronLeft size={20} />
            </button>
            <button
              onClick={navigateNext}
              disabled={currentIndex === subsections.length - 1}
              className={`p-2 border rounded-lg transition-all duration-200 ${
                currentIndex === subsections.length - 1
                  ? 'border-zinc-800 text-zinc-600 cursor-not-allowed'
                  : 'border-zinc-700 text-zinc-400 hover:text-white hover:border-zinc-500 hover:bg-zinc-900'
              }`}
              aria-label="Next section"
            >
              <ChevronRight size={20} />
            </button>

            {/* Current section indicator */}
            {activeSection && (
              <span className="ml-4 text-sm text-zinc-500 font-mono">
                {currentIndex + 1} / {subsections.length}
              </span>
            )}
          </div>

          {/* Section Pills */}
          <div className="hidden md:flex items-center gap-2">
            {subsections.map((section) => (
              <button
                key={section.id}
                onClick={() => toggleSection(section.id)}
                className={`px-3 py-1.5 text-xs font-mono rounded-full transition-all duration-200 ${
                  activeSection === section.id
                    ? 'bg-orange-500/20 text-orange-400 border border-orange-500/30'
                    : 'text-zinc-500 hover:text-zinc-300 border border-zinc-800 hover:border-zinc-600'
                }`}
              >
                {section.shortTitle}
              </button>
            ))}
          </div>
        </div>

        {/* Collapsible Subsections */}
        <div className="space-y-4">
          {subsections.map((section, index) => (
            <div
              key={section.id}
              id={section.anchorId}
              className="scroll-mt-24"
            >
              {/* Collapsible Header */}
              <button
                onClick={() => toggleSection(section.id)}
                className={`w-full flex items-center justify-between p-4 md:p-6 border rounded-lg transition-all duration-300 ${
                  activeSection === section.id
                    ? 'bg-zinc-900/50 border-zinc-700'
                    : 'bg-zinc-900/20 border-zinc-800 hover:border-zinc-700 hover:bg-zinc-900/30'
                }`}
              >
                <div className="flex items-center gap-3 md:gap-4">
                  <span className="text-orange-500">{section.icon}</span>
                  <div className="text-left">
                    <h3 className="text-lg md:text-xl font-bold text-white">{section.title}</h3>
                    <p className="text-xs text-zinc-500 mt-1 hidden md:block">
                      {section.id === 'slsa' && 'Bundle payload structure and field descriptions'}
                      {section.id === 'oidc-fulcio' && 'Identity verification and certificate issuance'}
                      {section.id === 'timestamping' && 'RFC 3161 TSA and Rekor transparency log'}
                    </p>
                  </div>
                </div>
                <ChevronDown
                  size={20}
                  className={`text-zinc-400 transition-transform duration-300 ${
                    activeSection === section.id ? 'rotate-180' : ''
                  }`}
                />
              </button>

              {/* Collapsible Content */}
              <div
                className={`overflow-hidden transition-all duration-500 ease-in-out ${
                  activeSection === section.id
                    ? 'max-h-[5000px] opacity-100'
                    : 'max-h-0 opacity-0'
                }`}
              >
                <div className="pt-6 pb-2 animate-in fade-in slide-in-from-top-2 duration-300">
                  {section.id === 'slsa' && <SLSAContent />}
                  {section.id === 'oidc-fulcio' && <OIDCFulcioContent />}
                  {section.id === 'timestamping' && <TimestampingContent />}
                </div>
              </div>
            </div>
          ))}
        </div>

      </div>
    </section>
  );
};

/* ==================== SLSA PROVENANCE CONTENT ==================== */
const SLSAContent: React.FC = () => (
  <div>
    <p className="text-zinc-400 mb-8 max-w-3xl">
      The attestation payload follows the SLSA Provenance v1 schema, an in-toto statement that describes what was built, how it was built, and by whom.
    </p>

    <div className="grid lg:grid-cols-2 gap-6 md:gap-8 mb-8">
      {/* JSON Structure */}
      <div>
        <h4 className="text-white font-bold mb-4">Bundle Payload Structure</h4>
        <CodeBlock
          language="json"
          code={`{
  "_type": "https://in-toto.io/Statement/v1",
  "subject": [{
    "name": "<artifact-name>",
    "digest": { "sha256": "<hex-digest>" }
  }],
  "predicateType": "https://slsa.dev/provenance/v1",
  "predicate": {
    "buildDefinition": {
      "buildType": "<uri>",
      "externalParameters": { ... },
      "internalParameters": { ... },
      "resolvedDependencies": [{ ... }]
    },
    "runDetails": {
      "builder": {
        "id": "<uri>",
        "version": { ... }
      },
      "metadata": {
        "invocationId": "<string>",
        "startedOn": "<timestamp>",
        "finishedOn": "<timestamp>"
      }
    }
  }
}`}
        />
      </div>

      {/* Example Card */}
      <div>
        <h4 className="text-white font-bold mb-4">External Parameters → Resolved Dependencies</h4>
        <p className="text-sm text-zinc-500 mb-4">
          The build process converts user-provided inputs into pinned, verified references:
        </p>
        <CodeBlock
          language="json"
          title="Transformation Example"
          code={`// External Parameters (user input)
{
  "repository": "https://github.com/octocat/hello-world",
  "ref": "refs/heads/main"
}

// Resolved Dependencies (verified)
[{
  "uri": "git+https://github.com/octocat/hello-world@refs/heads/main",
  "digest": {
    "gitCommit": "7fd1a60b01f91b314f59955a4e4d4e80d8edf11d"
  }
}]`}
        />
      </div>
    </div>

    {/* Field Descriptions Table */}
    <div>
      <h4 className="text-white font-bold mb-4">Field Reference</h4>
      <div className="overflow-x-auto -mx-4 md:mx-0 px-4 md:px-0">
        <table className="w-full text-left text-xs md:text-sm border-collapse min-w-[600px]">
          <thead>
            <tr className="border-b border-zinc-800 text-zinc-500 font-mono text-xs uppercase">
              <th className="py-3 pr-4">Field</th>
              <th className="py-3">Description</th>
            </tr>
          </thead>
          <tbody className="divide-y divide-zinc-800/50 text-zinc-300">
            <tr>
              <td className="py-3 pr-4 font-mono text-orange-400">subject</td>
              <td className="py-3 text-zinc-400">Array of artifacts being attested. Each contains <code className="text-orange-400 bg-zinc-900 px-1">name</code> and <code className="text-orange-400 bg-zinc-900 px-1">digest</code> (typically SHA256).</td>
            </tr>
            <tr>
              <td className="py-3 pr-4 font-mono text-orange-400">predicateType</td>
              <td className="py-3 text-zinc-400">URI identifying the attestation schema (<code className="text-amber-300 bg-zinc-900 px-1">https://slsa.dev/provenance/v1</code>).</td>
            </tr>
            <tr>
              <td className="py-3 pr-4 font-mono text-orange-400">buildDefinition.buildType</td>
              <td className="py-3 text-zinc-400">URI template defining how the build executes and how parameters are interpreted.</td>
            </tr>
            <tr>
              <td className="py-3 pr-4 font-mono text-orange-400">buildDefinition.externalParameters</td>
              <td className="py-3 text-zinc-400">User-controlled inputs (considered untrusted). E.g., repository URL, git ref, workflow inputs.</td>
            </tr>
            <tr>
              <td className="py-3 pr-4 font-mono text-orange-400">buildDefinition.resolvedDependencies</td>
              <td className="py-3 text-zinc-400">Artifacts fetched during the build with their verified digests. Converts external parameters to pinned references.</td>
            </tr>
            <tr>
              <td className="py-3 pr-4 font-mono text-orange-400">runDetails.builder.id</td>
              <td className="py-3 text-zinc-400">URI identifying the trusted build platform (e.g., GitHub Actions runner).</td>
            </tr>
            <tr>
              <td className="py-3 pr-4 font-mono text-orange-400">runDetails.metadata</td>
              <td className="py-3 text-zinc-400">Build execution metadata including <code className="text-orange-400 bg-zinc-900 px-1">invocationId</code>, <code className="text-orange-400 bg-zinc-900 px-1">startedOn</code>, and <code className="text-orange-400 bg-zinc-900 px-1">finishedOn</code> timestamps.</td>
            </tr>
          </tbody>
        </table>
      </div>
    </div>
  </div>
);

/* ==================== OIDC & FULCIO CONTENT ==================== */
const OIDCFulcioContent: React.FC = () => (
  <div>
    <p className="text-zinc-400 mb-8 max-w-3xl">
      Sigstore eliminates long-lived signing keys by using OpenID Connect (OIDC) for identity verification and Fulcio as a certificate authority that issues short-lived certificates tied to verified identities.
    </p>

    <div className="grid lg:grid-cols-2 gap-6 md:gap-8 mb-8">
      {/* What is OIDC Card */}
      <div className="p-6 border border-zinc-800 bg-zinc-900/20 rounded-lg">
        <div className="flex items-center gap-2 mb-4">
          <Key className="w-5 h-5 text-orange-500" />
          <h4 className="font-bold text-white">OpenID Connect (OIDC)</h4>
        </div>
        <p className="text-sm text-zinc-400 mb-6">
          OIDC is an identity layer built on OAuth 2.0 that provides verifiable identity tokens from trusted providers.
          When a GitHub Actions workflow runs, GitHub issues an OIDC token that cryptographically proves the workflow's identity.
        </p>

        <h5 className="text-xs font-mono text-zinc-500 uppercase tracking-wider mb-3">Token Claims</h5>
        <div className="space-y-2 text-sm">
          <div className="flex justify-between">
            <span className="font-mono text-orange-400">iss</span>
            <span className="text-zinc-400">Identity provider URL</span>
          </div>
          <div className="flex justify-between">
            <span className="font-mono text-orange-400">sub</span>
            <span className="text-zinc-400">Authenticated entity</span>
          </div>
          <div className="flex justify-between">
            <span className="font-mono text-orange-400">aud</span>
            <span className="text-zinc-400">Intended recipient</span>
          </div>
          <div className="flex justify-between">
            <span className="font-mono text-orange-400">exp</span>
            <span className="text-zinc-400">Expiration timestamp</span>
          </div>
        </div>
      </div>

      {/* Why Ephemeral Keys Card */}
      <div className="p-6 border border-zinc-800 bg-zinc-900/20 rounded-lg">
        <div className="flex items-center gap-2 mb-4">
          <Shield className="w-5 h-5 text-orange-500" />
          <h4 className="font-bold text-white">Why Ephemeral Keys Matter</h4>
        </div>
        <p className="text-sm text-zinc-400 mb-4">
          Traditional code signing requires managing long-lived private keys, which are frequently leaked or compromised.
          Ephemeral keys eliminate this risk entirely:
        </p>
        <ul className="space-y-2 text-sm text-zinc-400">
          <li className="flex items-start gap-2">
            <span className="text-orange-500 mt-1">•</span>
            <span>Private key exists only in memory during signing</span>
          </li>
          <li className="flex items-start gap-2">
            <span className="text-orange-500 mt-1">•</span>
            <span>Destroyed immediately after the signature is created</span>
          </li>
          <li className="flex items-start gap-2">
            <span className="text-orange-500 mt-1">•</span>
            <span>No key storage, no key rotation, no leaked secrets</span>
          </li>
          <li className="flex items-start gap-2">
            <span className="text-orange-500 mt-1">•</span>
            <span>Trust shifts from "who controls the key" to "who authenticated at signing time"</span>
          </li>
        </ul>
      </div>
    </div>

    {/* Fulcio Issuance Flow */}
    <div className="mb-8">
      <h4 className="text-white font-bold mb-4">Fulcio Certificate Issuance Flow</h4>
      <p className="text-sm text-zinc-500 mb-4">
        Fulcio is a free code-signing Certificate Authority that issues short-lived X.509 certificates (~10 minutes validity) to anyone with a verified OIDC identity.
      </p>
      <div className="overflow-x-auto -mx-4 md:mx-0 px-4 md:px-0">
        <table className="w-full text-left text-xs md:text-sm border-collapse min-w-[700px]">
          <thead>
            <tr className="border-b border-zinc-800 text-zinc-500 font-mono text-xs uppercase">
              <th className="py-3 pr-4 w-16">Step</th>
              <th className="py-3 pr-4 w-48">Action</th>
              <th className="py-3">Description</th>
            </tr>
          </thead>
          <tbody className="divide-y divide-zinc-800/50 text-zinc-300">
            <tr>
              <td className="py-3 pr-4 font-mono text-orange-500">1</td>
              <td className="py-3 pr-4 font-semibold text-white">OIDC Authentication</td>
              <td className="py-3 text-zinc-400">Client authenticates with identity provider (GitHub, Google, GitLab) and receives an OIDC token.</td>
            </tr>
            <tr>
              <td className="py-3 pr-4 font-mono text-orange-500">2</td>
              <td className="py-3 pr-4 font-semibold text-white">Ephemeral Key Generation</td>
              <td className="py-3 text-zinc-400">Client generates a one-time keypair in memory. The private key is never written to disk.</td>
            </tr>
            <tr>
              <td className="py-3 pr-4 font-mono text-orange-500">3</td>
              <td className="py-3 pr-4 font-semibold text-white">Certificate Request</td>
              <td className="py-3 text-zinc-400">Client sends the OIDC token, public key, and a signed challenge (proving private key possession) to Fulcio.</td>
            </tr>
            <tr>
              <td className="py-3 pr-4 font-mono text-orange-500">4</td>
              <td className="py-3 pr-4 font-semibold text-white">Token Verification</td>
              <td className="py-3 text-zinc-400">Fulcio fetches the issuer's public keys from the OIDC discovery endpoint and verifies the token signature.</td>
            </tr>
            <tr>
              <td className="py-3 pr-4 font-mono text-orange-500">5</td>
              <td className="py-3 pr-4 font-semibold text-white">Certificate Creation</td>
              <td className="py-3 text-zinc-400">Fulcio creates an X.509 certificate embedding the public key, identity (as SAN), and OIDC claims as extensions.</td>
            </tr>
            <tr>
              <td className="py-3 pr-4 font-mono text-orange-500">6</td>
              <td className="py-3 pr-4 font-semibold text-white">CT Log Submission</td>
              <td className="py-3 text-zinc-400">Certificate is submitted to a Certificate Transparency log; a Signed Certificate Timestamp (SCT) is embedded.</td>
            </tr>
            <tr>
              <td className="py-3 pr-4 font-mono text-orange-500">7</td>
              <td className="py-3 pr-4 font-semibold text-white">Certificate Returned</td>
              <td className="py-3 text-zinc-400">The signed certificate and SCT are returned to the client. The certificate is valid for ~10 minutes.</td>
            </tr>
          </tbody>
        </table>
      </div>
    </div>

    {/* Certificate Extensions */}
    <div>
      <h4 className="text-white font-bold mb-4">Certificate Extensions</h4>
      <p className="text-sm text-zinc-500 mb-4">
        Fulcio embeds OIDC claims as X.509 certificate extensions, enabling verifiers to enforce policies based on the signer's identity.
      </p>
      <div className="overflow-x-auto -mx-4 md:mx-0 px-4 md:px-0">
        <table className="w-full text-left text-xs md:text-sm border-collapse min-w-[700px]">
          <thead>
            <tr className="border-b border-zinc-800 text-zinc-500 font-mono text-xs uppercase">
              <th className="py-3 pr-4">Extension</th>
              <th className="py-3 pr-4">OID</th>
              <th className="py-3">Description</th>
            </tr>
          </thead>
          <tbody className="divide-y divide-zinc-800/50 text-zinc-300">
            <tr>
              <td className="py-3 pr-4 font-semibold text-white">Issuer</td>
              <td className="py-3 pr-4 font-mono text-zinc-500 text-xs">1.3.6.1.4.1.57264.1.1</td>
              <td className="py-3 text-zinc-400">OIDC provider URL</td>
            </tr>
            <tr>
              <td className="py-3 pr-4 font-semibold text-white">Workflow Trigger</td>
              <td className="py-3 pr-4 font-mono text-zinc-500 text-xs">1.3.6.1.4.1.57264.1.2</td>
              <td className="py-3 text-zinc-400">Event that triggered the workflow (e.g., <code className="text-amber-300 bg-zinc-900 px-1">push</code>, <code className="text-amber-300 bg-zinc-900 px-1">release</code>)</td>
            </tr>
            <tr>
              <td className="py-3 pr-4 font-semibold text-white">Workflow SHA</td>
              <td className="py-3 pr-4 font-mono text-zinc-500 text-xs">1.3.6.1.4.1.57264.1.3</td>
              <td className="py-3 text-zinc-400">Commit SHA of the workflow file</td>
            </tr>
            <tr>
              <td className="py-3 pr-4 font-semibold text-white">Workflow Name</td>
              <td className="py-3 pr-4 font-mono text-zinc-500 text-xs">1.3.6.1.4.1.57264.1.4</td>
              <td className="py-3 text-zinc-400">Name of the workflow</td>
            </tr>
            <tr>
              <td className="py-3 pr-4 font-semibold text-white">Repository</td>
              <td className="py-3 pr-4 font-mono text-zinc-500 text-xs">1.3.6.1.4.1.57264.1.5</td>
              <td className="py-3 text-zinc-400">Repository where the workflow ran</td>
            </tr>
            <tr>
              <td className="py-3 pr-4 font-semibold text-white">Workflow Ref</td>
              <td className="py-3 pr-4 font-mono text-zinc-500 text-xs">1.3.6.1.4.1.57264.1.6</td>
              <td className="py-3 text-zinc-400">Git ref (branch/tag) of the workflow</td>
            </tr>
            <tr>
              <td className="py-3 pr-4 font-semibold text-white">SAN</td>
              <td className="py-3 pr-4 font-mono text-zinc-500 text-xs">(Standard)</td>
              <td className="py-3 text-zinc-400">Full identity URI (e.g., <code className="text-amber-300 bg-zinc-900 px-1 text-xs">https://github.com/owner/repo/.github/workflows/build.yml@refs/heads/main</code>)</td>
            </tr>
          </tbody>
        </table>
      </div>
    </div>
  </div>
);

/* ==================== TIMESTAMPING CONTENT ==================== */
const TimestampingContent: React.FC = () => (
  <div>
    <p className="text-zinc-400 mb-8 max-w-3xl">
      Since Fulcio certificates are short-lived (~10 minutes), we need cryptographic proof that a signature was created while the certificate was still valid. Sigstore supports two timestamping mechanisms.
    </p>

    {/* Why Timestamping Card */}
    <div className="p-6 border border-zinc-800 bg-zinc-900/20 rounded-lg mb-8 max-w-2xl">
      <div className="flex items-center gap-2 mb-4">
        <Clock className="w-5 h-5 text-orange-500" />
        <h4 className="font-bold text-white">Why Timestamping Matters</h4>
      </div>
      <p className="text-sm text-zinc-400 mb-4">Without a trusted timestamp, an attacker could:</p>
      <ol className="space-y-2 text-sm text-zinc-400 list-decimal list-inside mb-4">
        <li>Steal a signature after the certificate expires</li>
        <li>Backdate a malicious artifact to appear legitimately signed</li>
      </ol>
      <p className="text-sm text-zinc-300">
        Timestamping proves the signature existed at a specific moment, enabling verification long after the certificate expires.
      </p>
    </div>

    {/* RFC 3161 Section */}
    <div className="mb-12">
      <div className="flex items-center gap-3 mb-4">
        <Lock className="w-5 h-5 text-orange-500" />
        <h4 className="text-lg md:text-xl font-bold text-white">RFC 3161 Timestamp Authority</h4>
        <span className="text-xs font-mono text-zinc-500 uppercase">Private Timestamping</span>
      </div>
      <p className="text-sm text-zinc-400 mb-6 max-w-3xl">
        RFC 3161 defines a protocol where a trusted Timestamp Authority (TSA) cryptographically binds a hash to a specific time. GitHub uses this for private repositories to avoid exposing signing events to public logs.
      </p>

      <div className="grid lg:grid-cols-2 gap-6 md:gap-8 mb-6">
        {/* How It Works */}
        <div>
          <h5 className="text-white font-bold mb-4">How It Works</h5>
          <div className="space-y-3">
            {[
              { step: '1', label: 'Hash Creation', desc: 'Client computes hash of the DSSE signature → "Message Imprint"' },
              { step: '2', label: 'Request', desc: 'Message Imprint sent to TSA (original data never leaves client)' },
              { step: '3', label: 'Timestamping', desc: 'TSA binds the hash + current time + serial number' },
              { step: '4', label: 'Signing', desc: 'TSA signs the bundle with its private key' },
              { step: '5', label: 'Response', desc: 'Timestamp Token returned and attached to the Sigstore bundle' },
            ].map((item) => (
              <div key={item.step} className="flex gap-3">
                <span className="font-mono text-orange-500 text-sm w-6 flex-shrink-0">{item.step}.</span>
                <div>
                  <span className="text-white text-sm font-semibold">{item.label}</span>
                  <p className="text-xs text-zinc-500">{item.desc}</p>
                </div>
              </div>
            ))}
          </div>
        </div>

        {/* TSTInfo Structure */}
        <div>
          <h5 className="text-white font-bold mb-4">Timestamp Token Structure</h5>
          <CodeBlock
            language="json"
            title="TSTInfo"
            code={`{
  "version": 1,
  "policy": "<TSA Policy OID>",
  "messageImprint": {
    "algorithm": "sha256",
    "hash": "<hash-of-signature>"
  },
  "serialNumber": "<unique-identifier>",
  "genTime": "<trusted-timestamp>",
  "accuracy": "<optional-precision>",
  "nonce": "<client-provided-nonce>"
}`}
          />
        </div>
      </div>

      {/* Key Components */}
      <div className="overflow-x-auto -mx-4 md:mx-0 px-4 md:px-0 mb-6">
        <table className="w-full text-left text-xs md:text-sm border-collapse min-w-[500px]">
          <thead>
            <tr className="border-b border-zinc-800 text-zinc-500 font-mono text-xs uppercase">
              <th className="py-3 pr-4">Component</th>
              <th className="py-3">Description</th>
            </tr>
          </thead>
          <tbody className="divide-y divide-zinc-800/50 text-zinc-300">
            <tr>
              <td className="py-3 pr-4 font-mono text-orange-400">Policy OID</td>
              <td className="py-3 text-zinc-400">Identifies the TSA's timestamping policy and legal obligations</td>
            </tr>
            <tr>
              <td className="py-3 pr-4 font-mono text-orange-400">messageImprint</td>
              <td className="py-3 text-zinc-400">Hash of the DSSE signature being timestamped (SHA256)</td>
            </tr>
            <tr>
              <td className="py-3 pr-4 font-mono text-orange-400">genTime</td>
              <td className="py-3 text-zinc-400">The trusted timestamp issued by the TSA</td>
            </tr>
            <tr>
              <td className="py-3 pr-4 font-mono text-orange-400">serialNumber</td>
              <td className="py-3 text-zinc-400">Unique identifier for this timestamp token</td>
            </tr>
            <tr>
              <td className="py-3 pr-4 font-mono text-orange-400">TSA Cert Chain</td>
              <td className="py-3 text-zinc-400">Chain of certificates to verify TSA signature (leaf → intermediates → root)</td>
            </tr>
          </tbody>
        </table>
      </div>

      {/* Privacy Note */}
      <div className="p-4 border border-zinc-800 bg-zinc-900/30 rounded-lg max-w-2xl">
        <p className="text-sm text-zinc-400">
          <span className="text-orange-500 font-semibold">Privacy:</span> Only the hash is sent to the TSA—the original signature and artifact data remain private.
          This makes RFC 3161 ideal for private repositories where signing events should not be publicly disclosed.
        </p>
      </div>
    </div>

    {/* Rekor Section */}
    <RekorSection />

    {/* Comparison Table */}
    <div>
      <h4 className="text-white font-bold mb-4">TSA vs Rekor: When to Use Each</h4>
      <div className="overflow-x-auto -mx-4 md:mx-0 px-4 md:px-0">
        <table className="w-full text-left text-xs md:text-sm border-collapse min-w-[600px]">
          <thead>
            <tr className="border-b border-zinc-800 text-zinc-500 font-mono text-xs uppercase">
              <th className="py-3 pr-4">Aspect</th>
              <th className="py-3 pr-4">
                <span className="flex items-center gap-2">
                  <Lock size={12} className="text-orange-500" />
                  RFC 3161 TSA
                </span>
              </th>
              <th className="py-3">
                <span className="flex items-center gap-2">
                  <Globe size={12} className="text-emerald-500" />
                  Rekor Transparency Log
                </span>
              </th>
            </tr>
          </thead>
          <tbody className="divide-y divide-zinc-800/50 text-zinc-300">
            <tr>
              <td className="py-3 pr-4 font-semibold text-white">Privacy</td>
              <td className="py-3 pr-4 text-zinc-400">Private—only hash sent to TSA</td>
              <td className="py-3 text-zinc-400">Public—entry visible to anyone</td>
            </tr>
            <tr>
              <td className="py-3 pr-4 font-semibold text-white">Verification</td>
              <td className="py-3 pr-4 text-zinc-400">Verify TSA certificate chain</td>
              <td className="py-3 text-zinc-400">Verify inclusion proof + log consistency</td>
            </tr>
            <tr>
              <td className="py-3 pr-4 font-semibold text-white">Auditability</td>
              <td className="py-3 pr-4 text-zinc-400">Limited to TSA operator</td>
              <td className="py-3 text-zinc-400">Anyone can monitor the log</td>
            </tr>
            <tr>
              <td className="py-3 pr-4 font-semibold text-white">Trust Model</td>
              <td className="py-3 pr-4 text-zinc-400">Trust the TSA operator</td>
              <td className="py-3 text-zinc-400">Trust through transparency</td>
            </tr>
            <tr>
              <td className="py-3 pr-4 font-semibold text-white">Use Case</td>
              <td className="py-3 pr-4 text-zinc-400">Private repositories</td>
              <td className="py-3 text-zinc-400">Public repositories</td>
            </tr>
            <tr>
              <td className="py-3 pr-4 font-semibold text-white">GitHub Usage</td>
              <td className="py-3 pr-4 text-zinc-400">GitHub Internal Services</td>
              <td className="py-3 text-zinc-400">Public Good Sigstore</td>
            </tr>
          </tbody>
        </table>
      </div>
    </div>
  </div>
);

/* ==================== REKOR SECTION COMPONENT ==================== */
const RekorSection: React.FC = () => {
  const [isMerkleTreeExpanded, setIsMerkleTreeExpanded] = useState(false);

  return (
    <div className="mb-12">
      <div className="flex items-center gap-3 mb-4">
        <Globe className="w-5 h-5 text-emerald-500" />
        <h4 className="text-lg md:text-xl font-bold text-white">Rekor Transparency Log</h4>
        <span className="text-xs font-mono text-zinc-500 uppercase">Public Timestamping</span>
      </div>
      <p className="text-sm text-zinc-400 mb-6 max-w-3xl">
        Rekor is an immutable, append-only ledger that records signing events in a publicly auditable log. Built on a Merkle tree data structure, it provides cryptographic guarantees that entries cannot be modified or deleted.
      </p>

      {/* Entry Body Structure Card */}
      <div className="grid lg:grid-cols-2 gap-6 md:gap-8 mb-8">
        <div>
          <h5 className="text-white font-bold mb-4">Entry Body Structure (DSSE Type)</h5>
          <p className="text-sm text-zinc-500 mb-4">
            When a DSSE signature is logged, Rekor stores it in a canonical format containing hashes of the envelope rather than the full data. This structure is what gets included in the Merkle tree.
          </p>
          <div className="overflow-x-auto -mx-4 md:mx-0 px-4 md:px-0">
            <div className="min-w-[320px]">
              <CodeBlock
                language="json"
                title="RekorDSSEEntry"
                code={`{
  "apiVersion": "0.0.1",
  "kind": "dsse",
  "spec": {
    "envelopeHash": {
      "algorithm": "sha256",
      "value": "<SHA256(DSSE-envelope-JSON)>"
    },
    "payloadHash": {
      "algorithm": "sha256",
      "value": "<SHA256(in-toto-payload)>"
    },
    "signatures": [{
      "signature": "<base64-encoded-DSSE-signature>",
      "verifier": "<base64-encoded-signing-certificate>"
    }]
  }
}`}
              />
            </div>
          </div>
        </div>

        {/* Key Components Table */}
        <div>
          <h5 className="text-white font-bold mb-4">Key Components</h5>
          <div className="overflow-x-auto">
            <table className="w-full text-left text-xs md:text-sm border-collapse min-w-[400px]">
              <thead>
                <tr className="border-b border-zinc-800 text-zinc-500 font-mono text-xs uppercase">
                  <th className="py-3 pr-4">Component</th>
                  <th className="py-3">Description</th>
                </tr>
              </thead>
              <tbody className="divide-y divide-zinc-800/50 text-zinc-300">
                <tr>
                  <td className="py-3 pr-4 font-mono text-orange-400">envelopeHash</td>
                  <td className="py-3 text-zinc-400">SHA256 hash of the entire DSSE envelope JSON (binds the signature to the log)</td>
                </tr>
                <tr>
                  <td className="py-3 pr-4 font-mono text-orange-400">payloadHash</td>
                  <td className="py-3 text-zinc-400">SHA256 hash of the in-toto statement payload</td>
                </tr>
                <tr>
                  <td className="py-3 pr-4 font-mono text-orange-400">signature</td>
                  <td className="py-3 text-zinc-400">The DSSE signature bytes (same as in <code className="text-amber-300 bg-zinc-900 px-1">dsseEnvelope.signatures</code>)</td>
                </tr>
                <tr>
                  <td className="py-3 pr-4 font-mono text-orange-400">verifier</td>
                  <td className="py-3 text-zinc-400">The signing certificate (same as <code className="text-amber-300 bg-zinc-900 px-1">verificationMaterial.certificate</code>)</td>
                </tr>
              </tbody>
            </table>
          </div>
        </div>
      </div>

      {/* Signed Entry Timestamp Card */}
      <div className="grid lg:grid-cols-2 gap-6 md:gap-8 mb-8">
        <div className="p-6 border border-zinc-800 bg-zinc-900/20 rounded-lg">
          <h5 className="text-white font-bold mb-4">Signed Entry Timestamp (SET)</h5>
          <p className="text-sm text-zinc-400 mb-4">
            The Signed Entry Timestamp cryptographically binds the integrated time to the entry. It is a signature from Rekor over the concatenation of log metadata and the entry body.
          </p>
          <div className="bg-zinc-950 p-4 rounded font-mono text-sm mb-4">
            <span className="text-zinc-500">SET = </span>
            <span className="text-emerald-400">Sign</span>
            <span className="text-zinc-500">_rekor(</span>
            <span className="text-orange-400">logID</span>
            <span className="text-zinc-500"> || </span>
            <span className="text-orange-400">logIndex</span>
            <span className="text-zinc-500"> || </span>
            <span className="text-orange-400">body</span>
            <span className="text-zinc-500"> || </span>
            <span className="text-orange-400">integratedTime</span>
            <span className="text-zinc-500">)</span>
          </div>
          <p className="text-sm text-zinc-500">
            Verifying the SET proves: <span className="text-zinc-300">"Rekor attests that this entry was added at this position and time."</span>
          </p>
        </div>

        {/* SET Components Table */}
        <div>
          <h5 className="text-white font-bold mb-4">SET Components</h5>
          <div className="overflow-x-auto">
            <table className="w-full text-left text-xs md:text-sm border-collapse min-w-[400px]">
              <thead>
                <tr className="border-b border-zinc-800 text-zinc-500 font-mono text-xs uppercase">
                  <th className="py-3 pr-4">Field</th>
                  <th className="py-3">Description</th>
                </tr>
              </thead>
              <tbody className="divide-y divide-zinc-800/50 text-zinc-300">
                <tr>
                  <td className="py-3 pr-4 font-mono text-orange-400">logID</td>
                  <td className="py-3 text-zinc-400">SHA256 hash of Rekor's public key (identifies the log instance)</td>
                </tr>
                <tr>
                  <td className="py-3 pr-4 font-mono text-orange-400">logIndex</td>
                  <td className="py-3 text-zinc-400">The tree leaf position of this entry</td>
                </tr>
                <tr>
                  <td className="py-3 pr-4 font-mono text-orange-400">body</td>
                  <td className="py-3 text-zinc-400">The canonicalized entry body (RekorDSSEEntry above)</td>
                </tr>
                <tr>
                  <td className="py-3 pr-4 font-mono text-orange-400">integratedTime</td>
                  <td className="py-3 text-zinc-400">Unix timestamp when Rekor accepted the entry</td>
                </tr>
              </tbody>
            </table>
          </div>
        </div>
      </div>

      {/* Collapsible Merkle Tree Section */}
      <div className="border border-zinc-800 rounded-lg overflow-hidden">
        <button
          onClick={() => setIsMerkleTreeExpanded(!isMerkleTreeExpanded)}
          className="w-full flex items-center justify-between p-4 md:p-6 bg-zinc-900/30 hover:bg-zinc-900/50 transition-colors"
        >
          <div className="flex items-center gap-3">
            <GitBranch className="w-5 h-5 text-zinc-500" />
            <div className="text-left">
              <h5 className="text-white font-bold">Merkle Tree Structure</h5>
              <p className="text-xs text-zinc-500 mt-1">Learn how entries are cryptographically verified</p>
            </div>
          </div>
          <ChevronDown
            size={20}
            className={`text-zinc-400 transition-transform duration-300 ${
              isMerkleTreeExpanded ? 'rotate-180' : ''
            }`}
          />
        </button>

        <div
          className={`overflow-hidden transition-all duration-500 ease-in-out ${
            isMerkleTreeExpanded ? 'max-h-[2000px] opacity-100' : 'max-h-0 opacity-0'
          }`}
        >
          <div className="p-4 md:p-6 border-t border-zinc-800 space-y-6">
            {/* Overview Sub-card */}
            <div className="p-4 border border-zinc-800/50 bg-zinc-900/20 rounded-lg">
              <h6 className="text-white font-semibold mb-3">What is a Merkle Tree?</h6>
              <p className="text-sm text-zinc-400 mb-4">
                A Merkle tree is a binary tree where each leaf node contains a hash of data, and each internal node contains a hash of its children. The root hash cryptographically commits to the entire tree contents.
              </p>
              <pre className="font-mono text-xs text-zinc-400 bg-zinc-950 p-4 rounded overflow-x-auto mb-4">
{`                [Root Hash]
                /          \\
         [H(AB)]            [H(CD)]
         /    \\              /    \\
      [H(A)] [H(B)]      [H(C)] [H(D)]
        |      |           |      |
     Entry   Entry      Entry   Entry
       1       2          3       4`}
              </pre>
              <div className="bg-zinc-950 p-3 rounded">
                <p className="text-xs font-mono text-zinc-500 mb-1">Leaf Hash Formula (RFC 6962):</p>
                <p className="text-sm font-mono">
                  <span className="text-orange-400">leaf_hash</span>
                  <span className="text-zinc-500"> = SHA256(</span>
                  <span className="text-emerald-400">0x00</span>
                  <span className="text-zinc-500"> || </span>
                  <span className="text-orange-400">RekorDSSEEntry_bytes</span>
                  <span className="text-zinc-500">)</span>
                </p>
                <p className="text-xs text-zinc-600 mt-2">The 0x00 prefix distinguishes leaf nodes from internal nodes (which use 0x01).</p>
              </div>
            </div>

            {/* Properties Sub-card */}
            <div className="p-4 border border-zinc-800/50 bg-zinc-900/20 rounded-lg">
              <h6 className="text-white font-semibold mb-3">Properties</h6>
              <div className="grid md:grid-cols-3 gap-4">
                <div>
                  <span className="text-orange-500 font-semibold text-sm">Tamper-Evident</span>
                  <p className="text-xs text-zinc-500">Any change to any entry changes the root hash</p>
                </div>
                <div>
                  <span className="text-orange-500 font-semibold text-sm">Append-Only</span>
                  <p className="text-xs text-zinc-500">New entries are added as leaves; existing entries cannot be modified</p>
                </div>
                <div>
                  <span className="text-orange-500 font-semibold text-sm">Efficient Verification</span>
                  <p className="text-xs text-zinc-500">Verify entry membership with O(log n) hashes</p>
                </div>
              </div>
            </div>

            {/* Inclusion Proofs Sub-card */}
            <div className="p-4 border border-zinc-800/50 bg-zinc-900/20 rounded-lg">
              <h6 className="text-white font-semibold mb-3">Inclusion Proofs</h6>
              <p className="text-sm text-zinc-400 mb-4">
                An inclusion proof demonstrates that a specific entry exists in the log without downloading the entire tree. The verifier receives the entry plus sibling hashes along the path to the root.
              </p>
              <div className="grid md:grid-cols-2 gap-4">
                <div className="text-xs text-zinc-500 space-y-1 font-mono bg-zinc-950 p-3 rounded">
                  <p className="text-zinc-400 mb-2">To prove Entry 2 is in the tree:</p>
                  <p>1. Provide: H(A), H(CD), and Entry 2</p>
                  <p>2. Compute: H(B) from Entry 2</p>
                  <p>3. Compute: H(AB) = H(H(A) || H(B))</p>
                  <p>4. Compute: Root = H(H(AB) || H(CD))</p>
                  <p className="text-emerald-400">5. Match root → Entry is included</p>
                </div>
                <div className="flex items-center">
                  <p className="text-sm text-zinc-400">
                    <span className="text-orange-500 font-semibold">Benefit:</span> Inclusion proofs can be "stapled" to artifacts, enabling offline verification without querying the log.
                  </p>
                </div>
              </div>
            </div>

            {/* Consistency Proofs Sub-card */}
            <div className="p-4 border border-zinc-800/50 bg-zinc-900/20 rounded-lg">
              <h6 className="text-white font-semibold mb-3">Consistency Proofs</h6>
              <p className="text-sm text-zinc-400 mb-4">
                Consistency proofs verify that the log is truly append-only—that no entries have been modified, deleted, or reordered between two points in time.
              </p>
              <div className="text-xs text-zinc-500 space-y-1">
                <p className="text-zinc-300 mb-2">Auditors continuously monitor the log by:</p>
                <p>1. Fetching the current root hash (signed by Rekor)</p>
                <p>2. Requesting a consistency proof from the previous root</p>
                <p>3. Verifying the old tree is a prefix of the new tree</p>
                <p>4. Alerting if any inconsistency is detected</p>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
};

export default Specification;
