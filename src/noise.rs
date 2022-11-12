use fastrand::Rng;

use crate::wasm4::trace;

const M: f32 = 4294967296.0;
const A: f32 = 1664525.0;
const C: f32 = 1.0;

pub struct PSNG {
    rng: Rng,
    z: f32,
}

impl PSNG {
    pub fn new(seed: u64) -> Self {
        let rand_gen = Rng::with_seed(seed);
        let z_init = rand_gen.f32() * M;
        trace(format!("z_init: {}", z_init));

        Self {
            rng: rand_gen,
            z: z_init.floor(),
        }
    }

    pub fn next(&mut self) -> f32 {
        //self.z = (A * self.z + C) % M;
        //self.z / M
        self.rng.f32()
    }
}
