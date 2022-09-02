use std::error::Error;
use std::thread;
use std::io;
use std::sync::mpsc;
use std::time::Duration;
use crossterm::event::{KeyCode, Event};
use crossterm::{terminal, ExecutableCommand, event};
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::cursor::{Hide, Show};
use invaders::frame;
use invaders::frame::Drawable;
use invaders::frame::new_frame;
use invaders::player::Player;
use invaders::render;
use rusty_audio::Audio;

fn main() -> Result <(), Box<dyn Error>> {
    // create mutable audio variable using rusty_audio crate Audio
    let mut audio = Audio::new();

    // add all audio sources to audio manager
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

    // RENDER LOOP in a separate thread (for speed optimization)
    // channel to communicate with threads
    // (render transciever and render receiver) using mspc channels 
    let (render_tx, render_rx) = mpsc::channel();
    
    // THREAD: recieve frames and render them
    // catch thread handle (render_handle) with closure
    // move || closure captures end of channel
    let render_handle = thread::spawn(move || {
        // variable to hold last frame (new empty frame)
        let mut last_frame = frame::new_frame();
        // render to standard output with new access to stdout
        let mut stdout = io::stdout();
        // render entire screen once with force rendering
        render::render(&mut stdout, &last_frame, &last_frame, true);

        loop {
            // set curr_frame to result of match, either valid Frame (x) or break loop on error
            // on error (shuts down child thread)
            let curr_frame = match render_rx.recv() {
                Ok(x) => x,
                Err(_) => break,
            };
            // render frame with references to last frame, current frame and force set to false
            render::render(&mut stdout, &last_frame, &curr_frame, false);
            // housekeeping: last frame is now current frame to set up for next loop
            last_frame = curr_frame;
        } 
    });

    // MAIN GAME LOOP
    // set new player
    let mut player = Player::new();

    'gameloop: loop {
        // Per-frame initialization
        let mut curr_frame = new_frame();

        // INPUT HANDLING
        // poll for input events, duration set to default (immediate)
        while event::poll(Duration::default())? {
            // if key event (code), read and match on input
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    // handle left and right keys
                    KeyCode::Left => player.move_left(),
                    KeyCode::Right => player.move_right(),
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

        // DRAW & RENDER
        // draw player with current frame position
        player.draw(&mut curr_frame);
        
        // send frame, .send moves to a different thread
        // expects to fail first few times because game loop starts before child thread
        // starts recieving. ignore it silently let _ (wildcard)
        let _ = render_tx.send(curr_frame);

        // game loop is faster than render loop
        // artificial sleep adds limit to no more than generating 1000 fps to avoid
        // falling behind when rendering
        thread::sleep(Duration::from_millis(1));

    }

    // CLEANUP
    // drop transmitting side of the channel, causes recieving channel to error and end loop 
    // becoming safe to join threads
    drop(render_tx);
    render_handle.join().unwrap();

    // block until all audio is done playing with wait()
    audio.wait();
    // show cursor, leave alternate screen, and disable raw mode
    stdout.execute(Show)?;
    stdout.execute(LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    Ok(())

}
   
