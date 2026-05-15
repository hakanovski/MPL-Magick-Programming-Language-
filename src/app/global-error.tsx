'use client'
 
export default function GlobalError({
  error,
  reset,
}: {
  error: Error & { digest?: string }
  reset: () => void
}) {
  return (
    <html>
      <body>
        <div className="flex h-screen w-full flex-col items-center justify-center bg-[#050505] text-[#e0e0e0] font-mono">
          <h2 className="text-xl font-bold text-red-500 mb-4">AETHER ANOMALY DETECTED</h2>
          <button onClick={() => reset()} className="px-6 py-2 border border-red-500 bg-red-900/20 text-red-300">
            Try again
          </button>
        </div>
      </body>
    </html>
  )
}
