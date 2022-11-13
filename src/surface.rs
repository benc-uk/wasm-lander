use crate::gfx;
use crate::wasm4;
use noise::{NoiseFn, Perlin};

pub struct Surface {
    //pub heights: [f64; 512],
    rng: Perlin,
}

const Y: f64 = 183.8;
const HIGH: f64 = 100.0;
const SMOOTH: f64 = 20.0;

impl Surface {
    pub fn new(seed: u32) -> Self {
        Self {
            rng: Perlin::new(seed),
        }
    }

    pub fn draw(&mut self, x_offset: i32, _y_offset: i32) {
        gfx::set_draw_color(2);

        for x in 0..160 {
            let h = self.get_height((x + x_offset) as f64);
            wasm4::rect(x, 160 - h as i32, 1, h as u32);
        }
    }

    pub fn check_collision(&self, x: f64, y: f64) -> bool {
        let h = self.get_height(x as f64);
        y > (159.0 - h)
    }

    fn get_height(&self, x: f64) -> f64 {
        (self.rng.get([x / SMOOTH, Y]) + 1.0) / 2.0 * HIGH
    }
}
