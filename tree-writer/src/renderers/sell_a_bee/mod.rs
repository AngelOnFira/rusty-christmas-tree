use std::cmp::min;

use image::io::Reader as ImageReader;
use image::imageops::{self};
use image::{Rgb};

use std::env;

use super::{Pixel, TreeCanvas};

// Name: Sell a bee
// Description: Celebi's got some new moves in this render!
// Author: Liam Henderson

pub fn draw(tick: u64) -> TreeCanvas {
    let mut canvas = TreeCanvas::new();

    let frame_num = tick % 249 + 1;
    let frame_name = format!("tree-writer/src/renderers/sell_a_bee/right resolution/{:0>4}.png", frame_num);
    let frame = ImageReader::open(frame_name).unwrap().decode().unwrap().resize_exact(20, 75, imageops::FilterType::Gaussian);
    

    for y in 0..75 {
        for x in 0..20 {
            let pixel: Rgb<u8> = *frame.clone().into_rgb8().get_pixel(x, 74-y);
            let this_pixel = Pixel {
                r: pixel[0],
                g: pixel[1],
                b: pixel[2],
            };
            canvas.set_pixel(x.try_into().unwrap(), y.try_into().unwrap(), this_pixel)
        }
    }

    canvas
}
