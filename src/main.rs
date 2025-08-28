extern crate cpal;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use anyhow::Error;
use binaural_beat_generator_cli::{BinauralPreset, duration_list, preset_list};
use binaural_beat_generator_cli::{Duration, ToMinutes, generate_binaural_beats};

use inquire::Select;

// --- Main function to demonstrate usage ---

fn main() -> Result<(), Error> {
    let preset_options = preset_list();
    let duration_options = duration_list();
    let preset_length = preset_options.len();
    let chosen_preset = Select::new("Choose a preset: ", preset_options)
        .with_page_size(preset_length)
        .prompt();

    match chosen_preset {
        Ok(choice) => {
            let binaural_preset = BinauralPreset::from(choice);
            println!(
                "You chose the preset : {} and it has duration of {}",
                choice,
                binaural_preset.duration.to_minutes()
            );

            let starting_duration_index = duration_options
                .iter()
                .position(|&x| x == binaural_preset.duration)
                .unwrap();

            let chosen_duration = Select::new("Choose a duration: ", duration_options)
                .with_starting_cursor(starting_duration_index)
                .prompt();

            match chosen_duration {
                Ok(duration) => {
                    run_binaural_beat(binaural_preset, duration)?;
                }
                Err(_) => eprintln!("There was an error choosing the duration, please try again."),
            }
        }
        Err(_) => eprintln!("There was an error, please try again."),
    }

    Ok(())
}

fn run_binaural_beat(preset_type: BinauralPreset, duration: Duration) -> Result<(), Error> {
    let cancel_token = Arc::new(AtomicBool::new(false));
    let cancel_token_clone = Arc::clone(&cancel_token);

    // 2. Start a separate thread to listen for user input
    std::thread::spawn(move || {
        println!("Press Enter to stop playback.");

        loop {
            match event::read() {
                Ok(Event::Key(key_event)) => {
                    if key_event.kind == KeyEventKind::Press && key_event.code == KeyCode::Enter {
                        cancel_token_clone.store(true, Ordering::Relaxed);
                    }
                }
                Ok(_) => {} // Ignore other events
                Err(e) => eprintln!("There was an error, please try again. {}", e),
            }
        }
    });

    generate_binaural_beats(
        preset_type.carrier,
        preset_type.beat,
        duration,
        Arc::clone(&cancel_token),
    )?;

    Ok(())
}
