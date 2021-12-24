use std::cmp::min;
use std::fs;

use super::{Pixel, TreeCanvas};
use serde::Deserialize;
use serde_json;

// Name: Red Wave
// Description: A basic red wave across the screen
// Author: Forest Anderson

pub fn draw(tick: u64) -> TreeCanvas {
    let mut canvas = TreeCanvas::new();

    let pixel_json = include_str!("jwst.json");

    let pixel_data: Vec<Pixel> = serde_json::from_str(&pixel_json).unwrap();

    let mut index = 0;
    for y in 0..75 {
        for x in 0..20 {
            let this_pixel = pixel_data.get(index).unwrap();
            canvas.set_pixel(x, y, *this_pixel);
            index += 1;
        }
    }

    canvas
} 