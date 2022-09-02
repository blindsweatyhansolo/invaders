use crate::{NUM_COLS, NUM_ROWS, frame::{Drawable, Frame}};

// create Player struct, fields for its position (x , y)
pub struct Player {
    x: usize,
    y: usize,
}

// public functions to implement on Player
impl Player {
    // create a new Player
    pub fn new() -> Self {
        Self {
            x: NUM_COLS / 2, // roughly half way point
            y: NUM_ROWS - 1, // last playable row 
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

}

// draw the player with draw()
// renders an "A" as the ship
impl Drawable for Player {
    fn draw(&self, frame: &mut Frame) {
        frame[self.x][self.y] = "A";
    }
}