import React, { useState } from 'react';
import { ChevronDown, ChevronUp, Copy, Check } from 'lucide-react';

interface CodeExampleProps {
  language: 'bash' | 'yaml' | 'json';
  title: string;
  code: string;
  defaultExpanded?: boolean;
}

const CodeExample: React.FC<CodeExampleProps> = ({ language, title, code, defaultExpanded = false }) => {
  const [isExpanded, setIsExpanded] = useState(defaultExpanded);
  const [isCopied, setIsCopied] = useState(false);

  const handleCopy = async () => {
    await navigator.clipboard.writeText(code);
    setIsCopied(true);
    setTimeout(() => setIsCopied(false), 2000);
  };

  return (
    <div className="bg-slate-950/50 rounded-lg border border-slate-800 overflow-hidden">
      <button
        onClick={() => setIsExpanded(!isExpanded)}
        className="w-full px-4 py-3 flex items-center justify-between text-left hover:bg-slate-900/30 transition-colors"
      >
        <span className="text-sm font-medium text-slate-300">{title}</span>
        <div className="flex items-center gap-2">
          <span className="text-xs text-slate-500 uppercase">{language}</span>
          {isExpanded ? (
            <ChevronUp className="w-4 h-4 text-slate-400" />
          ) : (
            <ChevronDown className="w-4 h-4 text-slate-400" />
          )}
        </div>
      </button>

      {isExpanded && (
        <div className="border-t border-slate-800 relative">
          <button
            onClick={handleCopy}
            className="absolute top-2 right-2 p-2 rounded bg-slate-800 hover:bg-slate-700 transition-colors z-10"
            title="Copy to clipboard"
          >
            {isCopied ? (
              <Check className="w-4 h-4 text-emerald-400" />
            ) : (
              <Copy className="w-4 h-4 text-slate-400" />
            )}
          </button>
          <pre className="p-4 overflow-x-auto text-sm">
            <code className={`language-${language} text-slate-300`}>
              {code}
            </code>
          </pre>
        </div>
      )}
    </div>
  );
};

export default CodeExample;
