import React from 'react';
import HalftoneMonolith from './ui/HalftoneMonolith';
import { Terminal } from 'lucide-react';

const Hero: React.FC = () => {
  return (
    <section id="overview" className="relative min-h-screen pt-24 md:pt-0 flex items-center border-b border-zinc-900 bg-black">

      {/* Background Grid Pattern */}
      <div className="absolute inset-0 bg-[url('https://grainy-gradients.vercel.app/noise.svg')] opacity-10 pointer-events-none"></div>
      <div className="absolute inset-0 bg-[linear-gradient(rgba(255,255,255,0.02)_1px,transparent_1px),linear-gradient(90deg,rgba(255,255,255,0.02)_1px,transparent_1px)] bg-[size:100px_100px] pointer-events-none"></div>

      <div className="max-w-7xl mx-auto px-6 w-full flex flex-col md:flex-row items-center z-10">
        {/* Text Content */}
        <div className="w-full md:w-1/2 flex flex-col justify-center h-full pt-12 md:pt-0">
        
        {/* Badge */}
        <div className="inline-flex items-center gap-2 px-3 py-1 mb-8 border border-zinc-800 rounded-full bg-zinc-950/50 w-fit">
          <span className="relative flex h-2 w-2">
            <span className="animate-ping absolute inline-flex h-full w-full rounded-full bg-orange-400 opacity-75"></span>
            <span className="relative inline-flex rounded-full h-2 w-2 bg-orange-500"></span>
          </span>
          <span className="text-xs font-mono text-zinc-400 uppercase tracking-wider">
            Running on RISC0, SP1 & Pico
          </span>
        </div>

        <h1 className="text-5xl md:text-7xl font-bold tracking-tighter leading-[1.1] mb-6 text-white">
          Trust your <br />
          software. <br />
          <span className="bg-gradient-to-r from-orange-400 to-amber-600 bg-clip-text text-transparent">
            Prove it on-chain.
          </span>
        </h1>

        <p className="text-lg md:text-xl text-zinc-400 max-w-xl leading-relaxed mb-10 font-light">
          The missing link between GitHub Actions and Smart Contracts.
          We verify SLSA Attestations using Sigstore inside zkVMs to bring software supply chain security to the blockchain.
        </p>

        <div className="flex flex-col sm:flex-row gap-4">
          <a
            href="#protocol"
            className="px-8 py-4 bg-white text-black font-semibold hover:bg-zinc-200 transition-colors text-center"
          >
            Learn the Protocol
          </a>
          <a
            href="#verifier"
            className="px-8 py-4 border border-zinc-700 text-white font-medium hover:border-orange-500 hover:text-orange-500 transition-colors flex items-center justify-center gap-2 group"
          >
            <Terminal size={18} className="group-hover:text-orange-500" />
            View Verifier
          </a>
        </div>
        </div>

        {/* Visual Content - Halftone Monolith */}
        <div className="w-full md:w-1/2 h-[60vh] md:h-[80vh] flex items-center justify-center relative">
          <HalftoneMonolith />
          {/* Subtle overlay to blend canvas edges */}
          <div className="absolute inset-0 bg-gradient-to-t from-black via-transparent to-transparent opacity-80 pointer-events-none md:hidden" />
        </div>
      </div>

    </section>
  );
};

export default Hero;
