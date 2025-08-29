use std::fmt;

use crate::modules::{
    duration::duration::Duration,
    frequency::{beat_frequency::BeatFrequency, carrier_frequency::CarrierFrequency},
};

#[derive(Debug, Clone, Copy)]
pub struct BinauralPreset {
    pub carrier: CarrierFrequency,
    pub beat: BeatFrequency,
    pub duration: Duration,
}

// The new Preset enum
#[derive(Debug, Clone, Copy)]
pub enum Preset {
    // General presets
    Focus,
    HighFocus,
    Relaxation,
    DeepRelaxation,
    Sleep,
    Chanting,
    Intuition,
    Astral,
    Healing,
    Alpha,
    Intelligence,
    Euphoria,
    // Crown Chakra presets
    CrownFocus,
    CrownRelaxation,
    CrownSleep,
    CrownChanting,
    CrownIntuition,
    CrownAstral,
    // Solfeggio Chakra presets
    SolfeggioRoot,
    SolfeggioSacral,
    SolfeggioSolarPlexus,
    SolfeggioHeart,
    SolfeggioThroat,
    SolfeggioThirdEye,
    SolfeggioCrown,
    // Tuning Fork Chakra presets
    TuningForkRoot,
    TuningForkSacral,
    TuningForkSolarPlexus,
    TuningForkHeart,
    TuningForkThroat,
    TuningForkThirdEye,
    TuningForkCrown,
}

impl From<Preset> for BinauralPreset {
    fn from(preset: Preset) -> Self {
        match preset {
            // General Presets
            Preset::Focus => BinauralPreset {
                carrier: CarrierFrequency::Beta,
                beat: BeatFrequency::Beta,
                duration: Duration::ThirtyMinutes,
            },
            Preset::HighFocus => BinauralPreset {
                carrier: CarrierFrequency::Gamma,
                beat: BeatFrequency::Gamma,
                duration: Duration::ThirtyMinutes,
            },
            Preset::Relaxation => BinauralPreset {
                carrier: CarrierFrequency::Alpha,
                beat: BeatFrequency::Alpha,
                duration: Duration::FifteenMinutes,
            },
            Preset::DeepRelaxation => BinauralPreset {
                carrier: CarrierFrequency::Theta,
                beat: BeatFrequency::Theta,
                duration: Duration::FifteenMinutes,
            },
            Preset::Sleep => BinauralPreset {
                carrier: CarrierFrequency::Delta,
                beat: BeatFrequency::Delta,
                duration: Duration::SixtyMinutes,
            },
            Preset::Chanting => BinauralPreset {
                carrier: CarrierFrequency::Theta,
                beat: BeatFrequency::Theta,
                duration: Duration::ThirtyMinutes,
            },
            Preset::Intuition => BinauralPreset {
                carrier: CarrierFrequency::Theta,
                beat: BeatFrequency::Theta,
                duration: Duration::FifteenMinutes,
            },
            Preset::Astral => BinauralPreset {
                carrier: CarrierFrequency::Custom(140.0),
                beat: BeatFrequency::Custom(6.3),
                duration: Duration::SixtyMinutes,
            },
            Preset::Healing => BinauralPreset {
                carrier: CarrierFrequency::Delta,
                beat: BeatFrequency::Theta,
                duration: Duration::SixtyMinutes,
            },
            Preset::Alpha => BinauralPreset {
                carrier: CarrierFrequency::Alpha,
                beat: BeatFrequency::Alpha,
                duration: Duration::ThirtyMinutes,
            },
            Preset::Intelligence => BinauralPreset {
                carrier: CarrierFrequency::Gamma,
                beat: BeatFrequency::Gamma,
                duration: Duration::TenMinutes,
            },
            Preset::Euphoria => BinauralPreset {
                carrier: CarrierFrequency::Custom(210.42),
                beat: BeatFrequency::Custom(20.0),
                duration: Duration::TenMinutes,
            },

            // Crown Chakra Presets
            Preset::CrownFocus => BinauralPreset {
                carrier: CarrierFrequency::TuningForkCrown,
                beat: BeatFrequency::Beta,
                duration: Duration::ThirtyMinutes,
            },
            Preset::CrownRelaxation => BinauralPreset {
                carrier: CarrierFrequency::TuningForkCrown,
                beat: BeatFrequency::Alpha,
                duration: Duration::FifteenMinutes,
            },
            Preset::CrownSleep => BinauralPreset {
                carrier: CarrierFrequency::TuningForkCrown,
                beat: BeatFrequency::Delta,
                duration: Duration::SixtyMinutes,
            },
            Preset::CrownChanting => BinauralPreset {
                carrier: CarrierFrequency::TuningForkCrown,
                beat: BeatFrequency::Theta,
                duration: Duration::ThirtyMinutes,
            },
            Preset::CrownIntuition => BinauralPreset {
                carrier: CarrierFrequency::TuningForkCrown,
                beat: BeatFrequency::Theta,
                duration: Duration::FifteenMinutes,
            },
            Preset::CrownAstral => BinauralPreset {
                carrier: CarrierFrequency::TuningForkCrown,
                beat: BeatFrequency::Delta,
                duration: Duration::SixtyMinutes,
            },

            // Solfeggio Chakra Presets
            Preset::SolfeggioRoot => BinauralPreset {
                carrier: CarrierFrequency::SolfeggioRoot,
                beat: BeatFrequency::Delta,
                duration: Duration::ThirtyMinutes,
            },
            Preset::SolfeggioSacral => BinauralPreset {
                carrier: CarrierFrequency::SolfeggioSacral,
                beat: BeatFrequency::Theta,
                duration: Duration::ThirtyMinutes,
            },
            Preset::SolfeggioSolarPlexus => BinauralPreset {
                carrier: CarrierFrequency::SolfeggioSolarPlexus,
                beat: BeatFrequency::Alpha,
                duration: Duration::ThirtyMinutes,
            },
            Preset::SolfeggioHeart => BinauralPreset {
                carrier: CarrierFrequency::SolfeggioHeart,
                beat: BeatFrequency::Alpha,
                duration: Duration::FifteenMinutes,
            },
            Preset::SolfeggioThroat => BinauralPreset {
                carrier: CarrierFrequency::SolfeggioThroat,
                beat: BeatFrequency::Beta,
                duration: Duration::TenMinutes,
            },
            Preset::SolfeggioThirdEye => BinauralPreset {
                carrier: CarrierFrequency::SolfeggioThirdEye,
                beat: BeatFrequency::Beta,
                duration: Duration::TenMinutes,
            },
            Preset::SolfeggioCrown => BinauralPreset {
                carrier: CarrierFrequency::SolfeggioCrown,
                beat: BeatFrequency::Gamma,
                duration: Duration::TenMinutes,
            },

            // Tuning Fork Chakra Presets
            Preset::TuningForkRoot => BinauralPreset {
                carrier: CarrierFrequency::TuningForkRoot,
                beat: BeatFrequency::Delta,
                duration: Duration::ThirtyMinutes,
            },
            Preset::TuningForkSacral => BinauralPreset {
                carrier: CarrierFrequency::TuningForkSacral,
                beat: BeatFrequency::Theta,
                duration: Duration::ThirtyMinutes,
            },
            Preset::TuningForkSolarPlexus => BinauralPreset {
                carrier: CarrierFrequency::TuningForkSolarPlexus,
                beat: BeatFrequency::Alpha,
                duration: Duration::ThirtyMinutes,
            },
            Preset::TuningForkHeart => BinauralPreset {
                carrier: CarrierFrequency::TuningForkHeart,
                beat: BeatFrequency::Alpha,
                duration: Duration::FifteenMinutes,
            },
            Preset::TuningForkThroat => BinauralPreset {
                carrier: CarrierFrequency::TuningForkThroat,
                beat: BeatFrequency::Beta,
                duration: Duration::TenMinutes,
            },
            Preset::TuningForkThirdEye => BinauralPreset {
                carrier: CarrierFrequency::TuningForkThirdEye,
                beat: BeatFrequency::Beta,
                duration: Duration::TenMinutes,
            },
            Preset::TuningForkCrown => BinauralPreset {
                carrier: CarrierFrequency::TuningForkCrown,
                beat: BeatFrequency::Gamma,
                duration: Duration::TenMinutes,
            },
        }
    }
}

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
