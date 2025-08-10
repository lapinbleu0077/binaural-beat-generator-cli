extern crate cpal;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use anyhow::Error;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration as StdDuration; // Alias to avoid conflict with enum variant

// --- 1. Define Traits for Generic Parameters ---

/// Trait for types that can provide a frequency in Hz (f32).
pub trait ToFrequency {
    fn to_hz(&self) -> f32;
}

/// Trait for types that can provide a duration in minutes (u64).
pub trait ToMinutes {
    fn to_minutes(&self) -> u64;
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
            BeatFrequency::Delta => 2.0, // Typical beat frequency for Delta
            BeatFrequency::Theta => 6.0, // Typical beat frequency for Theta
            BeatFrequency::Alpha => 10.0, // Typical beat frequency for Alpha
            BeatFrequency::Beta => 20.0, // Typical beat frequency for Beta
            BeatFrequency::Gamma => 40.0, // Typical beat frequency for Gamma
            BeatFrequency::Custom(hz) => *hz,
        }
    }
}

// --- 4. Define Enums for Duration ---

/// Represents common durations in minutes.
#[derive(Debug, Clone, Copy)]
pub enum Duration {
    FiveMinutes,
    TenMinutes,
    FifteenMinutes,
    ThirtyMinutes,
    SixtyMinutes,
    /// Allows specifying a custom duration in minutes.
    Custom(u64),
}

impl ToMinutes for Duration {
    fn to_minutes(&self) -> u64 {
        match self {
            Duration::FiveMinutes => 5,
            Duration::TenMinutes => 10,
            Duration::FifteenMinutes => 15,
            Duration::ThirtyMinutes => 30,
            Duration::SixtyMinutes => 60,
            Duration::Custom(minutes) => *minutes,
        }
    }
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
        return Err(anyhow::anyhow!("Calculated frequency for one ear is zero or negative. Adjust carrier or beat frequency."));
    }
    if duration_minutes == 0 {
        return Err(anyhow::anyhow!("Duration must be greater than zero minutes."));
    }

    println!("--- Binaural Beat Settings ---");
    println!("Carrier Frequency: {:.2} Hz", carrier_hz);
    println!("Beat Frequency: {:.2} Hz", beat_hz);
    println!("Left Ear Frequency: {:.2} Hz", f_left);
    println!("Right Ear Frequency: {:.2} Hz", f_right);
    println!("Duration: {} minutes", duration_minutes);
    println!("----------------------------");

    let host = cpal::default_host();
    
    let device = host.default_output_device()
        .ok_or_else(|| anyhow::anyhow!("No output device available."))?;
    
    let config = device.default_output_config()?;

    let sample_rate_val = config.sample_rate().0 as f32;
    let channels_val = config.channels() as usize;

    let sample_clock_left = Arc::new(Mutex::new(0f32));
    let sample_clock_right = Arc::new(Mutex::new(0f32));

    let sample_clock_left_for_closure = Arc::clone(&sample_clock_left);
    let sample_clock_right_for_closure = Arc::clone(&sample_clock_right);

    let stream = device.build_output_stream(
        &config.clone().into(), // Clone config for the stream builder
        move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
            let mut current_sample_clock_left = sample_clock_left_for_closure.lock().unwrap();
            let mut current_sample_clock_right = sample_clock_right_for_closure.lock().unwrap();

            for frame in data.chunks_mut(channels_val) {
                let left_sample = (2.0 * std::f32::consts::PI * f_left * *current_sample_clock_left / sample_rate_val).sin();
                *current_sample_clock_left += 1.0;

                let right_sample = (2.0 * std::f32::consts::PI * f_right * *current_sample_clock_right / sample_rate_val).sin();
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
        None
    )?;

    stream.play()?;

    let total_duration_secs = duration_minutes * 60;
    println!("Playing for {} seconds...", total_duration_secs);
    thread::sleep(StdDuration::from_secs(total_duration_secs));
    println!("Playback finished.");

    Ok(())
}

// --- Main function to demonstrate usage ---

fn relaxation() -> Result<(), Error> {
    /*
    For relaxation, you'll want to use frequencies in the Alpha and Theta ranges.

        Alpha (8-12 Hz): This range is associated with a state of relaxed alertness, perfect for de-stressing or light meditation.
        Theta (4-8 Hz): This range is linked to deep relaxation, meditation, and creativity.
    */

    generate_binaural_beats(
        CarrierFrequency::Alpha, // Or a custom value like CarrierFrequency::Custom(350.0)
        BeatFrequency::Alpha,
        Duration::FifteenMinutes,
    )?;

    // For deeper relaxation
    generate_binaural_beats(
        CarrierFrequency::Theta,
        BeatFrequency::Theta,
        Duration::ThirtyMinutes,
    )?;

    Ok(())
}

fn sleep() -> Result<(), Error> {
    /*
    For sleep, the goal is to use frequencies in the Delta range.

        Delta (0.5-4 Hz): This is the brainwave state associated with deep, dreamless sleep.
    */
    generate_binaural_beats(
        CarrierFrequency::Delta,
        BeatFrequency::Delta,
        Duration::SixtyMinutes,
    )?;
    Ok(())
}

fn focus_intelligence() -> Result<(), Error> {
    /*
    For focus and intelligence, you'll use frequencies in the Beta and Gamma ranges.

        Beta (12-30 Hz): This is the normal waking state, associated with alertness, concentration, and problem-solving.
        Gamma (30-100 Hz): Gamma waves are the fastest brainwaves and are linked to high-level cognitive function, learning, and intelligence.
    */

    // For normal focus and concentration
    generate_binaural_beats(
        CarrierFrequency::Beta,
        BeatFrequency::Beta,
        Duration::ThirtyMinutes,
    )?;

    // For high-level cognitive function
    generate_binaural_beats(
        CarrierFrequency::Gamma,
        BeatFrequency::Gamma,
        Duration::TenMinutes,
    )?;

    Ok(())
}

fn ephoria() -> Result<(), Error> {
    // For Euphoria (using a 40 Hz beat frequency)
    // Euphoria: This state is often linked to the release of endorphins and can be achieved with frequencies in the Gamma or high Alpha ranges. Some sources also suggest a beat frequency of 40 Hz for this state.

    generate_binaural_beats(
        CarrierFrequency::Custom(300.0), // Use a suitable carrier frequency
        BeatFrequency::Custom(40.0),
        Duration::TenMinutes,
    )?;

    Ok(())
}

fn intuition() -> Result<(), Error> {
    // For Intuition (using a Theta beat)
    //   Intuition: This is typically associated with Theta waves, the state between being awake and asleep, and Gamma waves, which are thought to integrate different parts of the brain. A combination of the two could be used.

    generate_binaural_beats(
        CarrierFrequency::Theta,
        BeatFrequency::Theta,
        Duration::FifteenMinutes,
    )?;

    Ok(())
}

fn healing() -> Result<(), Error> {
     
    // For Healing (using a Delta beat)
    //Healing: Many practitioners link healing states to a combination of Delta and Theta waves, as these are the states your brain is in during deep restorative sleep.
    generate_binaural_beats(
        CarrierFrequency::Delta,
        BeatFrequency::Delta,
        Duration::SixtyMinutes,
    )?;

    Ok(())
}


fn astral() -> Result<(), Error> {
     
    //Astral has a Carrier frequency of 140.0 and a beat frequency of 6.3
    generate_binaural_beats(
        CarrierFrequency::Custom(140.0),
        BeatFrequency::Custom(6.3),
        Duration::SixtyMinutes,
    )?;

    generate_binaural_beats(
        CarrierFrequency::Custom(140.0),
        BeatFrequency::Custom(6.3),
        Duration::SixtyMinutes,
    )?;

    generate_binaural_beats(
        CarrierFrequency::SolfeggioRoot,
        BeatFrequency::Custom(6.3),
        Duration::SixtyMinutes,
    )?;

    Ok(())
}

fn main() -> Result<(), Error> {
     astral()
}