'use client';

import React, { useState, useEffect } from 'react';
import { Activity, Cpu, Hexagon, Terminal, Zap, Code, Shield, Eye, Disc } from 'lucide-react';
import { AreaChart, Area, XAxis, YAxis, CartesianGrid, Tooltip, ResponsiveContainer } from 'recharts';

// Simulated Harmonic Flow Data (3-6-9 Frequencies)
const generateHarmonicData = () => {
  const data = [];
  for (let i = 0; i < 60; i++) {
    data.push({
      time: i,
      freq3: Math.abs(Math.sin(i * 0.3)) * 30 + 10,
      freq6: Math.abs(Math.cos(i * 0.6)) * 60 + 20,
      freq9: Math.abs(Math.sin(i * 0.9)) * 90 + 30,
    });
  }
  return data;
};

export default function MPLDashboard() {
  const [script, setScript] = useState(
`// MagickScript (.ms)
// Target: Apple M4 Max Unified Memory

sacrifice btc_stop = 369.0
sacrifice directive = "PREVENT LOSS"

// Invoke harmonic grid resonance
invoke synchronize_mlx()

transmute directive
`
  );

  const [logs, setLogs] = useState<string[]>([]);
  const [isRunning, setIsRunning] = useState(false);
  const [chartData, setChartData] = useState(generateHarmonicData());

  useEffect(() => {
    const interval = setInterval(() => {
      if (isRunning) {
        setChartData((prev) => {
          const newData = [...prev.slice(1)];
          const i = prev[prev.length - 1].time + 1;
          newData.push({
            time: i,
            freq3: Math.abs(Math.sin(i * 0.3)) * 30 + 10 + Math.random() * 10,
            freq6: Math.abs(Math.cos(i * 0.6)) * 60 + 20 + Math.random() * 20,
            freq9: Math.abs(Math.sin(i * 0.9)) * 90 + 30 + Math.random() * 30,
          });
          return newData;
        });
      }
    }, 100);
    return () => clearInterval(interval);
  }, [isRunning]);

  const runRitual = () => {
    setIsRunning(true);
    setLogs([]);
    
    const steps = [
      "SYSTEM: Connecting to Native Apple MLX bindings...",
      "SYSTEM: Initializing Occult Virtual Machine (OVM)...",
      "OVM: Loading AST parameters...",
      "OVM: [Parsed] sacrifice btc_stop = 369.0",
      "OVM: [Parsed] sacrifice directive = PREVENT LOSS",
      "METAL: Allocating Unified Memory (Buffer: 8.4 GB)",
      "MLX: Calibrating Tensor Gates...",
      "COMPILER: Tuning execution matrix to 432Hz baseline",
      ">> TRANSMUTATION COMPLETE -> TARGET: ETHER",
      ">> ANOMALY MATRIX ALIGNED",
      "SYSTEM: Ritual anchored. Probabilistic streams locked."
    ];

    steps.forEach((step, index) => {
      setTimeout(() => {
        setLogs(prev => [...prev, step]);
        if (index === steps.length - 1) {
          setTimeout(() => setIsRunning(false), 2000);
        }
      }, index * 400); // 400ms delay per step
    });
  };

  return (
    <div className="flex h-screen bg-[#050505] text-[#e0e0e0] font-mono overflow-hidden selection:bg-[#00f2ff] selection:text-black">
      <div className="scanline absolute inset-0 z-50 pointer-events-none"></div>
      
      {/* Sidebar Navigation */}
      <aside className="w-16 border-r border-[#1a1a1a] bg-[#050505] relative z-40 flex flex-col items-center py-4 gap-8">
        <Hexagon className="w-8 h-8 text-[#bc13fe] animate-pulse" />
        <nav className="flex flex-col gap-6">
          <button className="p-3 bg-[#00f2ff]/10 rounded-lg text-[#00f2ff] hover:text-white hover:bg-[#00f2ff]/20 transition-colors border border-[#00f2ff]/30" title="IDE">
            <Code className="w-5 h-5" />
          </button>
          <button className="p-3 text-zinc-500 hover:text-white transition-colors" title="Scrying Engine">
            <Eye className="w-5 h-5" />
          </button>
          <button className="p-3 text-zinc-500 hover:text-white transition-colors" title="Warding (Cyber Defense)">
            <Shield className="w-5 h-5" />
          </button>
          <button className="p-3 text-zinc-500 hover:text-white transition-colors" title="Hardware Matrix">
            <Cpu className="w-5 h-5" />
          </button>
        </nav>
      </aside>

      {/* Main Content Area */}
      <main className="flex-1 flex flex-col">
        {/* Header */}
        <header className="h-14 border-b border-[#1a1a1a] flex items-center justify-between px-6 bg-[#050505]/80 backdrop-blur-sm relative z-40">
          <div className="flex items-center gap-3">
            <h1 className="text-sm font-bold tracking-widest glow-text">PROJECT <span className="text-[#00f2ff]">ZERO</span> <span className="opacity-30">/</span> MPL</h1>
          </div>
          <div className="flex items-center gap-4 text-[10px] uppercase tracking-[0.2em] text-[#888]">
            <div className="flex items-center gap-2">
              <div className="w-2 h-2 rounded-full bg-[#00f2ff] animate-pulse" />
              APPLE M4 MAX UNIFIED MEMORY: 128GB
            </div>
            <span>|</span>
            <span>RUST KERNEL: V0.1.0</span>
          </div>
        </header>

        {/* Dashboard Grid */}
        <div className="flex-1 grid grid-cols-12 gap-6 p-6 geometric-grid relative z-40">
          
          {/* Left Column: Editor */}
          <section className="col-span-12 lg:col-span-7 border-arcane flex flex-col rounded-xl overflow-hidden relative backdrop-blur-sm bg-black/40">
            <div className="h-10 border-b border-[#222] bg-[#050505]/80 flex items-center justify-between px-4">
              <div className="flex items-center gap-2 text-[11px] uppercase tracking-widest text-[#00f2ff]">
                <Terminal className="w-4 h-4" />
                <span>MAGICK_SCRIPT_IDE</span>
                <span className="bg-[#111] px-2 py-0.5 rounded text-[#00f2ff]/50 text-[10px]">.ms</span>
              </div>
              <button 
                onClick={runRitual} 
                disabled={isRunning}
                className="flex items-center gap-2 border border-[#00f2ff]/50 bg-[#00f2ff]/10 text-[#00f2ff] hover:bg-[#00f2ff]/20 px-4 py-1 text-[10px] font-bold uppercase tracking-widest transition-all disabled:opacity-50 disabled:cursor-not-allowed"
              >
                <Zap className="w-3 h-3" />
                {isRunning ? 'Synthesizing...' : 'Run Ritual'}
              </button>
            </div>
            
            <div className="flex-1 relative">
              {/* Line numbers mock overlay */}
              <div className="absolute left-0 top-0 bottom-0 w-12 bg-[#050505]/80 border-r border-[#222] flex flex-col items-center py-4 text-[#555] select-none text-[10px]">
                {script.split('\n').map((_, i) => <span key={i} className="py-0.5">{i + 1}</span>)}
              </div>
              <textarea
                value={script}
                onChange={(e) => setScript(e.target.value)}
                spellCheck="false"
                suppressHydrationWarning
                className="w-full h-full bg-transparent text-[#e0e0e0] p-4 pl-16 resize-none focus:outline-none focus:ring-0 font-mono text-sm leading-6 relative z-10"
              />
            </div>
          </section>

          {/* Right Column: Visualization & OVM Output */}
          <section className="col-span-12 lg:col-span-5 flex flex-col gap-6">
            
            {/* Top Right: GPU / Resonance Monitor */}
            <div className="flex-1 border-arcane p-4 flex flex-col rounded-xl relative backdrop-blur-sm bg-black/40">
              <div className="flex items-center gap-2 text-[11px] uppercase tracking-widest text-[#bc13fe] mb-3">
                <Activity className="w-4 h-4" />
                <span>HARMONIC RESONANCE (3-6-9)</span>
              </div>
              
              <div className="flex-1 w-full min-h-[200px]">
                <ResponsiveContainer width="100%" height="100%">
                  <AreaChart data={chartData} margin={{ top: 0, right: 0, left: -20, bottom: 0 }}>
                    <defs>
                      <linearGradient id="color3" x1="0" y1="0" x2="0" y2="1">
                        <stop offset="5%" stopColor="#06b6d4" stopOpacity={0.8}/>
                        <stop offset="95%" stopColor="#06b6d4" stopOpacity={0}/>
                      </linearGradient>
                      <linearGradient id="color6" x1="0" y1="0" x2="0" y2="1">
                        <stop offset="5%" stopColor="#a855f7" stopOpacity={0.8}/>
                        <stop offset="95%" stopColor="#a855f7" stopOpacity={0}/>
                      </linearGradient>
                    </defs>
                    <CartesianGrid strokeDasharray="3 3" stroke="#222" vertical={false} />
                    <XAxis dataKey="time" hide />
                    <YAxis stroke="#555" fontSize={10} tickFormatter={(v) => `${v}Hz`} />
                    <Tooltip 
                      contentStyle={{ backgroundColor: '#050505', border: '1px solid #bc13fe', fontFamily: 'monospace', fontSize: '10px' }}
                      itemStyle={{ color: '#00f2ff' }}
                    />
                    <Area type="monotone" dataKey="freq3" stroke="#00f2ff" fillOpacity={1} fill="url(#color3)" />
                    <Area type="monotone" dataKey="freq6" stroke="#bc13fe" fillOpacity={1} fill="url(#color6)" />
                    <Area type="monotone" dataKey="freq9" stroke="#d4af37" fillOpacity={0.1} />
                  </AreaChart>
                </ResponsiveContainer>
              </div>
            </div>

            {/* Bottom Right: OVM Terminal output */}
            <div className="flex-1 border-arcane p-4 flex flex-col font-mono rounded-xl relative backdrop-blur-sm bg-black/40">
              <div className="flex items-center justify-between mb-3 text-[11px] uppercase tracking-widest text-[#d4af37]">
                 <div>OVM_COMPILER_LOG</div>
                 {isRunning && <span className="text-[9px] text-[#00f2ff] animate-pulse uppercase">Ingesting...</span>}
              </div>
              
              <div className="flex-1 overflow-y-auto space-y-1 bg-black/80 p-3 border border-dashed border-[#444] text-[9px] text-blue-300">
                {logs.length === 0 && !isRunning && (
                  <div className="opacity-50 italic">[INFO] Awaiting sacrifice vector...</div>
                )}
                {logs.map((log, index) => (
                  <div key={index} className={`${log.startsWith('>>') ? 'text-[#00f2ff] font-bold' : log.startsWith('OVM: [Parsed]') ? 'text-[#bc13fe]' : log.startsWith('SYSTEM:') ? 'text-[#d4af37]' : ''}`}>
                    {log}
                  </div>
                ))}
              </div>
            </div>
            
          </section>
        </div>
      </main>
    </div>
  );
}
