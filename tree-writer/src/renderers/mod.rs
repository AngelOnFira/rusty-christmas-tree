use tree_data_schema::Renderers;

use self::tree_canvas::{Pixel, TreeCanvas};

// Add a new renderer as a module here with the name you gave it
pub mod red_wave;
pub mod template;
pub mod snow;
pub mod ender_logo;
pub mod space_fight;

pub mod tree_canvas;

pub fn visualize_renderer(tick: u64, renderer: Renderers) -> TreeCanvas {
    // Add your enum variant here
    match renderer {
        Renderers::EnderLogo => ender_logo::draw(tick),
        Renderers::RedWave => red_wave::draw(tick),
        Renderers::Template => template::draw(tick),
        Renderers::Snow => snow::draw(tick),
        Renderers::SpaceFight => space_fight::draw(tick),
    }
}
