import { NextResponse } from 'next/server';

export const dynamic = 'force-dynamic';

export async function GET() {
  try {
    // Attempt to connect to the local Rust OVM 
    // Uses a short timeout so preview environments don't hang if it's missing
    const controller = new AbortController();
    const timeoutId = setTimeout(() => controller.abort(), 1000);
    
    const res = await fetch('http://127.0.0.1:3690/sentinel_logs', {
      signal: controller.signal,
    });
    
    clearTimeout(timeoutId);

    if (res.ok) {
      const data = await res.json();
      return NextResponse.json(data);
    }
  } catch (err) {
    // Rust Gateway not running (e.g. in web-only preview container)
    // Fall back to returning mock resonant data
  }

  // Graceful degradation for the web preview
  return NextResponse.json([
    {
      timestamp: Math.floor(Date.now() / 1000) - 12,
      resonance_score: 1.618,
      intent: "Harmonize fractal structures",
      seal_id: "seal_99f3b14d2e8aa192736",
    },
    {
      timestamp: Math.floor(Date.now() / 1000) - 369,
      resonance_score: 1.555,
      intent: "Stabilise chronos variance",
      seal_id: "seal_3a4b5c6d7e8f9g0h1i2",
    }
  ]);
}
