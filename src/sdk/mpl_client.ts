export interface AkashicResponse {
  message: string;
  probability_confidence: number;
  temporal_resonance: number;
  ritual_seal: string;
  entropy_consumed: number;
  [key: string]: any;
}

export class MplClient {
  private static readonly BASE_URL = 'http://localhost:3690'; // MPL Daemon default port

  /**
   * Dispatches human entropy and semantic queries to the OVM for non-linear resolution.
   * @param question The user's intent.
   * @param client_entropy Array of X/Y coordinate shifts.
   * @returns AkashicResponse containing the resonated outcome.
   */
  public async ask_akashic(question: string, client_entropy: number[]): Promise<AkashicResponse> {
    try {
      const response = await fetch(`${MplClient.BASE_URL}/api/v1/akashic/query`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          'X-MPL-Source': 'Genesis-Ouija-Client',
        },
        body: JSON.stringify({
          intent: question,
          entropy_payload: client_entropy,
          timestamp: Date.now(),
        }),
      });

      if (!response.ok) {
        throw new Error(`OVM Resonance Failure: HTTP ${response.status}`);
      }

      const data = await response.json();
      return data as AkashicResponse;
    } catch (error) {
      console.error('MPL Daemon connection lost or failed to resonate.', error);
      // Fallback response for demonstration/development if daemon is offline
      // Simulating a deterministic pseudo-random response based on entropy
      const entropySum = client_entropy.reduce((a, b) => a + b, 0);
      const isPositive = entropySum % 2 === 0;
      
      return {
        message: isPositive ? "THE AETHER ACKNOWLEDGES YOUR INTENT." : "DISTURBANCES DETECTED. RECALIBRATE ENTROPY.",
        probability_confidence: 0.82 + (Math.random() * 0.15),
        temporal_resonance: 432.0 + (Math.random() * 50),
        ritual_seal: 'ERR_0x' + Math.random().toString(16).slice(2, 10).toUpperCase(),
        entropy_consumed: client_entropy.length
      };
    }
  }
}

export const activeMplClient = new MplClient();
