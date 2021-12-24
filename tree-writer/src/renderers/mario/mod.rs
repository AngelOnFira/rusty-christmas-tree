use std::cmp::min;

use super::{Pixel, TreeCanvas};

// Name: Mario
// Author: Zesterer

pub fn draw(tick: u64) -> TreeCanvas {
    let mut canvas = TreeCanvas::new();

    let l = tick as usize;
    let o = (2, ((tick as f32 * 0.25).sin().abs() * 48.0) as usize);
    for y in 0usize..75 {
        for x in 0usize..20 {
            let r = (x.wrapping_sub(o.0), y.wrapping_sub(o.1));
            let c = if let Some(i) = (r.0 < 16 && r.1 < 14)
                .then(|| {
                    ([
                        0x124900dbu64,
                        0x9249249b,
                        0x16d6cb092,
                        0xaeb6cb6d2,
                        0xaed6d96da,
                        0xb5b6c9250,
                        0x1b6db690,
                        0x6da494494480,
                        0x6da492892805,
                        0x92924d25,
                        0x249a4925,
                        0xb64924925,
                        0x5b64924000,
                        0x5a00000000,
                    ][13 - r.1]
                        >> (45 - r.0 * 3)) as usize
                        & 7
                })
                .filter(|i| i > &0)
            {
                [!0, 0, 0xFF0000, 0xFF8060, 0x80FF, 0x805030, !0 - 0xFF][i]
            } else if y < 6 {
                0x5030 + ((l * 2 + x) % 5 == 0 || y % 5 == 0) as u32 * 0xFF0000
            } else if ((l / 2 + x) as i32 % 32 - 16).pow(2)
                + (y as i32 - 55 + (((l + x) as f32 * 0.07).sin() * 16.0) as i32).pow(2)
                < 24
            {
                0xFFFFFF
            } else if (y as f32) < ((l + x) as f32 * 0.1).sin().abs() * 16.0 + 10.0 {
                0x207010
            } else {
                0x000020
            };
            canvas.set_pixel(
                x,
                y,
                Pixel {
                    r: (c >> 16) as u8,
                    g: (c >> 8) as u8,
                    b: c as u8,
                },
            );
        }
    }

    canvas
}
