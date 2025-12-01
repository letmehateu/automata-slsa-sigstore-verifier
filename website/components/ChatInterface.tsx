import React, { useState, useRef, useEffect } from 'react';
import { Send, Bot, User, Loader2, Sparkles } from 'lucide-react';
import { ChatMessage } from '../types';
import { sendMessageToGemini } from '../services/geminiService';

const SUGGESTED_QUESTIONS = [
  "What is the difference between Fulcio and Rekor?",
  "How does the zkVM verification work?",
  "Why do I need a Transparency Log?",
  "Tell me about the Rust implementation."
];

const ChatInterface: React.FC = () => {
  const [messages, setMessages] = useState<ChatMessage[]>([
    {
      id: 'welcome',
      role: 'model',
      text: "Hi! I'm your Sigstore & zkVM guide. Ask me anything about the protocol or our verifier implementation.",
      timestamp: new Date()
    }
  ]);
  const [input, setInput] = useState('');
  const [loading, setLoading] = useState(false);
  const messagesEndRef = useRef<HTMLDivElement>(null);

  const scrollToBottom = () => {
    messagesEndRef.current?.scrollIntoView({ behavior: "smooth" });
  };

  useEffect(() => {
    scrollToBottom();
  }, [messages]);

  const handleSend = async (text: string) => {
    if (!text.trim() || loading) return;

    const userMsg: ChatMessage = {
      id: Date.now().toString(),
      role: 'user',
      text: text,
      timestamp: new Date()
    };

    setMessages(prev => [...prev, userMsg]);
    setInput('');
    setLoading(true);

    try {
      const responseText = await sendMessageToGemini(text);
      
      const botMsg: ChatMessage = {
        id: (Date.now() + 1).toString(),
        role: 'model',
        text: responseText,
        timestamp: new Date()
      };
      setMessages(prev => [...prev, botMsg]);
    } catch (err) {
      console.error(err);
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="bg-slate-900 rounded-xl border border-slate-700 overflow-hidden flex flex-col h-[600px] shadow-2xl">
      <div className="bg-slate-800 p-4 border-b border-slate-700 flex items-center gap-2">
        <Sparkles className="w-5 h-5 text-indigo-400" />
        <h3 className="font-semibold text-white">AI Assistant</h3>
      </div>

      <div className="flex-1 overflow-y-auto p-4 space-y-4">
        {messages.map((msg) => (
          <div key={msg.id} className={`flex gap-3 ${msg.role === 'user' ? 'justify-end' : 'justify-start'}`}>
            {msg.role === 'model' && (
              <div className="w-8 h-8 rounded-full bg-indigo-600 flex items-center justify-center shrink-0">
                <Bot size={16} className="text-white" />
              </div>
            )}
            
            <div className={`max-w-[80%] rounded-lg p-3 text-sm leading-relaxed ${
              msg.role === 'user' 
                ? 'bg-blue-600 text-white' 
                : 'bg-slate-800 text-slate-200'
            }`}>
              {msg.text}
            </div>

            {msg.role === 'user' && (
              <div className="w-8 h-8 rounded-full bg-slate-700 flex items-center justify-center shrink-0">
                <User size={16} className="text-slate-300" />
              </div>
            )}
          </div>
        ))}
        {loading && (
          <div className="flex gap-3">
             <div className="w-8 h-8 rounded-full bg-indigo-600 flex items-center justify-center shrink-0">
                <Bot size={16} className="text-white" />
              </div>
              <div className="bg-slate-800 rounded-lg p-3 flex items-center">
                <Loader2 className="w-4 h-4 text-slate-400 animate-spin" />
              </div>
          </div>
        )}
        <div ref={messagesEndRef} />
      </div>

      {messages.length < 3 && (
        <div className="px-4 pb-2 flex flex-wrap gap-2">
            {SUGGESTED_QUESTIONS.map((q, i) => (
                <button 
                    key={i}
                    onClick={() => handleSend(q)}
                    className="text-xs bg-slate-800 hover:bg-slate-700 text-indigo-300 px-3 py-1 rounded-full transition-colors border border-slate-700"
                >
                    {q}
                </button>
            ))}
        </div>
      )}

      <div className="p-4 bg-slate-800 border-t border-slate-700">
        <div className="flex gap-2">
          <input
            type="text"
            value={input}
            onChange={(e) => setInput(e.target.value)}
            onKeyDown={(e) => e.key === 'Enter' && handleSend(input)}
            placeholder="Ask about Sigstore or the Verifier..."
            className="flex-1 bg-slate-900 border border-slate-600 rounded-lg px-4 py-2 text-white placeholder-slate-500 focus:outline-none focus:ring-2 focus:ring-blue-500"
          />
          <button
            onClick={() => handleSend(input)}
            disabled={loading || !input.trim()}
            className="bg-blue-600 hover:bg-blue-500 disabled:opacity-50 text-white rounded-lg p-2 transition-colors"
          >
            <Send size={20} />
          </button>
        </div>
      </div>
    </div>
  );
};

export default ChatInterface;
