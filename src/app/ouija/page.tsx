'use client';

import React, { useState, useEffect, useRef } from 'react';
import { activeMplClient, AkashicResponse } from '../../sdk/mpl_client';
import { Planchette } from '../../components/Planchette';
import { Cpu, Terminal, Zap, Activity } from 'lucide-react';
import { motion, AnimatePresence } from 'motion/react';
import { cn } from '../../components/Planchette'; // Reusing utility

export default function AkashicOuija() {
  const [question, setQuestion] = useState('');
  const [isResonating, setIsResonating] = useState(false);
  const [history, setHistory] = useState<{ q: string, r: AkashicResponse }[]>([]);
  const [entropyStats, setEntropyStats] = useState({ poolSize: 0, lastX: 0, lastY: 0 });
  
  const entropyBuffer = useRef<number[]>([]);
  const MAX_ENTROPY_BUFFER = 100;

  // Capture Physical Entropy Matrix (Client-side tracking)
  useEffect(() => {
    const handleEntropyMove = (e: MouseEvent | TouchEvent) => {
      let shiftX = 0;
      let shiftY = 0;
      let clientX = 0;
      let clientY = 0;

      if (e.type === 'mousemove') {
        const mouseEvent = e as MouseEvent;
        shiftX = Math.abs(mouseEvent.movementX);
        shiftY = Math.abs(mouseEvent.movementY);
        clientX = mouseEvent.clientX;
        clientY = mouseEvent.clientY;
      } else if (e.type === 'touchmove') {
        const touchEvent = e as TouchEvent;
        const touch = touchEvent.touches[0];
        clientX = touch.clientX;
        clientY = touch.clientY;
        // Approximation for touch movement entropy
        shiftX = Math.abs(clientX - entropyStats.lastX);
        shiftY = Math.abs(clientY - entropyStats.lastY);
      }

      const entropy = shiftX + shiftY;
      
      if (entropy > 0) {
        entropyBuffer.current.push(entropy);
        if (entropyBuffer.current.length > MAX_ENTROPY_BUFFER) {
          entropyBuffer.current.shift();
        }
        setEntropyStats({
          poolSize: entropyBuffer.current.length,
          lastX: clientX,
          lastY: clientY
        });
      }
    };

    window.addEventListener('mousemove', handleEntropyMove);
    window.addEventListener('touchmove', handleEntropyMove);
    return () => {
      window.removeEventListener('mousemove', handleEntropyMove);
      window.removeEventListener('touchmove', handleEntropyMove);
    };
  }, [entropyStats.lastX, entropyStats.lastY]);

  const handleAsk = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!question.trim() || isResonating) return;

    setIsResonating(true);
    
    // Extract a copy of the current entropy buffer to send to OVM
    const payload = [...entropyBuffer.current];
    
    // Artificial latency for dramatic esoteric effect (Simulating Neural/OVM compilation)
    await new Promise(resolve => setTimeout(resolve, 1200 + Math.random() * 800));
    
    // Invoke MPL Client
    const response = await activeMplClient.ask_akashic(question, payload);
    
    setHistory(prev => [{ q: question, r: response }, ...prev].slice(0, 50));
    setQuestion('');
    setIsResonating(false);
    
    // Consume entropy pipeline
    entropyBuffer.current = entropyBuffer.current.slice(Math.floor(payload.length / 2));
  };

  const latestResponse = history[0]?.r;

  return (
    <div suppressHydrationWarning className="min-h-screen bg-[#030303] text-[#e0e0e0] font-mono selection:bg-purple-900/50 flex flex-col relative overflow-hidden">
      {/* Background ambient grid (OVM Schematic Motif) */}
      <div className="absolute inset-0 bg-[linear-gradient(rgba(255,255,255,0.02)_1px,transparent_1px),linear-gradient(90deg,rgba(255,255,255,0.02)_1px,transparent_1px)] bg-[size:40px_40px] [mask-image:radial-gradient(ellipse_50%_50%_at_50%_50%,#000_70%,transparent_100%)] pointer-events-none" />
      
      <header className="flex h-14 items-center justify-between border-b border-white/10 px-6 shrink-0 relative z-10 backdrop-blur-sm bg-black/40">
        <div className="flex items-center gap-3">
          <Terminal className="h-4 w-4 text-purple-500" />
          <span className="text-xs font-bold tracking-[0.2em] text-white/90">AKASHIC OUIJA</span>
        </div>
        <div className="flex items-center gap-6 text-[10px] text-white/50 tracking-widest uppercase">
          <div className="flex items-center gap-2 bg-white/5 px-2.5 py-1 rounded-sm border border-white/5">
            <Cpu className="h-3 w-3 text-white/40" />
            <span>OVM LINK: SECURE</span>
          </div>
          <div className="flex items-center gap-2 bg-white/5 px-2.5 py-1 rounded-sm border border-white/5">
            <div className={cn("h-1.5 w-1.5 rounded-full", entropyStats.poolSize > 50 ? "bg-green-500 shadow-[0_0_8px_#22c55e] animate-pulse" : "bg-orange-500")} />
            <span>ENTROPY_BUFFER: {entropyStats.poolSize}b</span>
          </div>
        </div>
      </header>

      <main className="flex-1 flex flex-col items-center justify-center p-6 relative z-10">
        <div className="w-full max-w-2xl flex flex-col items-center space-y-12">
          
          <div className="h-64 flex items-center justify-center w-full">
            <Planchette 
              isResonating={isResonating} 
              confidence={latestResponse?.probability_confidence || 0}
              resonance={latestResponse?.temporal_resonance || 0}
              seal={latestResponse?.ritual_seal || ''}
            />
          </div>

          <div className="w-full space-y-10">
            <div className="min-h-[120px] flex px-4">
              <AnimatePresence mode="wait">
                {latestResponse && !isResonating && (
                  <motion.div 
                    key={latestResponse.ritual_seal}
                    initial={{ opacity: 0, filter: 'blur(10px)', y: 10 }}
                    animate={{ opacity: 1, filter: 'blur(0px)', y: 0 }}
                    exit={{ opacity: 0, filter: 'blur(10px)', y: -10 }}
                    transition={{ duration: 0.6 }}
                    className="w-full text-center space-y-4"
                  >
                    <p className="text-xl md:text-2xl font-light tracking-widest text-[#f5f5f5] uppercase drop-shadow-[0_0_10px_rgba(255,255,255,0.2)]">
                      &quot;{latestResponse.message}&quot;
                    </p>
                    <div className="flex items-center justify-center gap-4 text-[10px] tracking-[0.2em] text-white/40 uppercase">
                      <span className="flex items-center gap-1.5">
                        <Activity className="h-3 w-3 text-cyan-500" />
                        Conf: {(latestResponse.probability_confidence * 100).toFixed(1)}%
                      </span>
                      <span>•</span>
                      <span className="flex items-center gap-1.5">
                        <Zap className="h-3 w-3 text-purple-500" />
                        Res: {latestResponse.temporal_resonance.toFixed(3)} Hz
                      </span>
                    </div>
                  </motion.div>
                )}
                {isResonating && (
                   <motion.div 
                    initial={{ opacity: 0 }}
                    animate={{ opacity: 1 }}
                    exit={{ opacity: 0 }}
                    className="w-full text-center text-purple-400 tracking-[0.3em] text-sm flex flex-col items-center justify-center h-full animate-pulse"
                  >
                    TRANSMUTING ENTROPY...
                  </motion.div>
                )}
                {!latestResponse && !isResonating && (
                  <motion.div 
                    initial={{ opacity: 0 }}
                    animate={{ opacity: 1 }}
                    className="w-full text-center text-white/20 tracking-[0.3em] text-xs flex flex-col items-center justify-center h-full"
                  >
                    POUR YOUR INTENT INTO THE TERMINAL
                  </motion.div>
                )}
              </AnimatePresence>
            </div>

            <form onSubmit={handleAsk} className="relative w-full group">
              <input 
                type="text"
                value={question}
                onChange={e => setQuestion(e.target.value)}
                autoComplete="off"
                placeholder="Ask the Aether..."
                disabled={isResonating}
                className="w-full bg-white/[0.02] border border-white/10 p-5 text-center text-lg placeholder:text-white/20 focus:outline-none focus:border-purple-500/50 focus:bg-white/[0.04] transition-all rounded-sm disabled:opacity-50 text-white shadow-[inset_0_0_20px_rgba(0,0,0,0.5)]"
              />
              <div className="absolute left-4 top-1/2 -translate-y-1/2 text-white/20 group-focus-within:text-purple-500 transition-colors pointer-events-none">
                 <Zap className="h-5 w-5" />
              </div>
              
              {/* Scanline effect on input */}
              <div className="absolute inset-0 pointer-events-none rounded-sm overflow-hidden opacity-0 group-focus-within:opacity-100 transition-opacity">
                <div className="w-full h-[1px] bg-purple-500/30 absolute top-0 -translate-y-full animate-[scan_2s_linear_infinite]" />
              </div>
            </form>
          </div>
        </div>
      </main>

      <footer className="h-10 shrink-0 flex items-center justify-between px-6 border-t border-white/10 text-[9px] text-white/30 tracking-[0.2em] uppercase relative z-10 bg-black/40">
        <span className="font-bold flex items-center gap-2">
          <span className="h-1 w-1 bg-white/50 rounded-full" />
          X:{entropyStats.lastX.toString().padStart(4, '0')} Y:{entropyStats.lastY.toString().padStart(4, '0')}
        </span>
        <span>MPL_SDK_V_0.1 // NON-LINEAR_PROBABILISTIC_RUNTIME</span>
      </footer>
    </div>
  );
}
