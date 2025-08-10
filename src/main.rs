extern crate cpal;
use anyhow::Error;
use binaural_beat_generator::generate_binaural_beats;
use binaural_beat_generator::{BinauralPreset, Preset};

// --- Main function to demonstrate usage ---

fn main() -> Result<(), Error> {
      // Call the function using a single Preset enum variant.
    let preset_type = Preset::Relaxation;
    let preset: BinauralPreset = preset_type.into();
    println!("\n--- Playing a pre-defined preset: {} ---",preset_type);
    generate_binaural_beats(preset.carrier, preset.beat, preset.duration)?;

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