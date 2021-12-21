use nannou::{color, prelude::*};

use aidan_tree::render::build_array;

fn main() {
    nannou::sketch(view).run();
}

fn view(app: &App, frame: Frame) {
    // Begin drawing
    let draw = app.draw();

    // Clear the background to blue.
    draw.background().color(BLACK);

    let render = build_array(frame.nth());
    let COLUMN_HEIGHT = 75;

    // Draw a 20x75 grid of rectangles that are each 5x5 pixels. The colour of
    // each rectangle is determined by each window of 3 values in the array. The
    // first value is the red, the second is the green, and the third is the
    // blue.

    let dim = 15.0;

    // Iterate over two halfs of the output
    for side in 0..2 {
        // Get the lenght of one half
        let len_half = render.len() / 3 / 2;

        // Iterate over half the columns
        for pixel in side * len_half..(side + 1) * len_half {
            let x = pixel / 3 / 20;

            let mut y = pixel / 3 % 20;
            // If we're on an odd column, flip the y direction
            if pixel % 2 == 0 {
                y = COLUMN_HEIGHT - y;
            }

            draw.rect()
                .x_y(x as f32 * dim - 200.0, y as f32 * dim - 200.0)
                .w_h(dim as f32, dim as f32)
                .color(color::rgb(
                    render[pixel * 3 + 0],
                    render[pixel * 3 + 1],
                    render[pixel * 3 + 2],
                ));
        }
    }

    // Write the result of our drawing to the window's frame.
    draw.to_frame(app, &frame).unwrap();
}
