use std::cmp::min;

use super::{Pixel, TreeCanvas};

use serde::Deserialize;
use serde_json;

use cached::proc_macro::cached;

// Name: Sell a bee
// Description: Celebi's got some new moves in this render!
// Author: Liam Henderson
#[cached(size=1)]
fn get_video(n: u8) -> Vec<Vec<Vec<Pixel>>> {
    let video = include_str!("video.json");
    serde_json::from_str(&video).unwrap()
}

pub fn draw(tick: u64) -> TreeCanvas {
    let mut canvas = TreeCanvas::new();

    
    let frame_num : usize = (tick % 250).try_into().unwrap();

    let parsed: Vec<Vec<Vec<Pixel>>> = get_video(0);

    let frame: Vec<Vec<Pixel>> = parsed[frame_num].clone();

    for y in 0..75 {
        for x in 0..20 {
            let pixel: Pixel = frame[y][x];
            canvas.set_pixel(x, 74-y, pixel)
        }
    }

    canvas
}
