import React from 'react';
import { Analytics } from '@vercel/analytics/react';
import Navigation from './components/Navigation';
import Hero from './components/Hero';
import ProtocolOverview from './components/ProtocolOverview';
import Specification from './components/Specification';
import GettingStarted from './components/GettingStarted';
import ZkVerification from './components/ZkVerification';
import Integration from './components/Integration';
import Footer from './components/Footer';

const App: React.FC = () => {
  return (
    <div className="min-h-screen bg-black text-slate-200 font-sans selection:bg-orange-500/30 overflow-x-hidden">
      <Navigation />

      <main>
        <Hero />
        <ProtocolOverview />
        <Specification />
        <GettingStarted />
        <ZkVerification />
        <Integration />
      </main>

      <Footer />
      <Analytics />
    </div>
  );
};

export default App;
