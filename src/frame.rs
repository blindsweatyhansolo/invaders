use crate::{NUM_COLS, NUM_ROWS};
// FRAME MODULE

// TYPE ALIAS - Vector of vectors of borrowed static string slices :O
pub type Frame = Vec<Vec<&'static str>>;

// public module function to generate new frame, return Frame
pub fn new_frame() -> Frame {
    // OUTER VECTOR //
    // cols = vector with a capacity set to NUM_COLS value
    // gets generated every frame
    let mut cols = Vec::with_capacity(NUM_COLS);

    // loop through every number of times that we have NUM_COLS, and
    // generate a column
    for _ in 0..NUM_COLS {
        // individual column with a capacity set to NUM_ROWS value
        let mut col = Vec::with_capacity(NUM_ROWS);
        // loop through and add single row character for each row
        for _ in 0..NUM_ROWS {
            // create new frame (blank space)
            col.push(" ");
        }
        // push col to cols variable
        cols.push(col);
    }
    // return cols: vector that is a column of columns
    cols
}

// to be Drawable, implement draw method
pub trait Drawable {
    // take immutable reference to self and mutable reference to Frame,
    // and draw yourself into the frame
    fn draw(&self, frame: &mut Frame);
}
