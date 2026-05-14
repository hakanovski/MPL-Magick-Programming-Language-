import { NextResponse } from 'next/server';

export const dynamic = 'force-dynamic';

export async function POST(req: Request) {
  try {
    const payload = await req.json();
    const headers: Record<string, string> = {
      'Content-Type': 'application/json',
      'X-MPL-SIGIL': '369-TESLA-RESONANCE',
    };
    
    if (req.headers.get('x-mpl-shadow')) {
      headers['X-MPL-SHADOW'] = 'true';
    }

    const controller = new AbortController();
    const timeoutId = setTimeout(() => controller.abort(), 2000);

    const res = await fetch('http://127.0.0.1:3690/simulate_intent', {
      method: 'POST',
      headers,
      body: JSON.stringify(payload),
      signal: controller.signal,
    });

    clearTimeout(timeoutId);

    if (res.ok) {
        const data = await res.json();
        return NextResponse.json(data);
    }
    return NextResponse.json({ error: "Failed to simulate" }, { status: 500 });
  } catch (err) {
    return NextResponse.json({
        status: "Simulated (Mock - Gateway down)",
        resonance_score: 1.618,
        temporal_resonance: 432.0,
        probability_confidence: 0.99,
        neural_signature: "mock_signature_abc123",
        visual_sigil: null,
        is_shadow_mode: true,
        message: "Simulation active (Mock).",
    });
  }
}
