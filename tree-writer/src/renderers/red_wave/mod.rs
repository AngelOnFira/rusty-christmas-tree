use super::{Pixel, TreeCanvas};

// Name: Red Wave
// Description: A basic red wave across the screen
// Author: Forest Anderson

pub fn draw(tick: u64) -> TreeCanvas {
    let mut canvas = TreeCanvas::new();

    for y in 0..75 {
        for x in 0..20 {
            let this_pixel = Pixel {
                r: ((((tick + y as u64) as f32 * 0.1).sin() + 0.3) * 255.0) as u8,
                g: 0,
                b: 0,
            };
            canvas.set_pixel(x, y, this_pixel)
        }
    }

    canvas
}
