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


#[cfg(test)]
mod test {
    use super::*;

    macro_rules! test_beat_frequency_to_integer_cases {
        ($($name:ident:($a:expr, $expected:expr),)*) => {
            $(
                #[test]
                fn $name() {
                    assert_eq!(BeatFrequency::to_hz($a),$expected)
                }
            )*
        };
    }

    test_beat_frequency_to_integer_cases! {
        test_beat_freuency_delta_enum_to_integer: (&BeatFrequency::Delta, 2.0),
        test_beat_freuency_theta_enum_to_integer: (&BeatFrequency::Theta, 6.0),
        test_beat_freuency_alpha_enum_to_integer: (&BeatFrequency::Alpha, 10.0),
        test_beat_freuency_beta_enum_to_integer: (&BeatFrequency::Beta, 20.0),
        test_beat_freuency_gamma_enum_to_integer: (&BeatFrequency::Gamma, 40.0),
        test_beat_freuency_custom_enum_to_integer: (&BeatFrequency::Custom(99.9), 99.9),
    }
}