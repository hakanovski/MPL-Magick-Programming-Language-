'use client';

import React, { useState, useEffect } from 'react';
import { Activity, Cpu, Hexagon, Terminal, Zap, Code, Shield, Eye, Disc } from 'lucide-react';
import dynamic from 'next/dynamic';

const ChartComponent = dynamic(() => import('../components/ChartComponent'), { ssr: false });

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
  const [sentinelLogs, setSentinelLogs] = useState<any[]>([]);
  const [isRunning, setIsRunning] = useState(false);
  const [isMirrorMode, setIsMirrorMode] = useState(false);
  const [chartData, setChartData] = useState(generateHarmonicData());

  const accentColor = isMirrorMode ? '#00f2ff' : '#bc13fe';
  const accentText = isMirrorMode ? 'text-[#00f2ff]' : 'text-[#bc13fe]';
  const accentBorder = isMirrorMode ? 'border-[#00f2ff]' : 'border-[#bc13fe]';
  const accentBg = isMirrorMode ? 'bg-[#00f2ff]' : 'bg-[#bc13fe]';

  useEffect(() => {
    const fetchSentinelLogs = async () => {
      try {
        // Fallback mock if gateway isn't reachable (for initial SSR styling)
        const res = await fetch('/api/sentinel_logs');
        if (res.ok) {
          const data = await res.json();
          setSentinelLogs(data);
        }
      } catch (e) {
        // Ignore CORS/Network errors in preview sandbox until proxy is up
      }
    };
    
    fetchSentinelLogs();
    const interval = setInterval(fetchSentinelLogs, 3000);
    return () => clearInterval(interval);
  }, []);

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

  const runRitual = async () => {
    setIsRunning(true);
    setLogs([]);
    
    // Simulate real logs over time
    const initialSteps = [
      "SYSTEM: Connecting to Native Apple MLX bindings...",
      "SYSTEM: Initializing Occult Virtual Machine (OVM)...",
      "OVM: Loading AST parameters...",
      `METAL: Allocating ${isMirrorMode ? 'Shadow' : 'Unified'} Memory (Buffer: 8.4 GB)`,
      "MLX: Calibrating Tensor Gates...",
      "COMPILER: Tuning execution matrix to 432Hz baseline",
    ];

    for (let i = 0; i < initialSteps.length; i++) {
        await new Promise(r => setTimeout(r, 400));
        setLogs(prev => [...prev, initialSteps[i]]);
    }

    try {
      if (isMirrorMode) {
          const res = await fetch('/api/simulate_intent', {
              method: 'POST',
              headers: {
                  'Content-Type': 'application/json',
                  'X-MPL-SHADOW': 'true'
              },
              body: JSON.stringify({ intent_text: "Execute from Mirror Sandbox" })
          });
          const data = await res.json();
          setLogs(prev => [...prev, `>> SHADOW SIMULATION COMPLETE`]);
          setLogs(prev => [...prev, `>> PROBABILITY CONFIDENCE: ${(data.probability_confidence * 100).toFixed(2)}%`]);
          setLogs(prev => [...prev, `SYSTEM: Sandbox simulation finished.`]);
      } else {
          setLogs(prev => [...prev, ">> TRANSMUTATION COMPLETE -> TARGET: ETHER"]);
          setLogs(prev => [...prev, ">> ANOMALY MATRIX ALIGNED"]);
          setLogs(prev => [...prev, "SYSTEM: Ritual anchored. Probabilistic streams locked."]);
      }
    } catch (e) {
      setLogs(prev => [...prev, "SYSTEM_FAULT: Temporal gateway offline."]);
    }
    
    setTimeout(() => setIsRunning(false), 1000);
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
            <button 
              onClick={() => setIsMirrorMode(!isMirrorMode)}
              className={`px-3 py-1 border rounded-md uppercase tracking-widest text-[9px] transition-all flex border-opacity-50 ${isMirrorMode ? 'border-[#00f2ff] text-[#00f2ff] bg-[#00f2ff]/10' : 'border-[#bc13fe] text-[#bc13fe] bg-[#bc13fe]/10'}`}
            >
              Mirror Sandbox {isMirrorMode ? "ON" : "OFF"}
            </button>
            <span>|</span>
            <div className="flex items-center gap-2">
              <div className={`w-2 h-2 rounded-full ${accentBg} animate-pulse`} />
              APPLE M4 MAX
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
              <div className={`flex items-center gap-2 text-[11px] uppercase tracking-widest ${accentText}`}>
                <Terminal className="w-4 h-4" />
                <span>MAGICK_SCRIPT_IDE</span>
                <span className="bg-[#111] px-2 py-0.5 rounded opacity-50 text-[10px]">.ms</span>
              </div>
              <button 
                onClick={runRitual} 
                disabled={isRunning}
                className={`flex items-center gap-2 border ${accentBorder} border-opacity-50 ${accentBg}/10 ${accentText} hover:bg-opacity-20 px-4 py-1 text-[10px] font-bold uppercase tracking-widest transition-all disabled:opacity-50 disabled:cursor-not-allowed`}
              >
                <Zap className="w-3 h-3" />
                {isRunning ? 'Synthesizing...' : (isMirrorMode ? 'Simulate' : 'Run Ritual')}
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
                <ChartComponent chartData={chartData} />
              </div>
            </div>

            {/* Bottom Right: OVM Terminal output */}
            <div className={`flex-1 border-arcane p-4 flex flex-col font-mono rounded-xl relative backdrop-blur-sm bg-black/40 border-t ${accentBorder} border-opacity-30`}>
              <div className={`flex items-center justify-between mb-3 text-[11px] uppercase tracking-widest ${accentText}`}>
                 <div>OVM_COMPILER_LOG</div>
                 {isRunning && <span className={`text-[9px] ${accentText} animate-pulse uppercase`}>Ingesting...</span>}
              </div>
              
              <div className="flex-1 overflow-y-auto space-y-1 bg-black/80 p-3 border border-dashed border-[#444] text-[9px] text-blue-300">
                {logs.length === 0 && !isRunning && (
                  <div className="opacity-50 italic">[INFO] Awaiting sacrifice vector...</div>
                )}
                {logs.map((log, index) => (
                  <div key={index} className={`${log.startsWith('>>') ? `${accentText} font-bold` : log.startsWith('OVM: [Parsed]') ? 'text-[#bc13fe]' : log.startsWith('SYSTEM:') ? 'text-[#d4af37]' : ''}`}>
                    {log}
                  </div>
                ))}
              </div>
            </div>

            {/* Sovereign Sentinel Feed */}
            <div className="flex-[0.8] border-arcane p-4 flex flex-col font-mono rounded-xl relative backdrop-blur-sm bg-black/40 border-[#bc13fe]/30">
              <div className="flex items-center justify-between mb-3 text-[11px] uppercase tracking-widest text-[#bc13fe]">
                 <div className="flex items-center gap-2">
                   <Shield className="w-4 h-4" />
                   <span>SOVEREIGN_SENTINEL_FEED</span>
                 </div>
                 <span className="text-[9px] text-[#bc13fe] animate-pulse uppercase">Monitoring</span>
              </div>
              
              <div className="flex-1 overflow-y-auto space-y-2 text-[9px]">
                {sentinelLogs.length === 0 ? (
                  <div className="opacity-50 italic text-[#bc13fe]">[AWAITING GOLDEN RESONANCE...]</div>
                ) : (
                  sentinelLogs.map((slog, i) => (
                    <div key={i} className="p-2 border border-[#bc13fe]/20 bg-[#bc13fe]/5 rounded">
                      <div className="flex justify-between items-center text-[#bc13fe] mb-1 opacity-70">
                        <span>RES: {slog.resonance_score.toFixed(3)}</span>
                        <span>{new Date(slog.timestamp * 1000).toLocaleTimeString()}</span>
                      </div>
                      <div className="text-white mb-1">&gt; {slog.intent}</div>
                      <div className="text-[#00f2ff] opacity-60">SEAL_{slog.seal_id?.slice(0, 16)}...</div>
                    </div>
                  ))
                )}
              </div>
            </div>
            
          </section>
        </div>
      </main>
    </div>
  );
}
