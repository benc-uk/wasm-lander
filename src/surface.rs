use crate::gfx;
use crate::wasm4;
use noilib_simple::NoiseGenerator;

pub struct Surface {
    noise: NoiseGenerator,
    pad_locations: [f32; PAD_COUNT],
    heights: [u8; SCREEN_SZ as usize],
    pub scale: f32,
}

const PAD_COUNT: usize = 1;
const PAD_SZ: f32 = 0.4;
const SCREEN_SZ: i32 = 160;
const Y: f32 = 183.8;
const HIGH: f32 = 150.0;
const SMOOTH: f32 = 40.0;

impl Surface {
    pub fn new(seed: u32) -> Self {
        let mut surface = Surface {
            noise: NoiseGenerator::new(seed as u64),
            pad_locations: [0.0; PAD_COUNT],
            scale: 1.0,
            heights: [0; SCREEN_SZ as usize],
        };

        // randomize pad locations
        for i in 0..PAD_COUNT {
            surface.pad_locations[i] = 8.0;
        }

        surface
    }

    pub fn draw(&mut self, x_offset: f32, y_offset: f32) {
        let zoom = SMOOTH * self.scale;

        for x in 0..SCREEN_SZ {
            let x_zoom = (x_offset / SMOOTH) + ((x as f32 - 80.0) / zoom);
            let mut h = ((self.noise.perlin(x_zoom, Y) + 1.0) / 2.0) * HIGH * self.scale;
            h = h + y_offset - 50.0;

            // check if we're on a pad
            let mut is_pad = false;
            for i in 0..PAD_COUNT {
                if (x_zoom > self.pad_locations[i]) && (x_zoom < self.pad_locations[i] + PAD_SZ) {
                    let mut pad_h = ((self.noise.perlin(self.pad_locations[i] + PAD_SZ, Y) + 1.0)
                        / 2.0)
                        * HIGH
                        * self.scale;
                    pad_h = pad_h + y_offset - 50.0;
                    h = pad_h;
                    is_pad = true;
                }
            }
            self.heights[x as usize] = h as u8;

            gfx::set_draw_color(2);
            wasm4::line(x, SCREEN_SZ, x, SCREEN_SZ - h as i32);

            if is_pad {
                gfx::set_draw_color(4);
                wasm4::rect(x, SCREEN_SZ - h as i32, 1, 1);
                wasm4::rect(x, 162 - h as i32, 1, 1);
            }
        }
    }

    pub fn check_collision(&self, x: f64, y: f64) -> bool {
        let h = self.heights[x as usize];
        if y > SCREEN_SZ as f64 - h as f64 {
            return true;
        }

        return false;
    }
}
