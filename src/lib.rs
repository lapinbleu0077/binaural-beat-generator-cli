use anyhow::Error;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::sync::{Arc, Mutex};
use std::time::{Duration as StdDuration, Instant};
use std::{fmt, thread}; // Alias to avoid conflict with enum variant

//Cancellation support
use std::sync::atomic::{AtomicBool, Ordering};

// --- 1. Define Traits for Generic Parameters ---

/// Trait for types that can provide a frequency in Hz (f32).
pub trait ToFrequency {
    fn to_hz(&self) -> f32;
}

/// Trait for types that can provide a duration in minutes (u32).
pub trait ToMinutes {
    fn to_minutes(&self) -> u32;
}

// --- 2. Define Enums for Carrier Frequency ---

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

// --- 3. Define Enums for Beat Frequency (Binaural Beat) ---

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

impl ToFrequency for BeatFrequency {
    fn to_hz(&self) -> f32 {
        match self {
            BeatFrequency::Delta => 2.0,  // Typical beat frequency for Delta
            BeatFrequency::Theta => 6.0,  // Typical beat frequency for Theta
            BeatFrequency::Alpha => 10.0, // Typical beat frequency for Alpha
            BeatFrequency::Beta => 20.0,  // Typical beat frequency for Beta
            BeatFrequency::Gamma => 40.0, // Typical beat frequency for Gamma
            BeatFrequency::Custom(hz) => *hz,
        }
    }
}

// --- 4. Define Enums for Duration ---

/// Represents common durations in minutes.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Duration {
    FiveMinutes,
    TenMinutes,
    FifteenMinutes,
    TwentyMinutes,
    ThirtyMinutes,
    ThirtyFiveMinutes,
    FortyMinutes,
    FiftyMinutes,
    SixtyMinutes   
}

impl ToMinutes for Duration {
    fn to_minutes(&self) -> u32 {
        match self {
            Duration::FiveMinutes => 5,
            Duration::TenMinutes => 10,
            Duration::FifteenMinutes => 15,           
            Duration::TwentyMinutes => 20,
            Duration::ThirtyMinutes => 30,
            Duration::ThirtyFiveMinutes => 35,
            Duration::FortyMinutes => 40,
            Duration::FiftyMinutes => 50,            
            Duration::SixtyMinutes => 60
        }
    }
}

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

impl fmt::Display for Duration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Duration::FiveMinutes => write!(f, "5 min"),
            Duration::TenMinutes => write!(f, "10 min"),
            Duration::FifteenMinutes => write!(f, "15 min"),
            Duration::TwentyMinutes => write!(f, "20 min"),
            Duration::ThirtyMinutes => write!(f, "30 min"),
            Duration::ThirtyFiveMinutes => write!(f, "35 min"),
            Duration::FortyMinutes => write!(f, "40 min"),
            Duration::FiftyMinutes => write!(f, "50 min"),
            Duration::SixtyMinutes => write!(f, "60 min"),
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

fn wait_until_end(cancel_token: Arc<AtomicBool>, duration_minutes: u32) {
    let total_duration = StdDuration::from_secs((duration_minutes * 60) as u64);
    let start_time = Instant::now();

    println!("Playing for a maximum of {} minutes...", duration_minutes);

    while start_time.elapsed() < total_duration {
        // Break the loop immediately if the user requested cancellation
        if cancel_token.load(Ordering::Relaxed) {
            println!("Playback cancelled by user.");
            break;
        }
        // Sleep for a short period to avoid high CPU usage
        thread::sleep(StdDuration::from_millis(500));
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

pub fn duration_list() -> Vec<Duration> {
    return vec![Duration::FiveMinutes, Duration::TenMinutes, Duration::FifteenMinutes, 
                Duration::TwentyMinutes, Duration::ThirtyMinutes, Duration::ThirtyFiveMinutes, 
                Duration::FortyMinutes, Duration::FiftyMinutes, Duration::SixtyMinutes];
}

// --- 5. Generic Binaural Beat Generation Function ---

/// Generates and plays binaural beat tones based on specified carrier frequency,
/// beat frequency, and duration.
///
/// # Type Parameters
/// - `C`: Type implementing `ToFrequency` for the carrier frequency.
/// - `B`: Type implementing `ToFrequency` for the beat frequency.
/// - `D`: Type implementing `ToMinutes` for the duration.
///
/// # Arguments
/// - `carrier`: An instance of a type providing the carrier frequency.
/// - `beat`: An instance of a type providing the beat frequency.
/// - `duration`: An instance of a type providing the total duration.
///
/// # Returns
/// `Result<(), anyhow::Error>` indicating success or failure.
pub fn generate_binaural_beats<C, B, D>(
    carrier: C,
    beat: B,
    duration: D,
    cancel_token: Arc<AtomicBool>,
) -> Result<(), Error>
where
    C: ToFrequency,
    B: ToFrequency,
    D: ToMinutes,
{
    // Extract concrete values from generic parameters
    let carrier_hz = carrier.to_hz();
    let beat_hz = beat.to_hz();
    let duration_minutes = duration.to_minutes();

    // Calculate left and right ear frequencies
    let f_left = carrier_hz - (beat_hz / 2.0);
    let f_right = carrier_hz + (beat_hz / 2.0);

    // Basic validation for frequencies
    if f_left <= 0.0 || f_right <= 0.0 {
        return Err(anyhow::anyhow!(
            "Calculated frequency for one ear is zero or negative. Adjust carrier or beat frequency."
        ));
    }
    if duration_minutes == 0 {
        return Err(anyhow::anyhow!(
            "Duration must be greater than zero minutes."
        ));
    }

    println!("--- Binaural Beat Settings ---");
    println!("Carrier Frequency: {:.2} Hz", carrier_hz);
    println!("Beat Frequency: {:.2} Hz", beat_hz);
    println!("Left Ear Frequency: {:.2} Hz", f_left);
    println!("Right Ear Frequency: {:.2} Hz", f_right);
    println!("Duration: {} minutes", duration_minutes);
    println!("----------------------------");

    let host = cpal::default_host();

    let device = host
        .default_output_device()
        .ok_or_else(|| anyhow::anyhow!("No output device available."))?;

    let config = device.default_output_config()?;

    let sample_rate_val = config.sample_rate().0 as f64;
    let channels_val = config.channels() as usize;

    let sample_clock_left = Arc::new(Mutex::new(0f64));
    let sample_clock_right = Arc::new(Mutex::new(0f64));

    let sample_clock_left_for_closure = Arc::clone(&sample_clock_left);
    let sample_clock_right_for_closure = Arc::clone(&sample_clock_right);
    let stream_cancel_token = Arc::clone(&cancel_token); // Clone for the stream closure

    let stream = device.build_output_stream(
        &config.clone().into(), // Clone config for the stream builder
        move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
            // Check the token's state inside the audio loop
            if stream_cancel_token.load(Ordering::Relaxed) {
                // If the token is true, fill the buffer with silence and return
                for frame in data.chunks_mut(channels_val) {
                    if channels_val == 2 {
                        frame[0] = 0.0;
                        frame[1] = 0.0;
                    } else {
                        frame[0] = 0.0;
                    }
                }
                return;
            }

            let mut current_sample_clock_left = sample_clock_left_for_closure.lock().unwrap();
            let mut current_sample_clock_right = sample_clock_right_for_closure.lock().unwrap();

            for frame in data.chunks_mut(channels_val) {
                //Always keep the final sample outputs as f32 but make the calculations using f64 so that we don't lose the signal.
                let left_sample =
                    ((2.0 * std::f64::consts::PI * f_left as f64 * *current_sample_clock_left
                        / sample_rate_val)
                        .sin()) as f32;
                *current_sample_clock_left += 1.0;

                let right_sample =
                    ((2.0 * std::f64::consts::PI * f_right as f64 * *current_sample_clock_right
                        / sample_rate_val)
                        .sin()) as f32;
                *current_sample_clock_right += 1.0;

                if channels_val == 2 {
                    frame[0] = left_sample * 0.5; // Reduce amplitude to avoid clipping
                    frame[1] = right_sample * 0.5;
                } else {
                    frame[0] = (left_sample + right_sample) * 0.25; // For mono, sum and reduce further
                }
            }
        },
        |err| eprintln!("An error occurred on stream: {}", err),
        None,
    )?;

    stream.play()?;

    // The main thread now waits for EITHER the timer to expire OR the cancel token to be set.
    wait_until_end(cancel_token, duration_minutes);

    Ok(())
}
