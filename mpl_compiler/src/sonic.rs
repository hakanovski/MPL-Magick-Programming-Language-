//! The Resonance Audio Engine
//! Translates resonance data into somatic audio frequencies based on 3-6-9 harmonics.

use std::f32::consts::PI;

pub struct SonicFrequency {
    pub base_hz: f64,
}

impl SonicFrequency {
    pub fn new(base_hz: f64) -> Self {
        Self { base_hz }
    }

    /// Generates audio sine wave parameters mapped to Solfeggio / 3-6-9 harmonics.
    /// Returns a normalized float vector for the frontend WebAudio API.
    pub fn generate_resonance_tone(&self, score: f64) -> Vec<f32> {
        let sample_rate = 44100.0;
        let duration = 0.5; // seconds
        let num_samples = (sample_rate * duration) as usize;
        
        let mut audio_data = Vec::with_capacity(num_samples);
        
        // Modulate the primary wave with 3-6-9 harmonics
        let freq_3 = self.base_hz * (score / 3.0 % 1.0 + 1.0);
        let freq_6 = self.base_hz * (score / 6.0 % 1.0 + 1.0);
        let freq_9 = self.base_hz * (score / 9.0 % 1.0 + 1.0);

        for i in 0..num_samples {
            let t = i as f32 / sample_rate as f32;
            let sample_3 = (t * freq_3 as f32 * 2.0 * PI).sin();
            let sample_6 = (t * freq_6 as f32 * 2.0 * PI).sin() * 0.5;
            let sample_9 = (t * freq_9 as f32 * 2.0 * PI).sin() * 0.33;
            
            // Normalize sum to prevent clipping
            let combined = (sample_3 + sample_6 + sample_9) / 1.83;
            audio_data.push(combined);
        }

        println!("[SONIC_ENGINE] Generated {} audio samples at base harmonic {:.1}Hz modulated by resonance {:.1}", num_samples, self.base_hz, score);
        
        audio_data
    }
}
