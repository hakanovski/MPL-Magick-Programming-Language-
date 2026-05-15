'use client';

import React, { useState, useRef, useEffect } from 'react';
import { Send, Terminal, Loader2, Cpu } from 'lucide-react';

interface ChatMessage {
  role: 'user' | 'mpl_servitor';
  content: string;
  code?: string;
  visual_sigil?: string;
  RitualSeal?: string;
}

export function MplServitorChat() {
  const [messages, setMessages] = useState<ChatMessage[]>([
    {
      role: 'mpl_servitor',
      content: 'ONLINE. I am the MPL Servitor. State your esoteric intent, and I shall translate it into a MagickScript execution manifest.',
    },
  ]);
  const [input, setInput] = useState('');
  const [isLoading, setIsLoading] = useState(false);
  const messagesEndRef = useRef<HTMLDivElement>(null);

  const scrollToBottom = () => {
    messagesEndRef.current?.scrollIntoView({ behavior: 'smooth' });
  };

  useEffect(() => {
    scrollToBottom();
  }, [messages]);

  const handleSend = async () => {
    if (!input.trim() || isLoading) return;

    const userMessage: ChatMessage = { role: 'user', content: input };
    setMessages((prev) => [...prev, userMessage]);
    setInput('');
    setIsLoading(true);

    try {
      // Intended call to the manifestation endpoint
      const response = await fetch('/api/manifest_from_text', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ intent: userMessage.content }),
      });

      if (!response.ok) {
        throw new Error('Manifestation failed at the API layer.');
      }

      const data = await response.json();
      
      const servitorMessage: ChatMessage = {
        role: 'mpl_servitor',
        content: data.explanation || 'Intent successfully synthesized into MagickScript.',
        code: data.code,
        visual_sigil: data.visual_sigil,
        RitualSeal: data.RitualSeal,
      };

      setMessages((prev) => [...prev, servitorMessage]);
    } catch (error) {
       console.error("Servitor Error:", error);
       // Mock response for now if the API route is not fully ready
       setTimeout(() => {
          setMessages((prev) => [...prev, {
            role: 'mpl_servitor',
            content: 'Intent registered, but the Astral Web disconnected. Simulating a basic MS struct:',
            code: `sacrifice _intent_ = "${userMessage.content.replace(/"/g, '\\"')}"\ninvoke synthesize(_intent_)`,
            visual_sigil: '❂',
            RitualSeal: '0x00F8A_VOID_SEAL'
          }]);
          setIsLoading(false);
       }, 800);
       return;
    }

    setIsLoading(false);
  };

  return (
    <div className="flex h-full flex-col border border-white/10 bg-black shadow-[0_0_20px_rgba(168,85,247,0.05)] rounded-md overflow-hidden relative">
      <div className="flex items-center justify-between border-b border-white/10 bg-white/5 px-4 py-2 opacity-90">
        <div className="flex items-center gap-2">
          <Terminal className="h-4 w-4 text-purple-500" />
          <h2 className="text-xs font-bold uppercase tracking-widest text-purple-400">
            MPL Servitor // Cortex Interface
          </h2>
        </div>
        <div className="flex items-center gap-2">
          <div className="h-1.5 w-1.5 rounded-full bg-green-500 shadow-[0_0_5px_rgba(34,197,94,0.8)] animate-pulse" />
          <span className="text-[10px] text-white/40 uppercase font-bold tracking-widest text-green-400/80">Active</span>
        </div>
      </div>

      <div className="flex-1 overflow-y-auto p-4 space-y-4 font-mono text-sm">
        {messages.map((msg, idx) => (
          <div
            key={idx}
            className={`flex flex-col max-w-[90%] ${
              msg.role === 'user' ? 'self-end items-end' : 'self-start items-start'
            }`}
          >
            <div className="flex items-center gap-2 mb-1.5 opacity-60 px-1">
              {msg.role === 'mpl_servitor' && <Cpu className="h-3 w-3 text-purple-500" />}
              <span className="text-[10px] uppercase tracking-wider">
                {msg.role === 'user' ? 'Operator' : 'Servitor'}
              </span>
            </div>
            <div
              className={`rounded-sm px-4 py-3 border ${
                msg.role === 'user'
                  ? 'bg-blue-900/20 border-blue-500/30 text-blue-200'
                  : 'bg-purple-900/10 border-purple-500/20 text-purple-100'
              }`}
            >
              <div className="leading-relaxed whitespace-pre-wrap">{msg.content}</div>
              
              {msg.code && (
                <div className="mt-4 border border-white/10 bg-black/60 rounded p-3">
                   <div className="text-[10px] text-white/40 uppercase mb-2 border-b border-white/10 pb-1">.ms / MagickScript Output</div>
                   <pre className="text-green-400 font-mono text-xs overflow-x-auto">
                     {msg.code}
                   </pre>
                </div>
              )}
              
              {(msg.visual_sigil || msg.RitualSeal) && (
                <div className="mt-3 flex items-center justify-between gap-4 border-t border-purple-500/20 pt-2 text-[10px]">
                  {msg.visual_sigil && <span className="text-xl opacity-80">{msg.visual_sigil}</span>}
                  {msg.RitualSeal && <span className="text-purple-400/60 uppercase tracking-widest font-bold font-mono">SEAL_REF: <span className="text-white/80">{msg.RitualSeal}</span></span>}
                </div>
              )}
            </div>
          </div>
        ))}
        {isLoading && (
          <div className="self-start flex items-center gap-2 text-purple-500/70 p-2">
            <Loader2 className="h-4 w-4 animate-spin" />
            <span className="text-xs uppercase tracking-widest">Synthesizing Intent...</span>
          </div>
        )}
        <div ref={messagesEndRef} />
      </div>

      <div className="border-t border-white/10 bg-white/5 p-3 flex gap-2">
        <input
          type="text"
          value={input}
          onChange={(e) => setInput(e.target.value)}
          onKeyDown={(e) => e.key === 'Enter' && handleSend()}
          placeholder="ENTER NEURAL PROMPT..."
          suppressHydrationWarning
          className="flex-1 bg-black/50 border border-white/10 rounded-sm px-4 py-2 font-mono text-sm text-white/90 placeholder-white/20 focus:outline-none focus:border-purple-500/50 focus:ring-1 focus:ring-purple-500/50 transition-all font-mono uppercase"
        />
        <button
          onClick={handleSend}
          disabled={!input.trim() || isLoading}
          className="bg-purple-900/30 hover:bg-purple-900/60 border border-purple-500/30 text-purple-300 disabled:opacity-40 disabled:cursor-not-allowed px-4 py-2 rounded-sm transition-all focus:outline-none flex items-center justify-center min-w-[50px]"
        >
          {isLoading ? <Loader2 className="h-4 w-4 animate-spin" /> : <Send className="h-4 w-4" />}
        </button>
      </div>
    </div>
  );
}
