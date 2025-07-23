extern crate cpal;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use anyhow::Error;
use std::sync::{Arc, Mutex};

fn main() -> Result<(), Error> {
    let host = cpal::default_host();
    
    let device = host.default_output_device()
        .ok_or_else(|| anyhow::anyhow!("No output device available."))?;
    
    let config = device.default_output_config()?;

    let sample_rate = config.sample_rate().0 as f32;
    let f_left = 440.0;
    let f_right = 447.0;

    let sample_clock_left = Arc::new(Mutex::new(0f32));
    let sample_clock_right = Arc::new(Mutex::new(0f32));

    let sample_clock_left_for_closure = Arc::clone(&sample_clock_left);
    let sample_clock_right_for_closure = Arc::clone(&sample_clock_right);

    // FIX: Clone the config here
    let config_for_stream = config.clone(); // <--- Clone the config for the stream builder

    let stream = device.build_output_stream(
        &config.into(), // This consumes 'config'
        move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
            let mut current_sample_clock_left = sample_clock_left_for_closure.lock().unwrap();
            let mut current_sample_clock_right = sample_clock_right_for_closure.lock().unwrap();

            // Use config_for_stream or a captured part of it within the closure
            for frame in data.chunks_mut(config_for_stream.channels() as usize) { // <--- Use config_for_stream
                let left_sample = (2.0 * std::f32::consts::PI * f_left * *current_sample_clock_left / sample_rate).sin();
                *current_sample_clock_left += 1.0;

                let right_sample = (2.0 * std::f32::consts::PI * f_right * *current_sample_clock_right / sample_rate).sin();
                *current_sample_clock_right += 1.0;

                if config_for_stream.channels() == 2 { // <--- Use config_for_stream
                    frame[0] = left_sample * 0.5;
                    frame[1] = right_sample * 0.5;
                } else {
                    frame[0] = (left_sample + right_sample) * 0.25;
                }
            }
        },
        |err| eprintln!("An error occurred on stream: {}", err),
        None
    )?;

    stream.play()?;

    println!("Generating binaural beats for 10 seconds...");
    std::thread::sleep(std::time::Duration::from_secs(10));
    println!("Done.");

    Ok(())
}