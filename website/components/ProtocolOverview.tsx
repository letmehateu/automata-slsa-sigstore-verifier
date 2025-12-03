import React, { useState } from 'react';
import { ShieldCheck, Lock, Eye, User, FileBadge, Clock, SearchCheck, ChevronDown } from 'lucide-react';

interface FlowStep {
  id: number;
  icon: React.ReactNode;
  title: string;
  desc: string;
  details?: { title: string; text: string }[];
  hasBranches?: boolean;
  branches?: {
    label: string;
    details: { title: string; text: string }[];
  }[];
}

const ProtocolOverview: React.FC = () => {
  const [activeStep, setActiveStep] = useState<number | null>(null);

  const flowSteps: FlowStep[] = [
    {
      id: 1,
      icon: <User className="w-5 h-5 md:w-6 md:h-6" />,
      title: "Developer / CI",
      desc: "The Initiator",
      details: [
        { title: "OIDC Authentication", text: "The developer or CI system (e.g., GitHub Actions) authenticates via an OpenID Connect provider (Google, GitHub)." },
        { title: "Ephemeral Keys", text: "A one-time key pair is generated. The private key is held only in memory and discarded immediately after signing." },
        { title: "Sign Artifact", text: "The software artifact (binary, container) is signed using this ephemeral private key." }
      ]
    },
    {
      id: 2,
      icon: <FileBadge className="w-5 h-5 md:w-6 md:h-6" />,
      title: "Fulcio",
      desc: "Certificate Authority",
      details: [
        { title: "Identity Verification", text: "Fulcio verifies the OIDC token to confirm the user owns the email or identity claimed." },
        { title: "Certificate Issuance", text: "Issues a short-lived X.509 certificate (valid for ~10 minutes) binding the public key to the identity." },
        { title: "No Key Management", text: "Because keys are ephemeral, there are no long-lived secrets to store or leak." }
      ]
    },
    {
      id: 3,
      icon: <Clock className="w-5 h-5 md:w-6 md:h-6" />,
      title: "Rekor or TSA",
      desc: "Transparency & Time",
      hasBranches: true,
      branches: [
        {
          label: "Public: Rekor Log",
          details: [
            { title: "Immutable Record", text: "Signature and certificate are stored in a public, append-only transparency log." },
            { title: "Inclusion Proof", text: "Returns a signed entry proving the event is public and cannot be altered." }
          ]
        },
        {
          label: "Private: TSA (RFC 3161)",
          details: [
            { title: "Timestamp Authority", text: "Provides an RFC 3161 compliant timestamp proving the artifact existed at a specific time." },
            { title: "Certificate Chain", text: "Verifies the timestamp signature against a trusted root, independent of a public log." }
          ]
        }
      ]
    },
    {
      id: 4,
      icon: <SearchCheck className="w-5 h-5 md:w-6 md:h-6" />,
      title: "Verifier",
      desc: "Policy Check",
      details: [
        { title: "Signature Check", text: "Verifies the digital signature using the public key embedded in the certificate." },
        { title: "Log/TSA Verification", text: "Checks either the Rekor entry inclusion proof OR the TSA timestamp signature." },
        { title: "Policy Enforcement", text: "Confirms the identity matches the policy (e.g., \"Must be built by GitHub Actions on repo X\")." }
      ]
    }
  ];

  const renderStepDetails = (step: FlowStep) => {
    if (step.hasBranches && step.branches) {
      return (
        <div className="space-y-3 md:grid md:grid-cols-2 md:gap-4 md:space-y-0">
          {step.branches.map((branch, branchIdx) => (
            <div key={branchIdx} className="border border-zinc-800 p-4 bg-zinc-900/30">
              <h5 className="text-orange-500 font-mono text-xs uppercase mb-3 border-b border-zinc-800 pb-2">{branch.label}</h5>
              <div className="space-y-3">
                {branch.details.map((detail, idx) => (
                  <div key={idx}>
                    <h6 className="text-zinc-200 font-medium text-sm mb-1">{detail.title}</h6>
                    <p className="text-xs text-zinc-400">{detail.text}</p>
                  </div>
                ))}
              </div>
            </div>
          ))}
        </div>
      );
    }
    return (
      <div className="space-y-3">
        {step.details?.map((detail, idx) => (
          <div key={idx} className="bg-zinc-900/50 p-3 border border-zinc-800/50">
            <h5 className="text-orange-400 font-mono text-xs uppercase mb-1">{detail.title}</h5>
            <p className="text-xs text-zinc-300">{detail.text}</p>
          </div>
        ))}
      </div>
    );
  };

  return (
    <section id="protocol" className="py-16 md:py-24 border-b border-zinc-900 bg-zinc-950 overflow-hidden">
      <div className="max-w-7xl mx-auto px-4 md:px-6">

        {/* Header */}
        <div className="mb-12 md:mb-16">
          <div className="flex items-center gap-2 mb-4 text-orange-500">
            <ShieldCheck size={20} />
            <span className="font-mono text-xs md:text-sm uppercase tracking-widest">Protocol Overview</span>
          </div>
          <p className="text-lg md:text-2xl text-zinc-300 max-w-4xl font-light">
            SLSA + Sigstore protocol democratizes software signing. It allows developers to sign code securely using their existing identity, creating a verifiable, tamper-proof record of software provenance.
          </p>
        </div>

        {/* Info Cards Grid */}
        <div className="grid md:grid-cols-2 gap-4 md:gap-8 mb-12 md:mb-20">
          <div className="p-6 md:p-8 border border-zinc-800 bg-zinc-900/20 hover:border-zinc-700 transition-colors">
            <div className="flex items-center gap-3 mb-4 text-zinc-200">
              <Lock className="w-5 h-5 flex-shrink-0" />
              <h3 className="font-bold text-base md:text-lg">Motivation</h3>
            </div>
            <p className="text-sm md:text-base text-zinc-400 leading-relaxed">
              Traditional signing keys are notoriously hard to manage and often leaked.
              Sigstore eliminates key management by using ephemeral keys tied to OIDC identities (like GitHub Actions or email).
              This allows for automatic signing during CI/CD without ever storing long-lived secrets.
            </p>
          </div>
          <div className="p-6 md:p-8 border border-zinc-800 bg-zinc-900/20 hover:border-zinc-700 transition-colors">
            <div className="flex items-center gap-3 mb-4 text-zinc-200">
              <Eye className="w-5 h-5 flex-shrink-0" />
              <h3 className="font-bold text-base md:text-lg">Why Care?</h3>
            </div>
            <p className="text-sm md:text-base text-zinc-400 leading-relaxed">
              It prevents supply chain attacks and enables Proof of Provenance.
              You can cryptographically prove who built the software, when, and from which repository,
              ensuring artifacts haven't been tampered with since the build action.
            </p>
          </div>
        </div>

        {/* Interactive Flowchart */}
        <div>
          <h3 className="text-xs md:text-sm font-mono text-zinc-500 uppercase tracking-wider mb-4 md:mb-6">How it Works (Tap for details)</h3>

          {/* Mobile: Accordion style */}
          <div className="md:hidden space-y-3">
            {flowSteps.map((step) => (
              <div key={step.id} className="border border-zinc-800 bg-zinc-950 overflow-hidden">
                <button
                  onClick={() => setActiveStep(activeStep === step.id ? null : step.id)}
                  className={`w-full text-left p-4 flex items-center justify-between transition-colors ${
                    activeStep === step.id ? 'bg-zinc-900 border-b border-zinc-800' : ''
                  }`}
                >
                  <div className="flex items-center gap-3">
                    <div className={`${activeStep === step.id ? 'text-orange-500' : 'text-zinc-400'}`}>
                      {step.icon}
                    </div>
                    <div>
                      <h4 className="font-bold text-white text-sm">{step.title}</h4>
                      <p className="text-xs text-zinc-500">{step.desc}</p>
                    </div>
                  </div>
                  <ChevronDown
                    className={`w-5 h-5 text-zinc-500 transition-transform ${
                      activeStep === step.id ? 'rotate-180 text-orange-500' : ''
                    }`}
                  />
                </button>

                {/* Expanded content */}
                {activeStep === step.id && (
                  <div className="p-4 animate-in fade-in slide-in-from-top-2 duration-200">
                    {renderStepDetails(step)}
                  </div>
                )}
              </div>
            ))}
          </div>

          {/* Desktop: Grid with separate details panel */}
          <div className="hidden md:block">
            <div className="grid grid-cols-4 gap-4">
              {flowSteps.map((step) => (
                <button
                  key={step.id}
                  onClick={() => setActiveStep(activeStep === step.id ? null : step.id)}
                  className={`text-left p-6 border transition-all duration-300 relative group ${
                    activeStep === step.id
                      ? 'border-orange-500 bg-zinc-900'
                      : 'border-zinc-800 bg-zinc-950 hover:border-zinc-600'
                  }`}
                >
                  <div className={`mb-4 ${activeStep === step.id ? 'text-orange-500' : 'text-zinc-400 group-hover:text-zinc-200'}`}>
                    {step.icon}
                  </div>
                  <h4 className="font-bold text-white mb-1">{step.title}</h4>
                  <p className="text-sm text-zinc-500">{step.desc}</p>

                  {/* Arrow Connector */}
                  {step.id < 4 && (
                    <div className="absolute -right-3 top-1/2 -translate-y-1/2 z-10 text-zinc-700">
                      →
                    </div>
                  )}
                </button>
              ))}
            </div>

            {/* Details Panel - Desktop only */}
            <div className="mt-4 min-h-[180px] border-t border-zinc-900 pt-8 transition-all">
              {activeStep ? (
                <div className="animate-in fade-in slide-in-from-top-2 duration-300">
                  {(() => {
                    const step = flowSteps.find(s => s.id === activeStep);
                    if (!step) return null;

                    if (step.hasBranches && step.branches) {
                      return (
                        <div className="grid grid-cols-2 gap-8">
                          {step.branches.map((branch, branchIdx) => (
                            <div key={branchIdx} className="border border-zinc-800 p-6 bg-zinc-900/30">
                              <h5 className="text-orange-500 font-mono text-sm uppercase mb-4 border-b border-zinc-800 pb-2">{branch.label}</h5>
                              <div className="space-y-4">
                                {branch.details.map((detail, idx) => (
                                  <div key={idx}>
                                    <h6 className="text-zinc-200 font-medium text-sm mb-1">{detail.title}</h6>
                                    <p className="text-sm text-zinc-400">{detail.text}</p>
                                  </div>
                                ))}
                              </div>
                            </div>
                          ))}
                        </div>
                      );
                    }
                    return (
                      <div className="grid grid-cols-3 gap-6">
                        {step.details?.map((detail, idx) => (
                          <div key={idx} className="bg-zinc-900/50 p-4 border border-zinc-800/50">
                            <h5 className="text-orange-400 font-mono text-xs uppercase mb-2">{detail.title}</h5>
                            <p className="text-sm text-zinc-300">{detail.text}</p>
                          </div>
                        ))}
                      </div>
                    );
                  })()}
                </div>
              ) : (
                <div className="h-full flex items-center justify-center text-zinc-600 font-mono text-sm animate-pulse">
                  Click any step above to learn more about the protocol...
                </div>
              )}
            </div>
          </div>
        </div>

      </div>
    </section>
  );
};

export default ProtocolOverview;
