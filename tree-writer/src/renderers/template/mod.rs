use std::cmp::min;

use super::{Pixel, TreeCanvas};

// Name: <Name for this renderer>
// Description: <Something about this renderer>
// Author: <Your name>

pub fn draw(tick: u64) -> TreeCanvas {
    let mut canvas = TreeCanvas::new();

    for y in 0..75 {
        for x in 0..20 {
            let this_pixel = Pixel {
                r: (
                    (tick as f64) // Start by converting the tick to a 64 bit float
                    .sin() // The sin will be between -1 and 1
                    .abs() // Get the absolute value so we are between 0 and 1
                    * 150.0 // Multiply by 150 to get a number between 0 and 150
                    + 100.0
                    // ^^ Add 100 to get a number between 100 and 250
                ) as u8, // Convert the float to an 8 bit integer
                g: 0,
                b: 0,
            };
            canvas.set_pixel(x, y, this_pixel)
        }
    }

    canvas
}
