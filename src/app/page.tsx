import { Activity, Cpu, Hexagon, Network, Zap } from 'lucide-react';
import Link from 'next/link';
import { MplServitorChat } from '@/components/MplServitorChat';

export default function Dashboard() {
  return (
    <div className="flex h-screen flex-col overflow-hidden bg-[#050505] text-[#e0e0e0] selection:bg-purple-900/50">
      <header className="flex h-14 items-center justify-between border-b border-white/10 px-6 backdrop-blur-md">
        <div className="flex items-center gap-3">
          <Hexagon className="h-5 w-5 text-purple-500" />
          <span className="font-mono text-sm font-bold tracking-widest text-[#e0e0e0]">
            MPL / TECHNOMANCY SYSTEM
          </span>
        </div>
        <div className="flex items-center gap-4 text-xs font-mono text-white/50">
          <div className="flex items-center gap-1.5">
            <div className="h-2 w-2 rounded-full bg-green-500 shadow-[0_0_8px_rgba(34,197,94,0.6)] animate-pulse" />
            <span>OVM ONLINE</span>
          </div>
          <span>[TENSOR SYNC: ARMED]</span>
          <span>APPLE M4 MAX / METAL ACCEL</span>
        </div>
      </header>

      <main className="flex-1 overflow-auto p-6 font-mono">
        <div className="mx-auto max-w-7xl space-y-6 h-full flex flex-col">
          <div className="grid grid-cols-1 gap-6 md:grid-cols-3">
            <div className="col-span-1 border border-white/10 bg-white/5 p-4 rounded-md">
              <h2 className="mb-4 text-xs font-bold uppercase tracking-widest text-purple-400">
                Resonance Core
              </h2>
              <div className="space-y-4">
                <div className="flex items-center justify-between">
                  <span className="text-white/60 text-sm">Harmonic Output</span>
                  <span className="text-sm font-bold">1.61803 Ψ</span>
                </div>
                <div className="flex items-center justify-between">
                  <span className="text-white/60 text-sm">Enochian Threads</span>
                  <span className="text-sm font-bold">72 / 72</span>
                </div>
              </div>
            </div>

            <div className="col-span-2 border border-white/10 bg-white/5 p-4 rounded-md overflow-hidden relative">
               <h2 className="mb-4 text-xs font-bold uppercase tracking-widest text-blue-400">
                Compiler Status (Rust/LLVM) :: Sentinel Feed
              </h2>
              <div className="flex flex-col gap-2 relative z-10 text-sm h-32 overflow-y-auto">
                 <div className="text-green-400">&gt; Initialize OVM kernel... OK</div>
                 <div className="text-green-400">&gt; Linking Metal Shaders (MPS)... OK</div>
                 <div className="text-white/80">&gt; Allocating Tensor buffers (MLX)...</div>
                 <div className="text-blue-400 opacity-80">&gt; Akashic Registry: Connected</div>
                 <div className="text-purple-400 opacity-80">&gt; Awaiting probabilistic intent matrix...</div>
              </div>
            </div>
          </div>

          <div className="grid grid-cols-1 lg:grid-cols-2 gap-6 flex-1 min-h-[400px]">
             {/* MPL Servitor Chat Area */}
             <div className="flex flex-col h-full rounded-md overflow-hidden">
               <MplServitorChat />
             </div>

             {/* Scrying / Ouija Redirect */}
             <div className="border border-white/10 bg-white/5 p-4 rounded-md h-full flex flex-col">
               <h2 className="mb-4 text-xs font-bold uppercase tracking-widest text-white/50">
                  Scrying Interface / Non-Linear Dashboard
                </h2>
                <div className="flex-1 w-full flex flex-col items-center justify-center border-dashed border border-white/20 relative group overflow-hidden">
                  <div className="absolute inset-0 bg-purple-500/5 opacity-0 group-hover:opacity-100 transition-opacity" />
                  <p className="text-white/30 text-sm tracking-widest text-center mb-6 z-10">
                    AWAITING OUIJA ALIGNMENT<br/>[ NO RITUAL DATA RECEIVED ]
                  </p>
                  <Link href="/ouija" className="z-10 px-6 py-2 border border-purple-500/50 bg-purple-900/20 text-purple-300 text-xs font-bold tracking-widest hover:bg-purple-500 hover:text-white transition-all uppercase rounded-sm flex items-center gap-2 shadow-[0_0_15px_rgba(168,85,247,0.2)]">
                    <Zap className="h-4 w-4" />
                    Enter Akashic Ouija
                  </Link>
                </div>
             </div>
          </div>
        </div>
      </main>
    </div>
  );
}
