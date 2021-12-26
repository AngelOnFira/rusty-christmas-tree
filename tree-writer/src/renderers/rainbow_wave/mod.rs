use std::sync::{Arc, Mutex};

use super::{Pixel, TreeCanvas};

// Name: <Name for this renderer>
// Description: <Something about this renderer>
// Author: <Your name>

lazy_static::lazy_static! {
    static ref RED_VAL:Arc<Mutex<i32>> = Arc::new(Mutex::new(235));
    static ref GREEN_VAL:Arc<Mutex<i32>> = Arc::new(Mutex::new(52));
    static ref BLUE_VAL:Arc<Mutex<i32>> = Arc::new(Mutex::new(52));

    static ref RED_STATE:Arc<Mutex<i32>> = Arc::new(Mutex::new(0));
    static ref GREEN_STATE:Arc<Mutex<i32>> = Arc::new(Mutex::new(1));
    static ref BLUE_STATE:Arc<Mutex<i32>> = Arc::new(Mutex::new(0));
}

pub fn draw(tick: u64) -> TreeCanvas {
    let mut canvas = TreeCanvas::new();

    let mut red_val = RED_VAL.lock().unwrap();
    let mut green_val = GREEN_VAL.lock().unwrap();
    let mut blue_val = BLUE_VAL.lock().unwrap();

    let mut red_state = RED_STATE.lock().unwrap();
    let mut green_state = GREEN_STATE.lock().unwrap();
    let mut blue_state = BLUE_STATE.lock().unwrap();

    for y in 0..75 {
        *red_val += *red_state;
        *green_val += *green_state;
        *blue_val += *blue_state;

        if *red_val == 235 && *green_val == 235 && *blue_val == 52 {
            *green_state = 0;
            *red_state = -1;
        }
        if *red_val == 52 && *green_val == 235 && *blue_val == 52 {
            *red_state = 0;
            *blue_state = 1;
        }
        if *red_val == 52 && *green_val == 235 && *blue_val == 235 {
            *blue_state = 0;
            *green_state = -1;
        }
        if *red_val == 52 && *green_val == 52 && *blue_val == 235 {
            *green_state = 0;
            *red_state = 1;
        }
        if *red_val == 235 && *green_val == 52 && *blue_val == 235 {
            *red_state = 0;
            *blue_state = -1;
        }
        if *red_val == 235 && *green_val == 52 && *blue_val == 52 {
            *blue_state = 0;
            *green_state = 1;
        }
        for x in 0..20 {
            if y == 0 || y == 74 || x == 0 || x == 19 {
                let this_pixel = Pixel {
                    r: 255 as u8,
                    g: 255 as u8,
                    b: 255 as u8,
                };
                canvas.set_pixel(x, y, this_pixel)
            } else if tick > 3 && y >= ((tick - 2) as usize % 75) && y <= ((tick + 2) as usize % 75)
            {
                let this_pixel = Pixel {
                    r: *red_val as u8,
                    g: *green_val as u8,
                    b: *blue_val as u8,
                };
                canvas.set_pixel(x, y, this_pixel)
            } else {
                let this_pixel = Pixel { r: 0, g: 0, b: 0 };
                canvas.set_pixel(x, y, this_pixel)
            }
        }
    }

    canvas
}
