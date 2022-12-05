use crate::gfx;
use crate::wasm4;
use fastrand::Rng;
use noise::{NoiseFn, Perlin};

pub struct Surface {
    noise: Perlin,
    pad_locations: [i32; PAD_COUNT],
}

const PAD_COUNT: usize = 8;
const SCREEN_SZ: i32 = 160;
const Y: f64 = 183.8;
const HIGH: f64 = 100.0;
const SMOOTH: f64 = 20.0;
const STEP: i32 = 8;

impl Surface {
    pub fn new(seed: u32) -> Self {
        let rng = Rng::with_seed(seed as u64);
        let mut surface = Surface {
            noise: Perlin::new(seed),
            pad_locations: [0; PAD_COUNT],
        };

        // randomize pad locations
        for i in 0..PAD_COUNT {
            surface.pad_locations[i] = rng.i32(100..185) * STEP as i32;
        }

        surface
    }

    pub fn draw(&mut self, x_offset: i32) {
        for x in (0..SCREEN_SZ + STEP).step_by(STEP as usize) {
            let x1 = round_to_limit(x + x_offset, -STEP);
            let x2 = round_to_limit(x + x_offset, STEP);
            let h1 = self.get_height(x1 as f64);
            let h2 = self.get_height(x2 as f64);

            // check if we need to draw a pad
            if self.pad_locations.contains(&x1) {
                gfx::set_draw_color(4);
                wasm4::rect(x1 - x_offset, SCREEN_SZ - h1 as i32, STEP as u32, 2);
                continue;
            }

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
        let x1 = round_to_limit(x as i32, -STEP);
        let x2 = round_to_limit(x as i32, STEP);
        let h1 = self.get_height(x1 as f64);
        let h2 = self.get_height(x2 as f64);

        if h1 < h2 {
            if y > SCREEN_SZ as f64 - lerp(h1, h2, (x - x1 as f64) / (x2 - x1) as f64) {
                return true;
            }
        } else {
            if y > SCREEN_SZ as f64 - lerp(h2, h1, (x - x2 as f64) / (x1 - x2) as f64) {
                return true;
            }
        }

        return false;
    }

    fn get_height(&self, x: f64) -> f64 {
        let mut x_pos = x;

        // Stuff to make pads work and join up
        for i in 0..PAD_COUNT {
            if x == (self.pad_locations[i] + STEP) as f64 {
                x_pos = x - STEP as f64;
                break;
            }
        }

        // if x == (self.pad_locations[0] + STEP) as f64 {
        //     x_pos = x - STEP as f64;
        // }

        (self.noise.get([x_pos / SMOOTH, Y]) + 1.0) / 2.0 * HIGH
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

// Clasic lerp function
fn lerp(a: f64, b: f64, t: f64) -> f64 {
    a + (b - a) * t
}
