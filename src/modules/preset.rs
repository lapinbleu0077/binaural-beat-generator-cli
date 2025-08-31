//! A module that contains code that allows for presets so that all settings can be easily used and passed around.
//!
use std::fmt;

use crate::modules::{
    duration::duration::Duration,
    frequency::{beat_frequency::BeatFrequency, carrier_frequency::CarrierFrequency},
};

/// This structure groups the basic values needed to run the binaural beat program.
#[derive(Debug, Clone, Copy)]
pub struct BinauralPresetGroup {
    pub preset: Preset,
    pub carrier: CarrierFrequency,
    pub beat: BeatFrequency,
    pub duration: Duration,
}

/// The preset enum allows the user to be able to select a preset to use on the command line.
#[derive(Debug, Clone, Copy)]
pub enum Preset {
    /// **Focus:**
    /// A preset for heightened concentration and alertness, typically used
    /// for studying or complex problem-solving. It utilizes Beta brainwaves
    /// (12-30 Hz) which are associated with active thinking.
    Focus,

    /// **High Focus:**
    /// A more intense version of the `Focus` preset, pushing the mind to
    /// higher levels of cognitive processing. It leverages Gamma brainwaves
    /// (30-100 Hz), linked to peak concentration and intelligence.
    HighFocus,

    /// **Relaxation:**
    /// Promotes a state of calm alertness, ideal for unwinding after a
    /// stressful day or for light meditation. This preset uses Alpha brainwaves
    /// (8-12 Hz).
    Relaxation,

    /// **Deep Relaxation:**
    /// A deeper state of calm, bridging the gap between wakefulness and
    /// sleep. It's often used for deep meditation or to prepare for rest.
    /// This preset uses Theta brainwaves (4-8 Hz).
    DeepRelaxation,

    /// **Sleep:**
    /// Designed to induce a state of deep, restorative sleep. It utilizes
    /// Delta brainwaves (0.5-4 Hz), which are associated with deep, dreamless sleep.
    Sleep,

    /// **Chanting:**
    /// A preset that mimics the meditative state achieved during chanting.
    /// It helps to calm the mind and body using Theta brainwaves (4-8 Hz).
    Chanting,

    /// **Intuition:**
    /// This preset is designed to enhance intuition and insight by fostering
    /// a Theta state, which is linked to creativity and subconscious processing.
    Intuition,

    /// **Astral:**
    /// An advanced preset aimed at assisting with out-of-body or astral projection
    /// experiences. It combines a deep Theta beat with a Delta carrier to induce
    /// a highly altered state of consciousness.
    Astral,

    /// **Healing:**
    /// This preset is thought to promote physical and mental healing by inducing
    /// a deep Delta state, which is associated with the body's natural restorative
    /// processes during sleep.
    Healing,

    /// **Alpha:**
    /// A preset that specifically targets the Alpha brainwave state (8-12 Hz)
    /// to encourage a feeling of relaxed awareness and stress reduction.
    Alpha,

    /// **Intelligence:**
    /// This preset stimulates the brain for enhanced learning and cognitive function.
    /// It primarily uses Gamma brainwaves (30-100 Hz), which are linked to
    /// high-level information processing.
    Intelligence,

    /// **Euphoria:**
    /// A preset designed to promote feelings of happiness and well-being.
    /// It utilizes a Gamma beat, which is often associated with endorphin
    /// release and positive emotional states.
    Euphoria,

    // --- Chakra Presets ---

    /// A group of presets designed to balance the seven main chakras. Each
    /// preset combines a specific chakra's carrier frequency with a
    /// binaural beat that induces a meditative or desired state.
    ///
    /// 

    // --- Crown Chakra Presets ---
    /// A collection of presets specifically targeting the Crown Chakra.
    /// They combine the Crown Chakra's carrier frequency with different
    /// mental states.

    /// **Crown Focus:**
    /// Combines the Crown Chakra's tuning fork frequency with a Beta beat
    /// for focused meditation on spiritual connection.
    CrownFocus,
    
    /// **Crown Relaxation:**
    /// Combines the Crown Chakra's tuning fork frequency with an Alpha beat
    /// to promote a relaxed spiritual state.
    CrownRelaxation,
    
    /// **Crown Sleep:**
    /// Combines the Crown Chakra's tuning fork frequency with a Delta beat
    /// for deep rest and spiritual renewal.
    CrownSleep,
    
    /// **Crown Chanting:**
    /// Combines the Crown Chakra's tuning fork frequency with a Theta beat
    /// for a deeply meditative state during spiritual practices.
    CrownChanting,
    
    /// **Crown Intuition:**
    /// Combines the Crown Chakra's tuning fork frequency with a Theta beat
    /// to enhance intuition and cosmic awareness.
    CrownIntuition,
    
    /// **Crown Astral:**
    /// Combines the Crown Chakra's tuning fork frequency with a Delta beat
    /// for advanced meditation and astral exploration.
    CrownAstral,

    // --- Solfeggio Chakra Presets ---
    /// These presets use the ancient Solfeggio tones as the carrier frequency
    /// for chakra balancing and meditation.
    
    /// **Solfeggio Root Chakra:**
    /// Uses the 396 Hz Solfeggio tone with a Delta beat for grounding and stability.
    SolfeggioRoot,
    
    /// **Solfeggio Sacral Chakra:**
    /// Uses the 417 Hz Solfeggio tone with a Theta beat for creativity and emotional release.
    SolfeggioSacral,
    
    /// **Solfeggio Solar Plexus Chakra:**
    /// Uses the 528 Hz Solfeggio tone with an Alpha beat for transformation and motivation.
    SolfeggioSolarPlexus,
    
    /// **Solfeggio Heart Chakra:**
    /// Uses the 639 Hz Solfeggio tone with an Alpha beat for love and connection.
    SolfeggioHeart,
    
    /// **Solfeggio Throat Chakra:**
    /// Uses the 741 Hz Solfeggio tone with a Beta beat for communication and expression.
    SolfeggioThroat,
    
    /// **Solfeggio Third Eye Chakra:**
    /// Uses the 852 Hz Solfeggio tone with a Beta beat for clarity and intuition.
    SolfeggioThirdEye,
    
    /// **Solfeggio Crown Chakra:**
    /// Uses the 963 Hz Solfeggio tone with a Gamma beat for spiritual connection and unity.
    SolfeggioCrown,

    // --- Tuning Fork Chakra Presets ---
    /// These presets use the Planetary/Tuning Fork frequencies as the
    /// carrier frequency for chakra balancing.
    
    /// **Tuning Fork Root Chakra:**
    /// Uses the 194.18 Hz Tuning Fork tone with a Delta beat for grounding.
    TuningForkRoot,
    
    /// **Tuning Fork Sacral Chakra:**
    /// Uses the 210.42 Hz Tuning Fork tone with a Theta beat for emotional flow.
    TuningForkSacral,
    
    /// **Tuning Fork Solar Plexus Chakra:**
    /// Uses the 126.22 Hz Tuning Fork tone with an Alpha beat for confidence.
    TuningForkSolarPlexus,
    
    /// **Tuning Fork Heart Chakra:**
    /// Uses the 136.10 Hz Tuning Fork tone with an Alpha beat for love and compassion.
    TuningForkHeart,
    
    /// **Tuning Fork Throat Chakra:**
    /// Uses the 141.27 Hz Tuning Fork tone with a Beta beat for communication.
    TuningForkThroat,
    
    /// **Tuning Fork Third Eye Chakra:**
    /// Uses the 221.23 Hz Tuning Fork tone with a Beta beat for insight and wisdom.
    TuningForkThirdEye,
    
    /// **Tuning Fork Crown Chakra:**
    /// Uses the 172.06 Hz Tuning Fork tone with a Gamma beat for spiritual transcendence.
    TuningForkCrown,
}

/// The this implementation converts a preset to a preset group of values based on predetermined settings.
impl From<Preset> for BinauralPresetGroup {
    fn from(preset: Preset) -> Self {
        match preset {
            // General Presets
            Preset::Focus => BinauralPresetGroup {
                preset: preset,
                carrier: CarrierFrequency::Beta,
                beat: BeatFrequency::Beta,
                duration: Duration::ThirtyMinutes,
            },
            Preset::HighFocus => BinauralPresetGroup {
                preset: preset,
                carrier: CarrierFrequency::Gamma,
                beat: BeatFrequency::Gamma,
                duration: Duration::ThirtyMinutes,
            },
            Preset::Relaxation => BinauralPresetGroup {
                preset: preset,
                carrier: CarrierFrequency::Alpha,
                beat: BeatFrequency::Alpha,
                duration: Duration::FifteenMinutes,
            },
            Preset::DeepRelaxation => BinauralPresetGroup {
                preset: preset,
                carrier: CarrierFrequency::Theta,
                beat: BeatFrequency::Theta,
                duration: Duration::FifteenMinutes,
            },
            Preset::Sleep => BinauralPresetGroup {
                preset: preset,
                carrier: CarrierFrequency::Delta,
                beat: BeatFrequency::Delta,
                duration: Duration::SixtyMinutes,
            },
            Preset::Chanting => BinauralPresetGroup {
                preset: preset,
                carrier: CarrierFrequency::Theta,
                beat: BeatFrequency::Theta,
                duration: Duration::ThirtyMinutes,
            },
            Preset::Intuition => BinauralPresetGroup {
                preset: preset,
                carrier: CarrierFrequency::Theta,
                beat: BeatFrequency::Theta,
                duration: Duration::FifteenMinutes,
            },
            Preset::Astral => BinauralPresetGroup {
                preset: preset,
                carrier: CarrierFrequency::Custom(140.0),
                beat: BeatFrequency::Custom(6.3),
                duration: Duration::SixtyMinutes,
            },
            Preset::Healing => BinauralPresetGroup {
                preset: preset,
                carrier: CarrierFrequency::Delta,
                beat: BeatFrequency::Theta,
                duration: Duration::SixtyMinutes,
            },
            Preset::Alpha => BinauralPresetGroup {
                preset: preset,
                carrier: CarrierFrequency::Alpha,
                beat: BeatFrequency::Alpha,
                duration: Duration::ThirtyMinutes,
            },
            Preset::Intelligence => BinauralPresetGroup {
                preset: preset,
                carrier: CarrierFrequency::Gamma,
                beat: BeatFrequency::Gamma,
                duration: Duration::TenMinutes,
            },
            Preset::Euphoria => BinauralPresetGroup {
                preset: preset,
                carrier: CarrierFrequency::Custom(210.42),
                beat: BeatFrequency::Custom(20.0),
                duration: Duration::TenMinutes,
            },

            // Crown Chakra Presets
            Preset::CrownFocus => BinauralPresetGroup {
                preset: preset,
                carrier: CarrierFrequency::TuningForkCrown,
                beat: BeatFrequency::Beta,
                duration: Duration::ThirtyMinutes,
            },
            Preset::CrownRelaxation => BinauralPresetGroup {
                preset: preset,
                carrier: CarrierFrequency::TuningForkCrown,
                beat: BeatFrequency::Alpha,
                duration: Duration::FifteenMinutes,
            },
            Preset::CrownSleep => BinauralPresetGroup {
                preset: preset,
                carrier: CarrierFrequency::TuningForkCrown,
                beat: BeatFrequency::Delta,
                duration: Duration::SixtyMinutes,
            },
            Preset::CrownChanting => BinauralPresetGroup {
                preset: preset,
                carrier: CarrierFrequency::TuningForkCrown,
                beat: BeatFrequency::Theta,
                duration: Duration::ThirtyMinutes,
            },
            Preset::CrownIntuition => BinauralPresetGroup {
                preset: preset,
                carrier: CarrierFrequency::TuningForkCrown,
                beat: BeatFrequency::Theta,
                duration: Duration::FifteenMinutes,
            },
            Preset::CrownAstral => BinauralPresetGroup {
                preset: preset,
                carrier: CarrierFrequency::TuningForkCrown,
                beat: BeatFrequency::Delta,
                duration: Duration::SixtyMinutes,
            },

            // Solfeggio Chakra Presets
            Preset::SolfeggioRoot => BinauralPresetGroup {
                preset: preset,
                carrier: CarrierFrequency::SolfeggioRoot,
                beat: BeatFrequency::Delta,
                duration: Duration::ThirtyMinutes,
            },
            Preset::SolfeggioSacral => BinauralPresetGroup {
                preset: preset,
                carrier: CarrierFrequency::SolfeggioSacral,
                beat: BeatFrequency::Theta,
                duration: Duration::ThirtyMinutes,
            },
            Preset::SolfeggioSolarPlexus => BinauralPresetGroup {
                preset: preset,
                carrier: CarrierFrequency::SolfeggioSolarPlexus,
                beat: BeatFrequency::Alpha,
                duration: Duration::ThirtyMinutes,
            },
            Preset::SolfeggioHeart => BinauralPresetGroup {
                preset: preset,
                carrier: CarrierFrequency::SolfeggioHeart,
                beat: BeatFrequency::Alpha,
                duration: Duration::FifteenMinutes,
            },
            Preset::SolfeggioThroat => BinauralPresetGroup {
                preset: preset,
                carrier: CarrierFrequency::SolfeggioThroat,
                beat: BeatFrequency::Beta,
                duration: Duration::TenMinutes,
            },
            Preset::SolfeggioThirdEye => BinauralPresetGroup {
                preset: preset,
                carrier: CarrierFrequency::SolfeggioThirdEye,
                beat: BeatFrequency::Beta,
                duration: Duration::TenMinutes,
            },
            Preset::SolfeggioCrown => BinauralPresetGroup {
                preset: preset,
                carrier: CarrierFrequency::SolfeggioCrown,
                beat: BeatFrequency::Gamma,
                duration: Duration::TenMinutes,
            },

            // Tuning Fork Chakra Presets
            Preset::TuningForkRoot => BinauralPresetGroup {
                preset: preset,
                carrier: CarrierFrequency::TuningForkRoot,
                beat: BeatFrequency::Delta,
                duration: Duration::ThirtyMinutes,
            },
            Preset::TuningForkSacral => BinauralPresetGroup {
                preset: preset,
                carrier: CarrierFrequency::TuningForkSacral,
                beat: BeatFrequency::Theta,
                duration: Duration::ThirtyMinutes,
            },
            Preset::TuningForkSolarPlexus => BinauralPresetGroup {
                preset: preset,
                carrier: CarrierFrequency::TuningForkSolarPlexus,
                beat: BeatFrequency::Alpha,
                duration: Duration::ThirtyMinutes,
            },
            Preset::TuningForkHeart => BinauralPresetGroup {
                preset: preset,
                carrier: CarrierFrequency::TuningForkHeart,
                beat: BeatFrequency::Alpha,
                duration: Duration::FifteenMinutes,
            },
            Preset::TuningForkThroat => BinauralPresetGroup {
                preset: preset,
                carrier: CarrierFrequency::TuningForkThroat,
                beat: BeatFrequency::Beta,
                duration: Duration::TenMinutes,
            },
            Preset::TuningForkThirdEye => BinauralPresetGroup {
                preset: preset,
                carrier: CarrierFrequency::TuningForkThirdEye,
                beat: BeatFrequency::Beta,
                duration: Duration::TenMinutes,
            },
            Preset::TuningForkCrown => BinauralPresetGroup {
                preset: preset,
                carrier: CarrierFrequency::TuningForkCrown,
                beat: BeatFrequency::Gamma,
                duration: Duration::TenMinutes,
            },
        }
    }
}

/// This implementation returns the human readable text name for for the preset enum.
impl fmt::Display for Preset {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Preset::Focus => write!(f, "Focus"),
            Preset::HighFocus => write!(f, "High Focus"),
            Preset::Relaxation => write!(f, "Relaxation"),
            Preset::DeepRelaxation => write!(f, "Deep Relaxation"),
            Preset::Sleep => write!(f, "Sleep"),
            Preset::Chanting => write!(f, "Chanting"),
            Preset::Intuition => write!(f, "Intuition"),
            Preset::Astral => write!(f, "Astral"),
            Preset::Healing => write!(f, "Healing"),
            Preset::Alpha => write!(f, "Alpha"),
            Preset::Intelligence => write!(f, "Intelligence"),
            Preset::Euphoria => write!(f, "Euphoria"),
            Preset::CrownFocus => write!(f, "Crown Chakra Focus"),
            Preset::CrownRelaxation => write!(f, "Crown Chakra Relaxation"),
            Preset::CrownSleep => write!(f, "Crown Chakra Sleep"),
            Preset::CrownChanting => write!(f, "Crown Chakra Chanting"),
            Preset::CrownIntuition => write!(f, "Crown Chakra Intuition"),
            Preset::CrownAstral => write!(f, "Crown Chakra Astral"),
            Preset::SolfeggioRoot => write!(f, "Solfeggio Root Chakra"),
            Preset::SolfeggioSacral => write!(f, "Solfeggio Sacral Chakra"),
            Preset::SolfeggioSolarPlexus => write!(f, "Solfeggio Solar Plexus Chakra"),
            Preset::SolfeggioHeart => write!(f, "Solfeggio Heart Chakra"),
            Preset::SolfeggioThroat => write!(f, "Solfeggio Throat Chakra"),
            Preset::SolfeggioThirdEye => write!(f, "Solfeggio Third Eye Chakra"),
            Preset::SolfeggioCrown => write!(f, "Solfeggio Crown Chakra"),
            Preset::TuningForkRoot => write!(f, "Tuning Fork Root Chakra"),
            Preset::TuningForkSacral => write!(f, "Tuning Fork Sacral Chakra"),
            Preset::TuningForkSolarPlexus => write!(f, "Tuning Fork Solar Plexus Chakra"),
            Preset::TuningForkHeart => write!(f, "Tuning Fork Heart Chakra"),
            Preset::TuningForkThroat => write!(f, "Tuning Fork Throat Chakra"),
            Preset::TuningForkThirdEye => write!(f, "Tuning Fork Third Eye Chakra"),
            Preset::TuningForkCrown => write!(f, "Tuning Fork Crown Chakra"),
        }
    }
}

/// This function returns all of the presets used in a vector.
pub fn preset_list() -> Vec<Preset> {
    return vec![
        Preset::Focus,
        Preset::HighFocus,
        Preset::Relaxation,
        Preset::DeepRelaxation,
        Preset::Sleep,
        Preset::Chanting,
        Preset::Intuition,
        Preset::Astral,
        Preset::Healing,
        Preset::Alpha,
        Preset::Intelligence,
        Preset::Euphoria,
        Preset::CrownFocus,
        Preset::CrownRelaxation,
        Preset::CrownSleep,
        Preset::CrownChanting,
        Preset::CrownIntuition,
        Preset::CrownAstral,
        Preset::SolfeggioRoot,
        Preset::SolfeggioSacral,
        Preset::SolfeggioSolarPlexus,
        Preset::SolfeggioHeart,
        Preset::SolfeggioThroat,
        Preset::SolfeggioThirdEye,
        Preset::SolfeggioCrown,
        Preset::TuningForkRoot,
        Preset::TuningForkSacral,
        Preset::TuningForkSolarPlexus,
        Preset::TuningForkHeart,
        Preset::TuningForkThroat,
        Preset::TuningForkThirdEye,
        Preset::TuningForkCrown,
    ];
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

     macro_rules! test_preset_enum_to_text_description_cases {
        ($($name:ident:($a:expr, $expected:expr),)*) => {
            $(
                #[test]
                fn $name() {
                    assert_eq!($a,$expected)
                }
            )*
        };
    }

    #[test]
    fn test_preset_list_length() {
        let lst = preset_list();
        assert_eq!(32,lst.len())
    }

    test_preset_enum_to_text_description_cases ! {
        preset_text_focus: (Preset::Focus.to_string(), "Focus"),
        preset_text_high_focus: (Preset::HighFocus.to_string(), "High Focus"),
        preset_text_relaxation: (Preset::Relaxation.to_string(), "Relaxation"),
        preset_text_deep_relaxation: (Preset::DeepRelaxation.to_string(), "Deep Relaxation"),
        preset_text_sleep: (Preset::Sleep.to_string(), "Sleep"),
        preset_text_chanting: (Preset::Chanting.to_string(), "Chanting"),
        preset_text_intuition: (Preset::Intuition.to_string(), "Intuition"),
        preset_text_astral: (Preset::Astral.to_string(), "Astral"),
        preset_text_healing: (Preset::Healing.to_string(), "Healing"),
        preset_text_alpha: (Preset::Alpha.to_string(), "Alpha"),
        preset_text_intelligence: (Preset::Intelligence.to_string(), "Intelligence"),
        preset_text_euphoria: (Preset::Euphoria.to_string(), "Euphoria"),
        preset_text_crown_chakra_focus: (Preset::CrownFocus.to_string(), "Crown Chakra Focus"),
        preset_text_crown_relaxation: (Preset::CrownRelaxation.to_string(), "Crown Chakra Relaxation"),
        preset_text_crown_sleep: (Preset::CrownSleep.to_string(), "Crown Chakra Sleep"),
        preset_text_crown_chanting: (Preset::CrownChanting.to_string(), "Crown Chakra Chanting"),
        preset_text_crown_intuition: (Preset::CrownIntuition.to_string(), "Crown Chakra Intuition"),
        preset_text_crown_astral: (Preset::CrownAstral.to_string(), "Crown Chakra Astral"),
        preset_text_solfeggio_root: (Preset::SolfeggioRoot.to_string(), "Solfeggio Root Chakra"),
        preset_text_solfeggio_sacral: (Preset::SolfeggioSacral.to_string(), "Solfeggio Sacral Chakra"),
        preset_text_solfeggio_solar_plexus: (Preset::SolfeggioSolarPlexus.to_string(), "Solfeggio Solar Plexus Chakra"),
        preset_text_solfeggio_heart: (Preset::SolfeggioHeart.to_string(), "Solfeggio Heart Chakra"),
        preset_text_solfeggio_throat: (Preset::SolfeggioThroat.to_string(), "Solfeggio Throat Chakra"),
        preset_text_solfeggio_third_eye: (Preset::SolfeggioThirdEye.to_string(), "Solfeggio Third Eye Chakra"),
        preset_text_solfeggio_crown: (Preset::SolfeggioCrown.to_string(), "Solfeggio Crown Chakra"),
        preset_text_tuning_fork_root: (Preset::TuningForkRoot.to_string(), "Tuning Fork Root Chakra"),
        preset_text_tuning_fork_sacral: (Preset::TuningForkSacral.to_string(), "Tuning Fork Sacral Chakra"),
        preset_text_tuning_fork_solar_plexus: (Preset::TuningForkSolarPlexus.to_string(), "Tuning Fork Solar Plexus Chakra"),
        preset_text_tuning_fork_heart: (Preset::TuningForkHeart.to_string(), "Tuning Fork Heart Chakra"),
        preset_text_tuning_fork_throat: (Preset::TuningForkThroat.to_string(), "Tuning Fork Throat Chakra"),
        preset_text_tuning_fork_third_eye: (Preset::TuningForkThirdEye.to_string(), "Tuning Fork Third Eye Chakra"),
        preset_text_tuning_fork_crown: (Preset::TuningForkCrown.to_string(), "Tuning Fork Crown Chakra"),
    }

}