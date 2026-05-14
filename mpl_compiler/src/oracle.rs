//! The Oracle (Telemetry & Event Loop)
//! 
//! The ingestion daemon for chaotic external data (e.g., market quotes, environmental noise).
//! Filters the noise of the universe through the strict mathematical lens
//! of the Harmonic Resonance engine, detecting actionable thermodynamic ripples.

use crate::harmonic::extract_tesla_harmonics;

/// The Oracle acts as the sentinel between the raw chaotic data stream
/// and the deterministic, esoteric Virtual Machine.
pub trait Oracle {
    /// Evaluates a raw, unfiltered slice of chaotic data.
    /// Passes the raw telemetry through the 3-6-9 harmonic filter to extract
    /// resonant vectors. Returns `Some` if anomalies are found, allocating
    /// memory only when deterministic intent is justified.
    fn ingest_tick_stream(&self, raw_ticks: &[f64]) -> Option<Vec<f64>>;
}

/// A Financial Oracle for market telemetry and tick data.
pub struct FinancialOracle {}

impl FinancialOracle {
    pub fn new() -> Self {
        Self {}
    }
}

impl Oracle for FinancialOracle {
    fn ingest_tick_stream(&self, raw_ticks: &[f64]) -> Option<Vec<f64>> {
        let harmonic_anomalies = extract_tesla_harmonics(raw_ticks);
        
        if harmonic_anomalies.is_empty() {
            None
        } else {
            Some(harmonic_anomalies)
        }
    }
}

/// An Environmental Oracle capable of ingesting non-financial entropy
/// (e.g., simulated solar flare indices, lunar phases, cosmic ray telemetry).
pub struct EnvironmentalOracle {
    pub environmental_weight: f64,
}

impl EnvironmentalOracle {
    pub fn new(weight: f64) -> Self {
        Self {
            environmental_weight: weight,
        }
    }
}

impl Oracle for EnvironmentalOracle {
    fn ingest_tick_stream(&self, raw_ticks: &[f64]) -> Option<Vec<f64>> {
        // Weigh the raw ticks against the environmental factor before passing through the harmonic filter
        let weighted_ticks: Vec<f64> = raw_ticks.iter().map(|&t| t * self.environmental_weight).collect();
        let harmonic_anomalies = extract_tesla_harmonics(&weighted_ticks);
        
        if harmonic_anomalies.is_empty() {
            None
        } else {
            Some(harmonic_anomalies)
        }
    }
}
