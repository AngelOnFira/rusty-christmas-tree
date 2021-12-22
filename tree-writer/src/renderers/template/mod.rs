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
                r: min(254, x * y) as u8,
                g: 20 + (tick as u8 % 40) * 3,
                b: 20 + (tick as u8 % 60) * 2,
            };
            canvas.set_pixel(x, y, this_pixel)
        }
    }

    canvas
}
