use std::io::Read;

use super::{Pixel, TreeCanvas};

use std::collections::HashMap;

use flate2::read::GzDecoder;
use serde::{de::value::MapDeserializer, Deserialize};
use serde_json::{self, Value};

use cached::proc_macro::cached;

#[derive(Deserialize, Debug, Clone)]
struct Video {
    values: Box<Vec<u8>>,
    r: Box<Vec<usize>>,
    g: Box<Vec<usize>>,
    b: Box<Vec<usize>>,
}

// Name: Sell a bee
// Description: Celebi's got some new moves in this render!
// Author: Liam Henderson
#[cached(size = 1)]
fn get_video(_n: u8) -> Video {
    // Compress the json file. Could be useful if another file needs to be
    // compressed.
    //
    // let json_video = include_str!("video.json"); let mut compressed_file =
    // File::create("video.json.gz").unwrap(); let mut e = GzEncoder::new(&mut
    // compressed_file, flate2::Compression::default());
    // e.write_all(json_video.as_bytes()).unwrap();

    let compressed_content = include_bytes!("video.json.gz");

    // Decompress the JSON
    let mut decompressed_content = String::new();
    GzDecoder::new(&compressed_content[..])
        .read_to_string(&mut decompressed_content)
        .unwrap();

    let data: HashMap<&str, Value> = serde_json::from_str(&decompressed_content).unwrap();
    let video = Video::deserialize(MapDeserializer::new(data.into_iter())).unwrap();

    video
}

pub fn draw(tick: u64) -> TreeCanvas {
    let mut canvas = TreeCanvas::new();

    let frame_num: usize = (tick % 250).try_into().unwrap();

    let parsed: Video = get_video(0);

    let frame_r = &parsed.r[frame_num * 75 * 20..(frame_num + 1) * 75 * 20];
    let frame_g = &parsed.g[frame_num * 75 * 20..(frame_num + 1) * 75 * 20];
    let frame_b = &parsed.b[frame_num * 75 * 20..(frame_num + 1) * 75 * 20];

    for y in 0..75 {
        for x in 0..20 {
            let pixel: Pixel = Pixel {
                r: parsed.values[frame_r[y * 20 + x]],
                g: parsed.values[frame_g[y * 20 + x]],
                b: parsed.values[frame_b[y * 20 + x]],
            };
            canvas.set_pixel(x, 74 - y, pixel)
        }
    }

    canvas
}
