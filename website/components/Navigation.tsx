import React, { useState, useEffect } from 'react';
import { Menu, X, ChevronDown } from 'lucide-react';

interface NavLink {
  label: string;
  href: string;
  children?: { label: string; href: string }[];
}

const Navigation: React.FC = () => {
  const [isOpen, setIsOpen] = useState(false);
  const [scrolled, setScrolled] = useState(false);
  const [specDropdownOpen, setSpecDropdownOpen] = useState(false);
  const [mobileSpecOpen, setMobileSpecOpen] = useState(false);

  useEffect(() => {
    const handleScroll = () => {
      setScrolled(window.scrollY > 20);
    };
    window.addEventListener('scroll', handleScroll);
    return () => window.removeEventListener('scroll', handleScroll);
  }, []);

  const links: NavLink[] = [
    { label: 'Overview', href: '#protocol' },
    {
      label: 'Specification',
      href: '#specification',
      children: [
        { label: 'SLSA Provenance', href: '#specification-slsa' },
        { label: 'OIDC & Fulcio', href: '#specification-oidc-fulcio' },
        { label: 'Timestamping', href: '#specification-signature-timestamping' },
      ]
    },
    { label: 'Get Started', href: '#bundles' },
    { label: 'ZK Verification', href: '#verifier' },
    { label: 'Integration', href: '#integration' },
  ];

  return (
    <nav className={`fixed top-0 left-0 right-0 z-50 border-b transition-all duration-300 ${scrolled ? 'border-zinc-800 bg-black/90 backdrop-blur-md py-4' : 'border-transparent bg-transparent py-6'}`}>
      <div className="max-w-7xl mx-auto px-6 flex items-center justify-between">

        {/* Logo */}
        <a href="#" className="flex items-center gap-2 group">
          <div className="font-sans font-bold text-xl tracking-tight">
            <span className="text-orange-500 group-hover:text-orange-400 transition-colors">Automata</span>
            <span className="text-white ml-2">SLSA Attestation</span>
          </div>
        </a>

        {/* Desktop Links */}
        <div className="hidden md:flex items-center gap-8">
          {links.map((link) => (
            link.children ? (
              <div
                key={link.label}
                className="relative"
                onMouseEnter={() => setSpecDropdownOpen(true)}
                onMouseLeave={() => setSpecDropdownOpen(false)}
              >
                <a
                  href={link.href}
                  className="text-sm font-medium text-zinc-400 hover:text-white transition-colors uppercase tracking-wide flex items-center gap-1"
                >
                  {link.label}
                  <ChevronDown size={14} className={`transition-transform duration-200 ${specDropdownOpen ? 'rotate-180' : ''}`} />
                </a>

                {/* Dropdown Menu */}
                {specDropdownOpen && (
                  <div className="absolute top-full left-0 pt-2">
                    <div className="bg-zinc-900 border border-zinc-800 rounded-lg py-2 min-w-[180px] shadow-xl">
                      {link.children.map((child) => (
                        <a
                          key={child.label}
                          href={child.href}
                          className="block px-4 py-2 text-sm text-zinc-400 hover:text-white hover:bg-zinc-800 transition-colors"
                        >
                          {child.label}
                        </a>
                      ))}
                    </div>
                  </div>
                )}
              </div>
            ) : (
              <a
                key={link.label}
                href={link.href}
                className="text-sm font-medium text-zinc-400 hover:text-white transition-colors uppercase tracking-wide"
              >
                {link.label}
              </a>
            )
          ))}
        </div>

        {/* Mobile Toggle */}
        <button
          className="md:hidden text-zinc-400 hover:text-white"
          onClick={() => setIsOpen(!isOpen)}
        >
          {isOpen ? <X /> : <Menu />}
        </button>
      </div>

      {/* Mobile Menu */}
      {isOpen && (
        <div className="md:hidden absolute top-full left-0 right-0 border-b border-zinc-800 bg-black">
          <div className="flex flex-col p-6 space-y-4">
            {links.map((link) => (
              link.children ? (
                <div key={link.label}>
                  <button
                    className="flex items-center justify-between w-full text-lg font-medium text-zinc-400 hover:text-white"
                    onClick={() => setMobileSpecOpen(!mobileSpecOpen)}
                  >
                    {link.label}
                    <ChevronDown size={18} className={`transition-transform duration-200 ${mobileSpecOpen ? 'rotate-180' : ''}`} />
                  </button>

                  {/* Mobile Accordion */}
                  {mobileSpecOpen && (
                    <div className="mt-2 ml-4 space-y-2 border-l border-zinc-800 pl-4">
                      {link.children.map((child) => (
                        <a
                          key={child.label}
                          href={child.href}
                          className="block text-base text-zinc-500 hover:text-white"
                          onClick={() => setIsOpen(false)}
                        >
                          {child.label}
                        </a>
                      ))}
                    </div>
                  )}
                </div>
              ) : (
                <a
                  key={link.label}
                  href={link.href}
                  className="text-lg font-medium text-zinc-400 hover:text-white"
                  onClick={() => setIsOpen(false)}
                >
                  {link.label}
                </a>
              )
            ))}
          </div>
        </div>
      )}
    </nav>
  );
};

export default Navigation;
