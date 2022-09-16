use std::time::Duration;
use crate::frame::{ Frame, Drawable };
use crate::{ NUM_COLS, NUM_ROWS };
use crate::shot::Shot;

// create Player struct, fields for its position (x , y)
pub struct Player {
    x: usize,
    y: usize,
    shots: Vec<Shot>,
}

// public functions to implement on Player
impl Player {
    // create a new Player
    pub fn new() -> Self {
        Self {
            x: NUM_COLS / 2, // roughly half way point
            y: NUM_ROWS - 1, // last playable row 
            shots: Vec::new(),
        }
    }

    // move left
    pub fn move_left(&mut self) {
        if self.x > 0 {
            self.x -= 1;
        }
    }

    // move right
    pub fn move_right(&mut self) {
        if self.x < NUM_COLS - 1 {
            self.x += 1;
        }
    }

    // shoot laser (results boolean for if shot was successful)
    pub fn shoot(&mut self) -> bool {
        // set to only 2 shots at a time
        if self.shots.len() < 2 {
            // push new coordinates (one cell above player position)
            self.shots.push(Shot::new(self.x, self.y -1));
            true
        } else {
            false
        }
    }

    // update shot timer
    pub fn update(&mut self, delta: Duration) {
        // mutably interate through shots and update with delta
        for shot in self.shots.iter_mut() {
            shot.update(delta);
        }

        // cleanup with closure
        // retain only shots that are not dead
        self.shots.retain(|shot| !shot.dead());
    }

}

impl Drawable for Player {
    // draw the player and shots with draw()
    // renders an "A" as the ship
    fn draw(&self, frame: &mut Frame) {
        frame[self.x][self.y] = "A";

        for shot in self.shots.iter() {
            shot.draw(frame);
        }
    }

}