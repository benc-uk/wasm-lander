use crate::gfx;
use crate::rand_tab;
use crate::ship::Ship;
use crate::wasm4;
use noilib_simple::NoiseGenerator;

pub struct Surface {
    noise: NoiseGenerator,
    pad_locations: [f32; PAD_COUNT],
    heights: [i16; SCREEN_SZ as usize],
    pub scale: f32,
}

const PAD_COUNT: usize = 4;
const PAD_SZ: f32 = 0.5;
const SCREEN_SZ: i32 = 160;
const SCREEN_SZ_H: i32 = 80;
const Y: f32 = 183.8;
const HIGH: f32 = 160.0;
const SMOOTH: f32 = 40.0;

impl Surface {
    pub fn new(seed: u32) -> Self {
        rand_tab::seed(seed as usize);
        let mut surface = Surface {
            noise: NoiseGenerator::new(seed as u64),
            pad_locations: [0.0; PAD_COUNT],
            scale: 1.0,
            heights: [0; SCREEN_SZ as usize],
        };

        // randomize pad locations
        for i in 0..PAD_COUNT {
            surface.pad_locations[i] = 0.0 + (rand_tab::f64() * 18.0) as f32;
        }

        surface
    }

    pub fn draw(&mut self, x_offset: f32, y_offset: f32) {
        let zoom = SMOOTH * self.scale;
        let zoom_magic = 110.0;

        for x in 0..SCREEN_SZ {
            let x_zoom = (x_offset / SMOOTH) + ((x as f32 - SCREEN_SZ_H as f32) / zoom);
            let mut h = ((self.noise.perlin(x_zoom, Y) + 1.0) / 2.0) * HIGH * self.scale;
            h = h + y_offset - (zoom_magic * self.scale);

            // check if we're on a pad
            let mut is_pad = false;
            for i in 0..PAD_COUNT {
                if (x_zoom > self.pad_locations[i]) && (x_zoom < self.pad_locations[i] + PAD_SZ) {
                    let mut pad_h = ((self.noise.perlin(self.pad_locations[i] + PAD_SZ, Y) + 1.0)
                        / 2.0)
                        * HIGH
                        * self.scale;
                    pad_h = pad_h + y_offset - (zoom_magic * self.scale);
                    h = pad_h;
                    is_pad = true;
                }
            }

            self.heights[x as usize] = h as i16;

            gfx::set_draw_color(2);
            wasm4::line(x, SCREEN_SZ, x, SCREEN_SZ - h as i32);

            if is_pad {
                gfx::set_draw_color(4);
                wasm4::rect(x, SCREEN_SZ - h as i32, 1, 1);
                wasm4::rect(x, SCREEN_SZ + 2 - h as i32, 1, 1);

                gfx::set_draw_color(2);
                wasm4::rect(x, 18 + (x % 3), 1, 1);

                // Use negative heights to indicate a pad
                self.heights[x as usize] = -h as i16;
            }
        }
    }

    pub fn check_collision(&self, x: f64, y: f64, ship: &Ship) -> u8 {
        // 0 is no collision
        // 1 is landed OK
        // 2 is a crash
        // 3+ is bad landing

        let mut h = self.heights[x as usize];
        let mut is_pad = false;
        if h < 0 {
            is_pad = true;
            h = -h;
        }

        if y > SCREEN_SZ as f64 - h as f64 {
            // Check if we're on a pad
            if is_pad {
                let ang = ship.angle.to_degrees() + 90.0;
                let speed = ship.get_speed() * 100.0;

                // Criteria for a safe & good landing
                if speed > 10.0 {
                    return 3;
                }
                if ang.abs() > 1.1 {
                    return 4;
                }
                if ship.get_velocity().x > 0.02 {
                    return 5;
                }

                return 1;
            }

            return 2;
        }

        return 0;
    }
}
