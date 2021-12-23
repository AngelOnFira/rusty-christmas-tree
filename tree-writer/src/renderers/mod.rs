use tree_data_schema::Renderers;

use self::tree_canvas::{Pixel, TreeCanvas};

// Add a new renderer as a module here with the name you gave it
pub mod red_wave;
pub mod template;
pub mod snow;

pub mod tree_canvas;

pub fn visualize_renderer(tick: u64, renderer: Renderers) -> TreeCanvas {
    match renderer {
        Renderers::RedWave => red_wave::draw(tick),
        Renderers::Template => template::draw(tick),
        Renderers::Snow => snow::draw(tick),
    }
}
