use std::io::{Stdout, Write};
use crate::frame::Frame;
use crossterm::{
    QueueableCommand,
    cursor::MoveTo,
    style::{SetBackgroundColor, Color},
    terminal::{Clear, ClearType},
};

// render function takes mutable reference to Stdout, only render what changed with
// last_frame and curr_frame which are references to Frame
// but everything must be rendered atleast once, so declare a boolean variable called force
pub fn render(stdout: &mut Stdout, last_frame: &Frame, curr_frame: &Frame, force: bool) {
    // if we are force rendering everything
    if force {
        // [crossterm] clear and set screen to blue color
        stdout.queue(SetBackgroundColor(Color::Blue)).unwrap();
        // clear all with ClearType::All
        stdout.queue(Clear(ClearType::All)).unwrap();
        // set background color to black, for black background on playing field
        stdout.queue(SetBackgroundColor(Color::Black)).unwrap();
    }

    // iterate through entire frame and draw whatever has changed
    // for every 'x' index of column vectors in current frame, iterate through immutably 
    // with iter() and enumerate with enumerate()
    for (x, col) in curr_frame.iter().enumerate() {
        // repeat process for 'y' and actual string character (s)
        for (y, s) in col.iter().enumerate() {
            // derefence double reference to "s" with *s
            // compare with last frame's character in x and y 
            if *s != last_frame[x][y] || force {
                // IF character changed OR we're forcing rendering, queue up a command to
                // move to the correct location (MoveTo)
                stdout.queue(MoveTo(x as u16, y as u16)).unwrap();
                // print a single character at this location (without a line or flushing)
                print!("{}", *s);
            }
        }
    }
    // flush our all queue commands at the end of the function
    stdout.flush().unwrap();
}