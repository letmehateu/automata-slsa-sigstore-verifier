import React, { useState } from 'react';
import { SectionId } from './types';
import OverviewDiagram from './components/OverviewDiagram';
import GitHubIntegration from './components/GitHubIntegration';
import ZkVerifierDetails from './components/ZkVerifierDetails';
import CodeIntegration from './components/CodeIntegration';
import ChatInterface from './components/ChatInterface';
import { ShieldCheck, ChevronDown, Github, Terminal, Menu, X } from 'lucide-react';

const App: React.FC = () => {
  const [activeSection, setActiveSection] = useState<SectionId>(SectionId.HERO);
  const [mobileMenuOpen, setMobileMenuOpen] = useState(false);

  // Add smooth scrolling behavior
  React.useEffect(() => {
    document.documentElement.style.scrollBehavior = 'smooth';
    return () => {
      document.documentElement.style.scrollBehavior = 'auto';
    };
  }, []);

  return (
    <div className="min-h-screen bg-slate-950 text-slate-200 selection:bg-orange-500/30">
      
      {/* Navigation */}
      <nav className="fixed top-0 w-full z-50 bg-slate-950/80 backdrop-blur-md border-b border-slate-800">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
          <div className="flex items-center justify-between h-16">
            <div className="flex items-center gap-2">
              <ShieldCheck className="w-8 h-8 text-orange-500" />
              <span className="font-bold text-xl tracking-tight text-white"><span className="text-orange-400">Automata</span> Sigstore Attest</span>
            </div>

            {/* Desktop navigation */}
            <div className="hidden md:block">
              <div className="flex items-baseline space-x-4">
                <a href="#overview" className="hover:text-white px-3 py-2 rounded-md text-sm font-medium transition-colors">Overview</a>
                <a href="#bundles" className="hover:text-white px-3 py-2 rounded-md text-sm font-medium transition-colors">Get Started</a>
                <a href="#verifier" className="hover:text-white px-3 py-2 rounded-md text-sm font-medium transition-colors">ZK Verification</a>
                <a href="#integration" className="hover:text-white px-3 py-2 rounded-md text-sm font-medium transition-colors">Integration</a>
              </div>
            </div>

            {/* Mobile hamburger button */}
            <button
              className="md:hidden p-2 rounded-md text-slate-400 hover:text-white hover:bg-slate-800 transition-colors"
              onClick={() => setMobileMenuOpen(!mobileMenuOpen)}
              aria-label="Toggle menu"
            >
              {mobileMenuOpen ? <X className="w-6 h-6" /> : <Menu className="w-6 h-6" />}
            </button>
          </div>
        </div>

        {/* Mobile menu */}
        {mobileMenuOpen && (
          <div className="md:hidden bg-slate-950 border-b border-slate-800">
            <div className="px-4 py-4 space-y-2">
              <a
                href="#overview"
                onClick={() => setMobileMenuOpen(false)}
                className="block px-4 py-3 rounded-lg text-slate-200 hover:bg-slate-800 hover:text-white transition-colors font-medium"
              >
                Overview
              </a>
              <a
                href="#bundles"
                onClick={() => setMobileMenuOpen(false)}
                className="block px-4 py-3 rounded-lg text-slate-200 hover:bg-slate-800 hover:text-white transition-colors font-medium"
              >
                Get Started
              </a>
              <a
                href="#verifier"
                onClick={() => setMobileMenuOpen(false)}
                className="block px-4 py-3 rounded-lg text-slate-200 hover:bg-slate-800 hover:text-white transition-colors font-medium"
              >
                ZK Verification
              </a>
              <a
                href="#integration"
                onClick={() => setMobileMenuOpen(false)}
                className="block px-4 py-3 rounded-lg text-slate-200 hover:bg-slate-800 hover:text-white transition-colors font-medium"
              >
                Integration
              </a>
            </div>
          </div>
        )}
      </nav>

      <main className="pt-16">
        
        {/* Hero Section */}
        <section id="overview" className="relative min-h-[70vh] flex items-center justify-center overflow-hidden scroll-mt-16">
            {/* Background Gradients */}
            <div className="absolute top-0 left-1/4 w-96 h-96 bg-orange-600/20 rounded-full blur-[128px] pointer-events-none" />
            <div className="absolute bottom-0 right-1/4 w-96 h-96 bg-amber-600/10 rounded-full blur-[128px] pointer-events-none" />

            <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 relative z-10 text-center">
                <div className="inline-flex items-center gap-2 px-4 py-2 rounded-full bg-slate-900 border border-slate-700 text-sm text-orange-300 mb-8">
                    <span className="relative flex h-2 w-2">
                      <span className="animate-ping absolute inline-flex h-full w-full rounded-full bg-orange-400 opacity-75"></span>
                      <span className="relative inline-flex rounded-full h-2 w-2 bg-orange-500"></span>
                    </span>
                    Running on RISC0, SP1 & Pico
                </div>
                <h1 className="text-4xl sm:text-5xl md:text-7xl font-bold text-white tracking-tight mb-6">
                    Trust your software.<br/>
                    <span className="text-transparent bg-clip-text bg-gradient-to-r from-orange-400 to-amber-400">Prove it on-chain.</span>
                </h1>
                <p className="mt-4 max-w-2xl mx-auto text-xl text-slate-400">
                    The missing link between GitHub Actions and Smart Contracts. 
                    We verify Sigstore attestations inside zkVMs to bring software supply chain security to the blockchain.
                </p>
                <div className="mt-10 flex flex-col sm:flex-row justify-center items-center gap-4 px-4 sm:px-0">
                    <a href="#protocol" className="w-full sm:w-auto text-center px-8 py-3 rounded-lg bg-white text-slate-900 font-semibold hover:bg-slate-200 transition-colors">
                        Learn the Protocol
                    </a>
                    <a href="#verifier" className="w-full sm:w-auto text-center px-8 py-3 rounded-lg bg-slate-800 text-white font-semibold hover:bg-slate-700 transition-colors border border-slate-700 flex items-center justify-center gap-2">
                        <Terminal size={18} />
                        View Verifier
                    </a>
                </div>
            </div>

            <div className="absolute bottom-10 left-1/2 transform -translate-x-1/2 animate-bounce text-slate-500">
                <ChevronDown />
            </div>
        </section>

        {/* Protocol Overview */}
        <section id="protocol" className="py-12 bg-slate-950 scroll-mt-16">
            <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
                <OverviewDiagram />
            </div>
        </section>

        {/* Getting Started */}
        <section id="bundles" className="py-20 bg-slate-900 border-y border-slate-800 scroll-mt-16">
            <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
                <div className="flex flex-col md:flex-row md:items-end justify-between mb-12">
                    <div className="max-w-xl">
                        <h2 className="text-3xl font-bold text-white mb-4">Getting Started: Bundles & Trust Roots</h2>
                        <p className="text-slate-400">
                            Generate attestations with GitHub Actions, download bundles with GitHub CLI, and fetch trust roots for verification.
                            Our implementation supports both public and private repository workflows.
                        </p>
                    </div>
                </div>
                
                <GitHubIntegration />
            </div>
        </section>

        {/* ZK Verification */}
        <section id="verifier" className="py-24 bg-slate-950 relative scroll-mt-16">
             <div className="absolute top-0 right-0 w-1/3 h-full bg-blue-900/5 pointer-events-none" />
             <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 relative z-10">
                <div className="mb-16">
                    <h2 className="text-3xl font-bold text-white mb-4">Zero-Knowledge Verification</h2>
                    <p className="text-slate-400 max-w-3xl">
                        We built a portable Rust library that verifies Sigstore bundles within Zero-Knowledge Virtual Machines (RISC0, SP1, and Pico).
                        The verifier generates Groth16 SNARK proofs containing certificate hashes, OIDC identity, and timestamp data,
                        enabling <span className="text-orange-400 font-semibold">Proof of Provenance</span> on Ethereum and other blockchains.
                    </p>
                </div>
                
                <ZkVerifierDetails />
             </div>
        </section>

        {/* Code Integration */}
        <section id="integration" className="py-20 bg-slate-900 border-y border-slate-800 scroll-mt-16">
            <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
              <CodeIntegration />
            </div>
        </section>

        {/* AI FAQ */}
        {/* <section id="faq" className="py-20 bg-slate-900 border-t border-slate-800 scroll-mt-16">
             <div className="max-w-4xl mx-auto px-4 sm:px-6 lg:px-8">
                <div className="text-center mb-10">
                    <h2 className="text-2xl font-bold text-white mb-2">Still have questions?</h2>
                    <p className="text-slate-400">Ask our AI assistant about specifics regarding Fulcio, Rekor, or the zkVM implementation.</p>
                </div>
                <ChatInterface />
             </div>
        </section> */}

        <footer className="bg-slate-950 py-12 border-t border-slate-800 text-center">
            <p className="text-slate-500 text-sm">© 2025 Sigstore Attest. Built for the community.</p>
        </footer>

      </main>
    </div>
  );
};

export default App;
