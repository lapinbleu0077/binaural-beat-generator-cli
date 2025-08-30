//! A module that contains code related to the beat functionality.

use crate::modules::frequency::frequency_common::ToFrequency;

/// Represents common brainwave beat frequencies.
#[derive(Debug, Clone, Copy)]
pub enum BeatFrequency {
    /// Delta wave range (0.5 - 4 Hz), for deep relaxation, sleep.
    Delta,
    /// Theta wave range (4 - 8 Hz), for meditation, creativity.
    Theta,
    /// Alpha wave range (8 - 12 Hz), for relaxation, focus.
    Alpha,
    /// Beta wave range (12 - 30 Hz), for alertness, concentration.
    Beta,
    /// Gamma wave range (30 - 100 Hz), for high-level cognitive processing.
    Gamma,
    /// Allows specifying a custom beat frequency in Hz.
    Custom(f32),
}

/// This implementation conerts the BeatFrequency enum to a known or different and concrete frequency.
impl ToFrequency for BeatFrequency {
    fn to_hz(&self) -> f32 {
        match self {
            // Typical beat frequency for Delta, Theta, Alpha, Beta & Gamma
            BeatFrequency::Delta => 2.0,
            BeatFrequency::Theta => 6.0,
            BeatFrequency::Alpha => 10.0,
            BeatFrequency::Beta => 20.0,
            BeatFrequency::Gamma => 40.0,
            BeatFrequency::Custom(hz) => *hz,
        }
    }
}
