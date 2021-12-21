use nannou::{color, prelude::*};

use tree_writer::render::build_array;
use tree_writer::FRAME_RATE;

fn main() {
    // let loop_mode = LoopMode::rate_fps(FRAME_RATE as usize);
    // nannou::app(model).update(update).loop_mode(loop_mode).run();

    nannou::app(model).update(update).run();
}

struct Model {
    _window: window::Id,
}

fn model(app: &App) -> Model {
    let _window = app.new_window().view(view).build().unwrap();
    Model { _window }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
    // Sleep for the FRAME_RATE
    std::thread::sleep(std::time::Duration::from_millis(1000 / FRAME_RATE));
}

fn view(app: &App, _model: &Model, frame: Frame) {
    // Begin drawing
    let draw = app.draw();

    // Clear the background to blue.
    draw.background().color(BLACK);

    let render = build_array(frame.nth());
    let column_height = 75;

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
                y = column_height - y;
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
