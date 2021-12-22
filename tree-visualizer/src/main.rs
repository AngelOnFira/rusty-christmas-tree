use nannou::{color, prelude::*};

use tree_data_schema::FRAME_RATE;
use tree_writer::render::build_array;

fn main() {
    // let loop_mode = LoopMode::rate_fps(FRAME_RATE as usize);
    // nannou::app(model).update(update).loop_mode(loop_mode).run();

    nannou::app(model).update(update).run();
}

struct Model {
    window: window::Id,
    dim: f32,
}

fn model(app: &App) -> Model {
    let window = app.new_window().view(view).build().unwrap();
    Model { window, dim: 5.0 }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
    // Sleep for the FRAME_RATE
    std::thread::sleep(std::time::Duration::from_millis(1000 / FRAME_RATE));
}

fn view(app: &App, model: &Model, frame: Frame) {
    // Begin drawing
    let draw = app.draw();

    // Clear the background to black.
    draw.background().color(BLACK);

    let render = build_array(frame.nth());
    let column_height = 75;

    // Draw a 20x75 grid of rectangles that are each 5x5 pixels. The colour of
    // each rectangle is determined by each window of 3 values in the array. The
    // first value is the red, the second is the green, and the third is the
    // blue.

    // Iterate over half the columns
    for pixel in 0..render.len() / 3 {
        let x = pixel / 75;

        let mut y = pixel % 75;
        // If we're on an odd column, flip the y direction
        if x % 2 == 0 {
            y = column_height - y;
        }

        // TODO: Make the offsets based on the window size
        draw.rect()
            .x_y(
                x as f32 * model.dim * 3.0 - 200.0,
                y as f32 * model.dim * 1.5 - 300.0,
            )
            .w_h(model.dim as f32, model.dim as f32)
            .color(color::rgb(
                render[pixel * 3 + 0],
                render[pixel * 3 + 1],
                render[pixel * 3 + 2],
            ));
    }

    // Write the result of our drawing to the window's frame.
    draw.to_frame(app, &frame).unwrap();
}
