use crate::frame::{Drawable, Frame};
use crate::{NUM_COLS, NUM_ROWS};
use rusty_time::timer::Timer;
use std::{cmp::max, time::Duration};

// create SINGLE Invader struct, fields for position
pub struct Invader {
    pub x: usize,
    pub y: usize,
}

// create ARMY of Invaders struct, vector of Invader so they all have
// the same structure and movement
pub struct Invaders {
    pub army: Vec<Invader>,
    move_timer: Timer,
    direction: i32,
}

// public functions to implement on Invader Army
impl Invaders {
    // create new Invader
    pub fn new() -> Self {
        let mut army = Vec::new();
        // placement of invaders on our grid using range of NUM_COLS and NUM_ROWS
        // for every x in all possible x's on playfield
        for x in 0..NUM_COLS {
            // and for every y in all rows on playfield
            for y in 0..NUM_ROWS {
                // if x is greater than 1, and all following conditions, push x and y position value to army
                if (x > 1)
                    && (x < NUM_COLS - 2) // -2 to keep off edge of playfield
                    && (y > 0)
                    && (y < 9) // stop spot halfway down screen
                    && (x % 2 == 0) // set to every even number for space
                    && (y % 2 == 0)
                {
                    army.push(Invader { x, y });
                }
            }
        }
        // return self
        Self {
            army,
            move_timer: Timer::from_millis(2000),
            // direction: positive number (1) moves army right
            direction: 1,
        }
    }

    // return boolean indicating whether army has moved, bind to sound
    pub fn update(&mut self, delta: Duration) -> bool {
        self.move_timer.update(delta);
        if self.move_timer.ready {
            // reset move timer
            self.move_timer.reset();
            // set downwards start to false
            let mut downwards = false;

            // if moving left, immutable interate through army and map through each
            // invader to their x value, take min x value return or 0 if none found
            if self.direction == -1 {
                let min_x = self.army.iter().map(|invader| invader.x).min().unwrap_or(0);
                // if reached left side of screen (min_x = 0), change direction to
                // right and move downwards
                if min_x == 0 {
                    self.direction = 1;
                    downwards = true;
                }
            } else {
                // if reached right side of screen (max_x = NUM_COLS - 1), change direction to
                // left and move downwards
                let max_x = self.army.iter().map(|invader| invader.x).max().unwrap_or(0);
                if max_x == NUM_COLS - 1 {
                    self.direction = -1;
                    downwards = true;
                }
            }

            //  if downwards is true, increase speed by creating a new duration and setting movement timer to smaller value
            if downwards {
                // new_duration set to max of our current move timer - 250ms, set to 250 min
                let new_duration = max(self.move_timer.duration.as_millis() - 250, 250);
                self.move_timer = Timer::from_millis(new_duration as u64);
                // loop through every invader in army, set y value to +1
                for invader in self.army.iter_mut() {
                    invader.y += 1;
                }
            } else {
                // if not moving downwards, move left or right
                // loop through every invader in Army, change x value
                for invader in self.army.iter_mut() {
                    // set x as i32 and add to self.direction (which is an i32), cast back as usize
                    invader.x = ((invader.x as i32) + self.direction) as usize;
                }
            }
            return true;
        }
        false
    }

    // WINNING AND LOSING CONDITIONS WITH INVADERS EITHER BEING ELIMIATED (win) OR REACHING BOTTOM (lose)
    pub fn all_killed(&self) -> bool {
        self.army.is_empty()
    }

    pub fn reached_bottom(&self) -> bool {
        self.army.iter().map(|invader| invader.y).max().unwrap_or(0) >= NUM_ROWS - 1
    }

    pub fn kill_invader_at(&mut self, x: usize, y: usize) -> bool {
        // get index of invaders position on screen
        if let Some(idx) = self
            .army
            .iter()
            .position(|invader| (invader.x == x) && (invader.y == y))
        {
            self.army.remove(idx);
            true
        } else {
            false
        }
    }
}

impl Drawable for Invaders {
    // draw each invader in army, draw either an x or + half the time on screen using the move_timer (fake animation)
    fn draw(&self, frame: &mut Frame) {
        for invader in self.army.iter() {
            // calculate what half of timer we are in: divide time left by total duration
            // if less than 0.5 display x, otherwise +
            frame[invader.x][invader.y] = if (self.move_timer.time_left.as_secs_f32()
                / self.move_timer.duration.as_secs_f32())
                > 0.5
            {
                "x"
            } else {
                "+"
            };
        }
    }
}
