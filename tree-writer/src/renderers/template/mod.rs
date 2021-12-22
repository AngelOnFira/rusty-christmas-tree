use super::{Pixel, TreeCanvas};

// Name: <Name for this renderer>
// Description: <Something about this renderer>
// Author: <Your name>

pub fn draw(tick: u64) -> TreeCanvas {
    let mut canvas = TreeCanvas::new();

    for y in 0..75 {
        for x in 0..20 {
            let this_pixel = Pixel {
                r: x * y,
                g: x * y,
                b: x * y,
            };
            canvas.set_pixel(x as usize, y as usize, this_pixel)
        }
    }

    canvas
}
