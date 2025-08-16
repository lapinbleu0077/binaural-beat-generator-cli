extern crate cpal;
use std::io;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use anyhow::Error;
use binaural_beat_generator::generate_binaural_beats;
use binaural_beat_generator::{BinauralPreset, Preset};
use chrono::Local;

// --- Main function to demonstrate usage ---

fn main() -> Result<(), Error> {
    // 1. Create the cancellation token
    let cancel_token = Arc::new(AtomicBool::new(false));
    let cancel_token_clone = Arc::clone(&cancel_token);

    // 2. Start a separate thread to listen for user input
    std::thread::spawn(move || {
        println!("Press Enter to stop playback.");
        let _ = io::stdin().read_line(&mut String::new());
        cancel_token_clone.store(true, Ordering::Relaxed);
    });

      // Call the function using a single Preset enum variant.
    let preset_type = Preset::Healing;
    let preset: BinauralPreset = preset_type.into();
    println!("\n--- Playing a pre-defined preset: {} ---",preset_type);

    let local = Local::now();  
    println!("\n--- Started the program at : {} ---",local);

    generate_binaural_beats(preset.carrier, preset.beat, preset.duration, Arc::clone(&cancel_token))?;

    /*
    // You can still manually create presets if you want
    let custom_preset = BinauralPreset {
        carrier: CarrierFrequency::Custom(250.0),
        beat: BeatFrequency::Custom(6.0),
        duration: Duration::FiveMinutes,
    };
    println!("\n--- Playing a custom preset ---");
    generate_binaural_beats(custom_preset.carrier, custom_preset.beat, custom_preset.duration)?;
    */

    Ok(())
}