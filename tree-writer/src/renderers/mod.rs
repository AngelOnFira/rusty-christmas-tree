use tree_data_schema::Renderers;

use self::tree_canvas::{Pixel, TreeCanvas};

pub mod red_wave;
pub mod template;

pub mod tree_canvas;

pub fn visualize_renderer(tick: u64, renderer: Renderers) -> TreeCanvas {
    match renderer {
        Renderers::RedWave => red_wave::draw(tick),
        Renderers::Template => template::draw(tick),
    }
}
