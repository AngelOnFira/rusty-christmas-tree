use serde::{Serialize, Deserialize};

#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct Pixel {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

pub struct TreeCanvas {
    pixels: [Pixel; 1500],
}

impl TreeCanvas {
    pub fn new() -> Self {
        Self {
            pixels: [Pixel {
                r: 255,
                g: 255,
                b: 255,
            }; 1500],
        }
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, pixel: Pixel) {
        if x >= 20 {
            panic!("Tried to set pixel at x={}, while x is max 20", x);
        }
        if y >= 75 {
            panic!("Tried to set pixel at y={}, while y is max 75", y);
        }

        self.pixels[x * 75 + y] = pixel;
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> Pixel {
        if x >= 20 {
            panic!("Tried to get pixel at x={}, while x is max 20", x);
        }
        if y >= 75 {
            panic!("Tried to get pixel at y={}, while y is max 75", y);
        }

        self.pixels[x * 75 + y]
    }

    pub fn get_canvas(&self) -> &[Pixel; 1500] {
        &self.pixels
    }

    pub fn convert_to_buffer(&self) -> [u8; 4500] {
        let mut buffer: [u8; 4500] = [0; 4500];
        let column_height = 75;

        for i in 0..1500 {
            let translation_x = i / column_height;
            let mut translation_y = i % column_height;

            // If we're on an odd column, flip the y direction
            if translation_x % 2 == 1 {
                translation_y = column_height - translation_y - 1;
            }

            buffer[i * 3 + 0] = self.get_pixel(translation_x, translation_y).r;
            buffer[i * 3 + 1] = self.get_pixel(translation_x, translation_y).g;
            buffer[i * 3 + 2] = self.get_pixel(translation_x, translation_y).b;
        }

        buffer
    }
}
