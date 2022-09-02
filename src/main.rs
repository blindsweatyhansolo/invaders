use std::error::Error;
use std::io;
use std::time::Duration;
use crossterm::event::{KeyCode, Event};
use crossterm::{terminal, ExecutableCommand, event};
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::cursor::{Hide, Show};
use rusty_audio::Audio;

fn main() -> Result <(), Box<dyn Error>> {
    // create mutable audio variable using rusty_audio crate Audio
    let mut audio = Audio::new();

    // add all audio sources to audio manager
    // (name of file, path of audio file)
    audio.add("explode", "explode.wav");
    audio.add("lose", "lose.wav");
    audio.add("move", "move.wav");
    audio.add("pew", "pew.wav");
    audio.add("startup", "startup.wav");
    audio.add("win", "win.wav");

    audio.play("startup");

    // TERMINAL
    let mut stdout = io::stdout();
    // [crossterm] enable raw mode to capture keyboard input as it occurs
    terminal::enable_raw_mode()?;
    // [crossterm] enter alternate screen
    // execute() immediately executes something
    stdout.execute(EnterAlternateScreen)?;
    // [crossterm] hide cursor when entering alternate screen
    stdout.execute(Hide)?;

    // MAIN GAME LOOP
    'gameloop: loop {
        // INPUT HANDLING
        // poll for input events, duration set to default (immediate)
        while event::poll(Duration::default())? {
            // if key event (code), read and match on input
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    // inputing 'q' or hitting 'esc'
                    KeyCode::Esc | KeyCode::Char('q') => {
                        // play lose sound and exit loop
                        audio.play("lose");
                        break 'gameloop;
                    }
                    _ => {}
                }
            }
        }
    }

    // CLEANUP
    // block until all audio is done playing with wait()
    audio.wait();
    // show cursor, leave alternate screen, and disable raw mode
    stdout.execute(Show)?;
    stdout.execute(LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    Ok(())

}
   
