use crate::gfx;
use crate::wasm4;
use noise::{NoiseFn, Perlin};

pub struct Surface {
    //h_buff: [f64; 162 + STEP as usize],
    noise: Perlin,
}

const SCREEN_SZ: i32 = 160;
const Y: f64 = 183.8;
const HIGH: f64 = 100.0;
const SMOOTH: f64 = 20.0;
const STEP: i32 = 8;

impl Surface {
    pub fn new(seed: u32) -> Self {
        Self {
            noise: Perlin::new(seed),
        }
    }

    pub fn draw(&mut self, x_offset: i32) {
        for x in (0..SCREEN_SZ + 4).step_by(STEP as usize) {
            let x1 = round_to_limit(x + x_offset, -STEP);
            let x2 = round_to_limit(x + x_offset, STEP);
            let h1 = self.get_height(x1 as f64);
            let h2 = self.get_height(x2 as f64);

            gfx::set_draw_color(3);
            wasm4::line(
                x1 - x_offset,
                SCREEN_SZ - h1 as i32,
                x2 - x_offset,
                SCREEN_SZ - h2 as i32,
            );
        }
    }

    pub fn check_collision(&self, x: f64, y: f64) -> bool {
        let h = self.get_height(x as f64);
        y > (SCREEN_SZ as f64 - h)
    }

    fn get_height(&self, x: f64) -> f64 {
        (self.noise.get([x / SMOOTH, Y]) + 1.0) / 2.0 * HIGH
    }
}

// Simple util function to round to set limit intervals
fn round_to_limit(value: i32, limit: i32) -> i32 {
    if limit < 0 {
        return value - (value % limit.abs());
    } else {
        return value + limit - (value % limit);
    }
}
