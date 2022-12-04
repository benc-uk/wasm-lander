use crate::gfx;
use crate::polygon::Point;
use crate::wasm4;

#[derive(Copy, Clone)]
pub struct Particle {
    pub pos: Point,
    angle: f64,
    speed: f64,
    lifetime: f64,
}

impl Particle {
    pub fn new(x: f64, y: f64, angle: f64, speed: f64, lifetime: f64) -> Self {
        Self {
            pos: Point::new(x, y),
            angle,
            speed,
            lifetime,
        }
    }

    pub fn update(&mut self) {
        self.lifetime -= 1.0;

        self.pos.x += self.angle.cos() * self.speed;
        self.pos.y += self.angle.sin() * self.speed;
    }

    pub fn is_dead(&self) -> bool {
        self.lifetime <= 0.0
    }

    pub fn draw(&self, color: u16) {
        if self.is_dead() {
            return;
        }
        gfx::set_draw_color(color);
        wasm4::rect(self.pos.x as i32, self.pos.y as i32, 1, 1);
    }
}
