use std::time::Duration;
use rusty_time::timer::Timer;
use crate::frame::{Drawable, Frame};

// create struct for Shot
// fields for position, whether it is exploding
pub struct Shot {
    pub x: usize,
    pub y: usize,
    pub exploding: bool,
    // internal timer to keep track of movement
    timer: Timer,
}

impl Shot {
    // construct a new shot
    pub fn new(x: usize, y: usize) -> Self {
        Self {
            x,
            y,
            exploding: false,
            // moves up one cell every 50ms
            timer: Timer::from_millis(50),
        }
    }

    // update Shot timer
    pub fn update(&mut self, delta: Duration) {
        self.timer.update(delta);
        // if timer is ready and shot is not exploding
        if self.timer.ready && !self.exploding {
            // if shot hasnt reached top of screen (0), move up
            if self.y > 0 {
                self.y -= 1;
            }
            // reset timer
            self.timer.reset();
        }
    }

    // explode shot
    pub fn explode(&mut self) {
        // set exploding to true, set new timer to 250ms
        self.exploding = true;
        self.timer = Timer::from_millis(250);
    }

    // is the shot 'dead'
    pub fn dead(&self) -> bool {
        // if exploded and timer has run out OR shot has reached top
        (self.exploding && self.timer.ready ) || (self.y == 0)
    }

}

// draw shot with draw()
impl Drawable for Shot {
    fn draw(&self, frame: &mut Frame) {
        frame[self.x][self.y] = if self.exploding { "*" } else { "|" };
    }
}