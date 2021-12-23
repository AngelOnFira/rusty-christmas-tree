use std::cmp::min;
use std::fs;

use super::{Pixel, TreeCanvas};
use serde::Deserialize;
use serde_json;

// Name: <Name for this renderer>
// Description: <Something about this renderer>
// Author: <Your name>

pub fn draw(tick: u64) -> TreeCanvas {
    let mut canvas = TreeCanvas::new();

    let pixel_json = include_str!("enderlogo.json");

    let pixel_data: Vec<Pixel> = serde_json::from_str(&pixel_json).unwrap();

    let mut index = 0;

    let wave_index: f32 = (tick * 2 % 33) as f32;

    for y in 0..75 {
        for x in 0..20 {
            if y >= 33 && y <= 41 {
                let this_pixel = pixel_data.get(index).unwrap();
                canvas.set_pixel(x, y, *this_pixel);
                index += 1;
            }
            else {
                canvas.set_pixel(x, y, Pixel { r: 0, g: 0, b: 0 })
            }
        }
    }

    for x in 0..20 {
        let wave_offset = (((((x as f32) - 9.5) as f64) / (2.0 * std::f64::consts::PI)).cos() * -4.0).round() as f32;

        let y1: f32 = 33.0 - wave_index + wave_offset;
        let y2: f32 = 41.0 + wave_index - wave_offset;
        canvas.set_pixel(x, std::cmp::max(0,y1 as i32) as usize, Pixel {
            r: 0,
            g: ((wave_index / 33.0) * 255.0) as u8,
            b: 0
        });
        canvas.set_pixel(x, std::cmp::min(74,y2 as i32) as usize, Pixel {
            r: 0,
            g: ((wave_index / 33.0) * 255.0) as u8,
            b: 0
        });
    }

    canvas
}
