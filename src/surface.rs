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
const STEP: usize = 4;

impl Surface {
    pub fn new(seed: u32) -> Self {
        Self {
            rng: Perlin::new(seed),
        }
    }

    pub fn draw(&mut self, x_offset: i32, _y_offset: i32) {
        for x in (0..160).step_by(STEP) {
            let h1 = self.get_height((x + x_offset) as f64);
            let h2 = self.get_height((x + x_offset + STEP as i32) as f64);
            gfx::set_draw_color(3);
            wasm4::line(x, 160 - h1 as i32, x + STEP as i32, 160 - h2 as i32);
            gfx::set_draw_color(2);
            wasm4::line(x, 162 - h1 as i32, x + STEP as i32, 162 - h2 as i32);
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
