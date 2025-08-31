//! A module that contains code related to the carrier functionality.

use crate::modules::frequency::frequency_common::ToFrequency;

/// Represents common brainwave carrier frequencies.
#[derive(Debug, Clone, Copy, PartialEq)]
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



#[cfg(test)]
mod test {
    use super::*;

    macro_rules! test_carrier_frequency_to_integer_cases {
        ($($name:ident:($a:expr, $expected:expr),)*) => {
            $(
                #[test]
                fn $name() {
                    assert_eq!(CarrierFrequency::to_hz($a),$expected)
                }
            )*
        };
    }

    test_carrier_frequency_to_integer_cases! {
        test_carrier_frequency_delta_enum_to_integer: (&CarrierFrequency::Delta, 100.0),
        test_carrier_frequency_theta_enum_to_integer: (&CarrierFrequency::Theta , 200.0),
        test_carrier_frequency_alpha_enum_to_integer: (&CarrierFrequency::Alpha , 300.0),
        test_carrier_frequency_beta_enum_to_integer: (&CarrierFrequency::Beta , 400.0),
        test_carrier_frequency_gamma_enum_to_integer: (&CarrierFrequency::Gamma , 500.0),
        
        test_carrier_frequency_solfeggio_root_enum_to_integer: (&CarrierFrequency::SolfeggioRoot , 396.0),
        test_carrier_frequency_solfeggio_sacral_enum_to_integer: (&CarrierFrequency::SolfeggioSacral , 417.0),
        test_carrier_frequency_solfeggio_solar_plexus_enum_to_integer: (&CarrierFrequency::SolfeggioSolarPlexus , 528.0),
        test_carrier_frequency_solfeggio_heart_enum_to_integer: (&CarrierFrequency::SolfeggioHeart , 639.0),
        test_carrier_frequency_solfeggio_throat_enum_to_integer: (&CarrierFrequency::SolfeggioThroat , 741.0),
        test_carrier_frequency_solfeggio_third_eye_enum_to_integer: (&CarrierFrequency::SolfeggioThirdEye , 852.0),
        test_carrier_frequency_solfeggio_crown_enum_to_integer: (&CarrierFrequency::SolfeggioCrown , 963.0),
        
        test_carrier_frequency_tuning_fork_root_enum_to_integer: (&CarrierFrequency::TuningForkRoot , 194.18),
        test_carrier_frequency_tuning_fork_sacral_enum_to_integer: (&CarrierFrequency::TuningForkSacral , 210.42),
        test_carrier_frequency_tuning_fork_solar_plexus_enum_to_integer: (&CarrierFrequency::TuningForkSolarPlexus , 126.22),
        test_carrier_frequency_tuning_fork_heart_enum_to_integer: (&CarrierFrequency::TuningForkHeart , 136.10),
        test_carrier_frequency_tuning_fork_throat_enum_to_integer: (&CarrierFrequency::TuningForkThroat , 141.27),
        test_carrier_frequency_tuning_fork_third_eye_enum_to_integer: (&CarrierFrequency::TuningForkThirdEye , 221.23),
        test_carrier_frequency_tuning_fork_crown_enum_to_integer: (&CarrierFrequency::TuningForkCrown , 172.06),
        test_carrier_frequency_custom_enum_to_integer: (&CarrierFrequency::Custom(199.99) , 199.99),

    }
}