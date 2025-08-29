// --- 2. Define Enums for Carrier Frequency ---

use crate::modules::frequency::frequency_common::ToFrequency;

/// Represents common brainwave carrier frequencies.
#[derive(Debug, Clone, Copy)]
pub enum CarrierFrequency {
    /// Delta wave range (0.5 - 4 Hz), often associated with deep sleep.
    Delta,
    /// Theta wave range (4 - 8 Hz), associated with meditation, relaxation, and creativity.
    Theta,
    /// Alpha wave range (8 - 12 Hz), associated with relaxed, yet alert states.
    Alpha,
    /// Beta wave range (12 - 30 Hz), associated with active, busy, or anxious thinking.
    Beta,
    /// Gamma wave range (30 - 100 Hz), associated with higher-level cognitive functions.
    Gamma,

    SolfeggioRoot,
    SolfeggioSacral,
    SolfeggioSolarPlexus,
    SolfeggioHeart,
    SolfeggioThroat,
    SolfeggioThirdEye,
    SolfeggioCrown,

    TuningForkRoot,
    TuningForkSacral,
    TuningForkSolarPlexus,
    TuningForkHeart,
    TuningForkThroat,
    TuningForkThirdEye,
    TuningForkCrown,

    /// Allows specifying a custom carrier frequency in Hz.
    Custom(f32),
}

impl ToFrequency for CarrierFrequency {
    fn to_hz(&self) -> f32 {
        match self {
            CarrierFrequency::Delta => 100.0, // Example base for Delta, often higher than beat freq
            CarrierFrequency::Theta => 200.0,
            CarrierFrequency::Alpha => 300.0,
            CarrierFrequency::Beta => 400.0,
            CarrierFrequency::Gamma => 500.0,

            // Solfeggio Tones
            CarrierFrequency::SolfeggioRoot => 396.0,
            CarrierFrequency::SolfeggioSacral => 417.0,
            CarrierFrequency::SolfeggioSolarPlexus => 528.0,
            CarrierFrequency::SolfeggioHeart => 639.0,
            CarrierFrequency::SolfeggioThroat => 741.0,
            CarrierFrequency::SolfeggioThirdEye => 852.0,
            CarrierFrequency::SolfeggioCrown => 963.0,

            // Planetary/Tuning Fork Tones
            CarrierFrequency::TuningForkRoot => 194.18,
            CarrierFrequency::TuningForkSacral => 210.42,
            CarrierFrequency::TuningForkSolarPlexus => 126.22,
            CarrierFrequency::TuningForkHeart => 136.10,
            CarrierFrequency::TuningForkThroat => 141.27,
            CarrierFrequency::TuningForkThirdEye => 221.23,
            CarrierFrequency::TuningForkCrown => 172.06,

            CarrierFrequency::Custom(hz) => *hz,
        }
    }
}
