extern crate cpal;
use std::io;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use anyhow::Error;
use binaural_beat_generator::{generate_binaural_beats, Duration, ToMinutes};
use binaural_beat_generator::{BinauralPreset, Preset};

use inquire::Select;

// --- Main function to demonstrate usage ---

fn main() -> Result<(), Error> {
    let preset_options = vec![Preset::Focus,Preset::HighFocus,Preset::Relaxation,Preset::DeepRelaxation,Preset::Sleep,Preset::Chanting
                ,Preset::Intuition,Preset::Astral,Preset::Healing,Preset::Alpha,Preset::Intelligence,Preset::Euphoria
                ,Preset::CrownFocus,Preset::CrownRelaxation,Preset::CrownSleep,Preset::CrownChanting,Preset::CrownIntuition
                ,Preset::CrownAstral,Preset::SolfeggioRoot,Preset::SolfeggioSacral,Preset::SolfeggioSolarPlexus,Preset::SolfeggioHeart
                ,Preset::SolfeggioThroat,Preset::SolfeggioThirdEye,Preset::SolfeggioCrown,Preset::TuningForkRoot,Preset::TuningForkSacral
                ,Preset::TuningForkSolarPlexus,Preset::TuningForkHeart,Preset::TuningForkThroat,Preset::TuningForkThirdEye,Preset::TuningForkCrown];

    let duration_options: Vec<u32> = vec![5, 10, 15, 20, 30, 35, 40, 50, 60];

    let chosen_preset = Select::new("Choose a preset: ", preset_options).prompt();

    match chosen_preset {
        Ok(choice) => {
            let binaural_preset = BinauralPreset::from(choice);
            println!("You chose the preset : {} and it has duration of {}", choice, binaural_preset.duration.to_minutes());

            let starting_duration_index = duration_options
                                                .iter()
                                                .position( |&x| x == binaural_preset.duration.to_minutes())
                                                .unwrap_or(15);

            
            let chosen_duration = Select::new("Choose a duration: ", duration_options)
                                                            .with_starting_cursor(starting_duration_index)
                                                            .prompt();

            match chosen_duration {
                Ok(duration) => {
                    run_binaural_beat(binaural_preset, Duration::Custom(duration))?;
                },
                Err(_) => println!("There was an error choosing the duration, please try again."),
            }
        },
        Err(_) => println!("There was an error, please try again."),
    }

    Ok(())
}

fn run_binaural_beat(preset_type : BinauralPreset, duration : Duration) -> Result<(), Error> {
    let cancel_token = Arc::new(AtomicBool::new(false));
    let cancel_token_clone = Arc::clone(&cancel_token);

    // 2. Start a separate thread to listen for user input
    std::thread::spawn(move || {
        println!("Press Enter to stop playback.");
        let _ = io::stdin().read_line(&mut String::new());
        cancel_token_clone.store(true, Ordering::Relaxed);
    });

    generate_binaural_beats(preset_type.carrier, preset_type.beat, duration, Arc::clone(&cancel_token))?;

    Ok(())
}