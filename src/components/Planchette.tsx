'use client';

import React from 'react';
import { motion } from 'motion/react';
import { Hexagon } from 'lucide-react';
import { clsx } from 'clsx';
import { twMerge } from 'tailwind-merge';

export function cn(...inputs: (string | undefined | null | false)[]) {
  return twMerge(clsx(inputs));
}

interface PlanchetteProps {
  isResonating: boolean;
  confidence: number;
  resonance: number;
  seal: string;
}

export function Planchette({ isResonating, confidence, resonance, seal }: PlanchetteProps) {
  // Translate confidence/resonance into animation values
  // High resonance causes erratic vibration, confidence determines glow intensity
  
  const intensity = isResonating ? 15 : (resonance > 450 ? 5 : 0);
  const glow = confidence > 0.9 ? 'drop-shadow-[0_0_20px_rgba(168,85,247,0.8)]' : 
               confidence > 0.8 ? 'drop-shadow-[0_0_12px_rgba(59,130,246,0.6)]' :
               isResonating ? 'drop-shadow-[0_0_10px_rgba(239,68,68,0.8)]' : 'drop-shadow-[0_0_2px_rgba(255,255,255,0.2)]';

  return (
    <div className="flex flex-col items-center justify-center space-y-8">
      <motion.div
        animate={
          isResonating 
            ? { x: [0, -intensity, intensity, -intensity/2, intensity/2, 0], y: [0, intensity/2, -intensity, intensity, -intensity/2, 0] }
            : { x: 0, y: 0 }
        }
        transition={{
          repeat: isResonating ? Infinity : 0,
          duration: 0.15,
          ease: "linear"
        }}
        className={cn("relative flex items-center justify-center p-8 transition-all duration-700", glow)}
      >
        <Hexagon className={cn("h-32 w-32 stroke-[0.5] transition-colors duration-700", isResonating ? "text-purple-400" : "text-white/20")} />
        
        {/* Core Resonance Indicator */}
        <div className="absolute inset-0 flex items-center justify-center">
          <div className={cn(
            "h-4 w-4 rounded-full border transition-all duration-700",
            isResonating ? "bg-white border-purple-300 animate-pulse scale-150 shadow-[0_0_15px_#fff]" : "bg-white/10 border-white/20"
          )} />
        </div>
        
        {/* Reticle / Decorative elements */}
        <div className="absolute top-0 w-[1px] h-6 bg-purple-500/40" />
        <div className="absolute bottom-0 w-[1px] h-6 bg-purple-500/40" />
        <div className="absolute left-0 h-[1px] w-6 bg-purple-500/40" />
        <div className="absolute right-0 h-[1px] w-6 bg-purple-500/40" />
      </motion.div>

      {/* Immutable Hash Output */}
      {seal && (
        <motion.div 
          initial={{ opacity: 0, y: 10 }}
          animate={{ opacity: 1, y: 0 }}
          className="font-mono text-center flex flex-col items-center"
        >
          <div className="text-white/30 mb-1 tracking-[0.3em] text-[8px] uppercase">Ritual Seal [Hash]</div>
          <div className="text-purple-300 font-bold bg-purple-900/20 px-4 py-1.5 rounded-sm border border-purple-500/20 text-xs tracking-wider shadow-[inset_0_0_10px_rgba(168,85,247,0.1)]">
            {seal}
          </div>
        </motion.div>
      )}
    </div>
  );
}
