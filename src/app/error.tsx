'use client';
export default function ErrorPage({
  error,
  reset,
}: {
  error: Error & { digest?: string }
  reset: () => void
}) {
  return (
    <div className="flex h-full w-full flex-col items-center justify-center bg-[#050505] text-[#e0e0e0] font-mono">
      <h2 className="text-xl font-bold text-red-500 mb-4">AETHER ERROR DETECTED</h2>
      <p className="text-white/50 mb-6 text-sm">{error.message || 'The Akashic record rejected the request.'}</p>
      <button onClick={() => reset()} className="px-6 py-2 border border-red-500 bg-red-900/20 text-red-300 hover:bg-red-900/50 transition-colors">
        RE-INITIATE SEQUENCE
      </button>
    </div>
  );
}
