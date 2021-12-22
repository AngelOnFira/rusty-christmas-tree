use self::tree_canvas::{Pixel, TreeCanvas};

pub mod red_wave;
pub mod template;

pub mod tree_canvas;

pub fn build_array(tick: u64) -> TreeCanvas {
    red_wave::draw(tick)
}
