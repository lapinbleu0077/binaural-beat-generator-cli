//! A module that contains the bulk of the code that allows the program to run.

use anyhow::Error;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration as StdDuration, Instant}; // Alias to avoid conflict with enum variant

//Cancellation support
use std::sync::atomic::{AtomicBool, Ordering};

use crate::modules::duration::duration_common::ToMinutes;
use crate::modules::frequency::frequency_common::ToFrequency;
use crate::modules::preset::{BinauralPresetGroup};

/// A function that wats for the chosen time limit to end before exiting. 
/// The function will constantly check if the user wants to stop running of the program.
/// 
fn wait_until_end(cancel_token: Arc<AtomicBool>, duration_minutes: u32) {
    let total_duration = StdDuration::from_secs((duration_minutes * 60) as u64);
    let start_time = Instant::now();

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

/// Generates and plays binaural beat tones based on specified carrier frequency,
/// beat frequency, and duration.
///
/// # Arguments
/// - `preset_options`: Specifies the binaural beat options choosen by the user to execute.
/// - `cancel_token`: An atomic instance of a boolean that controls the stopping of the program before the timelimit.
///
/// # Returns
/// `Result<(), anyhow::Error>` indicating success or failure.
pub fn generate_binaural_beats(
    preset_options : BinauralPresetGroup,
    cancel_token: Arc<AtomicBool>,
) -> Result<(), Error>
{
    // Extract concrete values from generic parameters
    let carrier_hz = preset_options.carrier.to_hz();
    let beat_hz = preset_options.beat.to_hz();
    let duration_minutes = preset_options.duration.to_minutes();

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
    println!("Preset {}", preset_options.preset);
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
