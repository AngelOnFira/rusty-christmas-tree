use std::cmp::min;
use super::{Pixel, TreeCanvas};

use std::sync::{Arc, Mutex};
use rand::Rng;

// Name: Snow
// Description: draw colorful snow, as well as provides an example of a stated renderer (using lazy static)
// Author: Eve (@ayyEve)


// because we need access to a list, we can store our state in a lazy static constant
lazy_static::lazy_static! {
    // stores the state
    static ref THINGY: Arc<Mutex<Thingy>> = Arc::new(Mutex::new(Thingy::new()));
}

/// generate a snowflake
fn gen_flake() -> (i32, i32, (u8, u8, u8)) {
    let x = rand::thread_rng().gen_range(0..20);
    let y = 0;

    let r = rand::thread_rng().gen_range(0..255);
    let g = rand::thread_rng().gen_range(0..255);
    let b = rand::thread_rng().gen_range(0..255);

    (x, y, (r, g, b))
}

struct Thingy {
    // list of current snow flakes
    flakes: Vec<(i32, i32, (u8, u8, u8))>,
}
impl Thingy {
    fn new() -> Self {
        Self {
            flakes: Vec::new(),
        }
    }
    fn update(&mut self, tick: u64) {

        // update snowflakes
        for (x, y, _) in self.flakes.iter_mut() {
            // move every snowflake down
            *y += 1;

            // every 2 ticks, move a snowflake either left, right, or not at all
            if tick % 2 == 0 {
                let mut offset = rand::thread_rng().gen_range(0i32..2);
                // 50% chance to move left, if moving right
                if rand::thread_rng().gen_range(0..10) > 5u8 {
                    offset *= -1;
                }
                
                *x += offset;
                if *x < 0 {*x = 20}
            }
        }

        // every 3 frames, add a few snowflakes
        if tick % 3 == 0 {
            for i in 0..rand::thread_rng().gen_range(0..3) {
                self.flakes.push(gen_flake());
            }
        }
        
        // remove flakes too low
        self.flakes.retain(|(x, y, _)| {
            *y < 75 && *x < 20
        });
    }
}


pub fn draw(tick: u64) -> TreeCanvas {
    let mut canvas = TreeCanvas::new();

    // lock the thingy so we can access it
    let mut lock = THINGY.lock().unwrap();
    lock.update(tick);

    // clear the screen (could technically loop through flakes before update and )
    for y in 0..75 {
        for x in 0..20 {
            let this_pixel = Pixel {
                r: 0,
                g: 0,
                b: 0,
            };
            canvas.set_pixel(x, y, this_pixel)
        }
    }

    // draw current flakes
    for (x, y, (r, g, b)) in lock.flakes.iter() {
        let this_pixel = Pixel {
            r: *r,
            g: *g,
            b: *b,
        };
        canvas.set_pixel(*x as usize, 74 - *y as usize, this_pixel)
    }

    canvas
}
