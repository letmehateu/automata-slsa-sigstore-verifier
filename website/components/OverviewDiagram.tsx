import React, { useState } from 'react';
import { 
  ShieldCheck, 
  User, 
  FileBadge, 
  Database, 
  SearchCheck, 
  ArrowRight, 
  X,
  Lock,
  Eye,
  CheckCircle2,
  Clock, // Added Clock for TSA
  GlobeIcon
} from 'lucide-react';

const SigstoreExplainer = () => {
  const [activeStep, setActiveStep] = useState(null);

  // Data for the flowchart steps
  const steps = [
    {
      id: 'developer',
      title: 'Developer / CI',
      icon: <User className="w-8 h-8" />,
      color: 'bg-blue-600',
      description: 'The Initiator',
      details: [
        { title: 'OIDC Authentication', text: 'The developer or CI system (e.g., GitHub Actions) authenticates via an OpenID Connect provider (Google, GitHub).' },
        { title: 'Ephemeral Keys', text: 'A one-time key pair is generated. The private key is held only in memory and discarded immediately after signing.' },
        { title: 'Sign Artifact', text: 'The software artifact (binary, container) is signed using this ephemeral private key.' }
      ]
    },
    {
      id: 'fulcio',
      title: 'Fulcio',
      icon: <FileBadge className="w-8 h-8" />,
      color: 'bg-orange-600',
      description: 'Certificate Authority',
      details: [
        { title: 'Identity Verification', text: 'Fulcio verifies the OIDC token to confirm the user owns the email or identity claimed.' },
        { title: 'Certificate Issuance', text: 'Issues a short-lived X.509 certificate (valid for ~10 minutes) binding the public key to the identity.' },
        { title: 'No Key Management', text: 'Because keys are ephemeral, there are no long-lived secrets to store or leak.' }
      ]
    },
    {
      id: 'rekor',
      title: 'Rekor or TSA',
      icon: <Clock className="w-8 h-8" />,
      color: 'bg-amber-600',
      description: 'Transparency & Time',
      // New structure for split view
      branches: [
        {
          label: 'Public: Rekor Log',
          subIcon: <GlobeIcon className="w-5 h-5" />,
          items: [
            { title: 'Immutable Record', text: 'Signature and certificate are stored in a public, append-only transparency log.' },
            { title: 'Inclusion Proof', text: 'Returns a signed entry proving the event is public and cannot be altered.' }
          ]
        },
        {
          label: 'Private: TSA (RFC 3161)',
          subIcon: <Lock className="w-5 h-5" />,
          items: [
            { title: 'Timestamp Authority', text: 'Provides an RFC 3161 compliant timestamp proving the artifact existed at a specific time.' },
            { title: 'Certificate Chain', text: 'Verifies the timestamp signature against a trusted root, independent of a public log.' }
          ]
        }
      ]
    },
    {
      id: 'verifier',
      title: 'Verifier',
      icon: <SearchCheck className="w-8 h-8" />,
      color: 'bg-emerald-600',
      description: 'Policy Check',
      details: [
        { title: 'Signature Check', text: 'Verifies the digital signature using the public key embedded in the certificate.' },
        { title: 'Log/TSA Verification', text: 'Checks either the Rekor entry inclusion proof OR the TSA timestamp signature.' },
        { title: 'Policy Enforcement', text: 'Confirms the identity matches the policy (e.g., "Must be built by GitHub Actions on repo X").' }
      ]
    }
  ];

  return (
    <div className="min-h-screen bg-slate-900 text-slate-100 p-6 font-sans">
      <div className="max-w-5xl mx-auto space-y-12">
        
        {/* --- Section 1: Header & Overview --- */}
        <header className="space-y-6 text-center">
          <div className="inline-flex items-center space-x-3 bg-slate-800 px-4 py-2 rounded-full border border-slate-700">
            <ShieldCheck className="w-6 h-6 text-green-400" />
            <span className="font-semibold text-green-400 tracking-wide uppercase text-sm">Sigstore Protocol Overview</span>
          </div>
          
          <p className="text-lg text-slate-400 max-w-3xl mx-auto leading-relaxed">
            The Sigstore protocol democratizes software signing. It allows developers to sign code securely using their existing identity, creating a verifiable, tamper-proof record of software provenance.
          </p>
        </header>

        {/* --- Section 2: Motivation & Why Care --- */}
        <div className="grid md:grid-cols-2 gap-6">
          {/* Motivation Card */}
          <div className="bg-slate-800/50 border border-slate-700 p-6 rounded-xl hover:border-blue-500/50 transition-colors duration-300">
            <div className="flex items-center space-x-3 mb-4">
              <div className="p-2 bg-blue-500/10 rounded-lg">
                <Lock className="w-6 h-6 text-blue-400" />
              </div>
              <h2 className="text-xl font-bold text-blue-100">Motivation</h2>
            </div>
            <p className="text-slate-300 leading-relaxed">
              Traditional signing keys are notoriously hard to manage and often leaked. 
              Sigstore eliminates key management by using <span className="text-blue-300 font-medium">ephemeral keys</span> tied to OIDC identities (like GitHub Actions or email). 
              This allows for automatic signing during CI/CD without ever storing long-lived secrets.
            </p>
          </div>

          {/* Why Care Card */}
          <div className="bg-slate-800/50 border border-slate-700 p-6 rounded-xl hover:border-emerald-500/50 transition-colors duration-300">
            <div className="flex items-center space-x-3 mb-4">
              <div className="p-2 bg-emerald-500/10 rounded-lg">
                <Eye className="w-6 h-6 text-emerald-400" />
              </div>
              <h2 className="text-xl font-bold text-emerald-100">Why Care?</h2>
            </div>
            <p className="text-slate-300 leading-relaxed">
              It prevents supply chain attacks and enables <span className="text-emerald-300 font-medium">Proof of Provenance</span>. 
              You can cryptographically prove <em>who</em> built the software, <em>when</em>, and <em>from which repository</em>, 
              ensuring artifacts haven't been tampered with since the build action.
            </p>
          </div>
        </div>

        {/* --- Section 3: Interactive Flowchart --- */}
        <div className="space-y-8">
          <div className="flex items-center justify-between mb-8">
            <h3 className="text-2xl font-bold text-slate-200">How it Works</h3>
            <span className="text-sm text-slate-500 hidden md:block">Click any step below for details</span>
          </div>

          {/* Desktop/Tablet Flow */}
          <div className="relative">
            {/* Connecting Line (Absolute) */}
            <div className="hidden md:block absolute top-1/2 left-0 w-full h-1 bg-slate-800 -translate-y-1/2 z-0"></div>

            <div className="grid grid-cols-1 md:grid-cols-4 gap-6 relative z-10">
              {steps.map((step, index) => (
                <div key={step.id} className="flex flex-col items-center relative group">
                  
                  {/* Arrow Indicator for Flow (Mobile hidden) */}
                  {index < steps.length - 1 && (
                    <div className="hidden md:block absolute -right-3 top-1/2 -translate-y-1/2 z-0 text-slate-600">
                      <ArrowRight className="w-6 h-6" />
                    </div>
                  )}

                  {/* The Card */}
                  <button
                    onClick={() => setActiveStep(activeStep === step.id ? null : step.id)}
                    className={`
                      w-full relative p-6 rounded-xl border-2 transition-all duration-300 flex flex-col items-center text-center space-y-3
                      shadow-lg cursor-pointer outline-none focus:ring-4 focus:ring-opacity-50
                      ${activeStep === step.id 
                        ? `border-${step.color.replace('bg-', '')} bg-slate-800 scale-105 shadow-2xl` 
                        : 'border-slate-700 bg-slate-800/80 hover:border-slate-500 hover:bg-slate-800'
                      }
                    `}
                  >
                    {/* Step Number Badge */}
                    <div className="absolute -top-3 -right-3 w-8 h-8 rounded-full bg-slate-900 border border-slate-600 flex items-center justify-center font-bold text-sm text-slate-400">
                      {index + 1}
                    </div>

                    <div className={`p-4 rounded-full ${step.color} text-white shadow-inner`}>
                      {step.icon}
                    </div>
                    <div>
                      <h4 className="font-bold text-lg text-slate-100">{step.title}</h4>
                      <p className="text-sm text-slate-400">{step.description}</p>
                    </div>

                    {/* Active Indicator Triangle */}
                    {activeStep === step.id && (
                      <div className="absolute -bottom-4 left-1/2 -translate-x-1/2 w-0 h-0 border-l-[10px] border-l-transparent border-r-[10px] border-r-transparent border-t-[10px] border-t-slate-800 hidden md:block" />
                    )}
                  </button>
                  
                  {/* Mobile Down Arrow */}
                  {index < steps.length - 1 && (
                    <div className="md:hidden py-2 text-slate-600">
                      <ArrowRight className="w-6 h-6 rotate-90" />
                    </div>
                  )}
                </div>
              ))}
            </div>
          </div>

          {/* --- Section 4: Expandable Details Panel --- */}
          <div className={`
             transition-all duration-500 ease-in-out overflow-hidden
             ${activeStep ? 'opacity-100 max-h-[800px] mt-8' : 'opacity-0 max-h-0 mt-0'}
          `}>
            {activeStep && (() => {
              const currentStep = steps.find(s => s.id === activeStep);
              return (
                <div className="bg-slate-800 border border-slate-700 rounded-2xl p-8 shadow-2xl relative animate-in fade-in slide-in-from-top-4 duration-300">
                  <button 
                    onClick={() => setActiveStep(null)}
                    className="absolute top-4 right-4 p-2 hover:bg-slate-700 rounded-full transition-colors text-slate-400 hover:text-white"
                  >
                    <X className="w-5 h-5" />
                  </button>

                  <div className="flex flex-col md:flex-row gap-8">
                    {/* Left Column: Icon & Title */}
                    <div className="md:w-1/3 flex flex-col items-center justify-center text-center border-b md:border-b-0 md:border-r border-slate-700 pb-6 md:pb-0 md:pr-6">
                      <div className={`p-6 rounded-2xl ${currentStep.color} bg-opacity-20 mb-4`}>
                        {React.cloneElement(currentStep.icon, { className: `w-16 h-16 ${currentStep.color.replace('bg-', 'text-')}` })}
                      </div>
                      <h3 className="text-3xl font-bold text-white mb-2">{currentStep.title}</h3>
                      <p className="text-slate-400 font-medium">{currentStep.description}</p>
                    </div>

                    {/* Right Column: Details */}
                    <div className="md:w-2/3 space-y-6">
                      <h4 className="text-lg font-semibold text-slate-300 uppercase tracking-wider border-b border-slate-700 pb-2">
                        Process Details
                      </h4>

                      {/* Conditional Render: Split Branches (for Rekor/TSA) vs Standard List */}
                      {currentStep.branches ? (
                         <div className="grid md:grid-cols-2 gap-4">
                           {currentStep.branches.map((branch, i) => (
                             <div key={i} className="bg-slate-900/50 p-4 rounded-xl border border-slate-700">
                               <div className="flex items-center gap-2 mb-3 text-amber-400 font-bold">
                                 {branch.subIcon}
                                 <span>{branch.label}</span>
                               </div>
                               <ul className="space-y-3">
                                 {branch.items.map((item, j) => (
                                   <li key={j} className="text-sm">
                                      <span className="font-semibold text-slate-200 block">{item.title}</span>
                                      <span className="text-slate-400 text-xs leading-relaxed">{item.text}</span>
                                   </li>
                                 ))}
                               </ul>
                             </div>
                           ))}
                         </div>
                      ) : (
                        <ul className="space-y-4">
                          {currentStep.details.map((detail, idx) => (
                            <li key={idx} className="flex items-start gap-3">
                              <CheckCircle2 className="w-6 h-6 text-green-500 shrink-0 mt-0.5" />
                              <div>
                                <span className="font-bold text-white block mb-1">{detail.title}</span>
                                <span className="text-slate-400 leading-relaxed">{detail.text}</span>
                              </div>
                            </li>
                          ))}
                        </ul>
                      )}
                    </div>
                  </div>
                </div>
              );
            })()}
          </div>
          
          {/* Helper text if nothing selected */}
          {!activeStep && (
            <div className="text-center text-slate-600 italic py-8 animate-pulse">
              Select a stage above to verify the protocol steps...
            </div>
          )}
        </div>

      </div>
    </div>
  );
};

export default SigstoreExplainer;