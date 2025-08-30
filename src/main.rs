extern crate cpal;
use colored::Colorize;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use std::cmp;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use anyhow::Error;
use inquire::Select;

use crate::modules::bb_generator::generate_binaural_beats;
use crate::modules::duration::duration::{duration_list};
use crate::modules::preset::{BinauralPresetGroup, preset_list};

mod modules;

// --- Main function to demonstrate usage ---

fn main() -> Result<(), Error> {
    let preset_options = preset_list();
    let duration_options = duration_list();
    let preset_length = preset_options.len();

    print_program_info("Binaural Beat Generator","By Chris Horton");

    let chosen_preset = Select::new("Choose a preset: ", preset_options)
        .with_page_size(preset_length)
        .prompt();

    match chosen_preset {
        Ok(preset) => {
            let mut binaural_preset_options = BinauralPresetGroup::from(preset);

            let starting_duration_index = duration_options
                .iter()
                .position(|&x| x == binaural_preset_options.duration)
                .unwrap();

            let chosen_duration = Select::new("Choose a duration: ", duration_options)
                .with_starting_cursor(starting_duration_index)
                .prompt();

            match chosen_duration {
                Ok(duration) => {
                    //Get the chosen duration if it has changed.
                    binaural_preset_options.duration = duration;
                    run_binaural_beat(binaural_preset_options)?;
                }
                Err(err) => eprintln!("There was an error choosing the duration, please try again. {}", err),
            }
        }
        Err(err) => eprintln!("There was an error, please try again. {}", err),
    }

    Ok(())
}

fn run_binaural_beat(preset_options: BinauralPresetGroup) -> Result<(), Error> {
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
                Err(err) => eprintln!("There was an error, please try again. {}", err),
            }
        }
    });

    generate_binaural_beats(
        preset_options,
        Arc::clone(&cancel_token),
    )?;

    Ok(())
}

fn print_program_info(title : &str, author_text : &str) {
    let max_chars = cmp::max(title.len(), author_text.len());
    let separator = "=".repeat(max_chars + 10);
    println!("{}",separator.blue());
    println!("   {}   ", title.red().bold());
    println!("   {}   ", author_text.red().bold().italic());
    println!("{}",separator.blue());
}