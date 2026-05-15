import Link from 'next/link'
 
export default function NotFound() {
  return (
    <div className="flex h-screen w-full flex-col items-center justify-center bg-[#050505] text-[#e0e0e0] font-mono">
      <h2 className="text-xl font-bold mb-2">Not Found</h2>
      <p className="text-sm text-white/50 mb-4">Could not find requested resource</p>
      <Link href="/" className="text-purple-400 hover:text-purple-300">Return Home</Link>
    </div>
  )
}
