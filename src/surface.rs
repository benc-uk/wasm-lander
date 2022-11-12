use crate::gfx;
use crate::wasm4;
use noise::{NoiseFn, Perlin};

pub struct Surface {
    pub heights: [f64; 512],
    rng: Perlin,
}

const Y: f64 = 183.8;
const HIGH: f64 = 100.0;
const SMOOTH: f64 = 20.0;

impl Surface {
    pub fn new() -> Self {
        Self {
            heights: [0.0; 512],
            rng: Perlin::new(1234567),
        }
    }

    pub fn set_heights(&mut self) {
        for i in 0..512 {
            let rand_f = self.rng.get([i as f64 / SMOOTH, Y]);
            self.heights[i] = (rand_f + 1.0) / 2.0 * HIGH;
        }
    }

    pub fn draw(&mut self, x_offset: i32, _y_offset: i32) {
        gfx::set_draw_color(2);

        for x in 0..160 as i32 {
            let h = self.heights[(x + x_offset) as usize];
            wasm4::rect(x, 159 - h as i32, 1, h as u32);
        }
    }

    pub fn check_collision(&self, x: f64, y: f64) -> bool {
        let h = self.heights[x as usize];
        y > (159.0 - h)
    }
}
