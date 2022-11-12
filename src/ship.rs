use crate::gfx;
use crate::surface::Surface;
use crate::wasm4;
use cgmath::*;

pub struct Ship {
    ship_points: [Point2<f64>; 3],
    pos: cgmath::Point2<f64>,
    velocity: cgmath::Vector2<f64>,
    angle: f64,
    thrust: f64,
    engine_on: bool,
    fuel: f64,
    destroyed: bool,
}

impl Ship {
    pub fn new() -> Self {
        Self {
            ship_points: [
                Point2::new(0.0, -2.0),
                Point2::new(5.0, 0.0),
                Point2::new(0.0, 2.0),
            ],
            pos: cgmath::Point2::new(80.0, 20.0),
            velocity: cgmath::Vector2::new(0.0, 0.0),
            thrust: 0.002,
            engine_on: false,
            angle: -1.5,
            fuel: 100.0,
            destroyed: false,
        }
    }

    pub fn update(&mut self, pressed: u8, gravity: f64) {
        if pressed & wasm4::BUTTON_RIGHT != 0 {
            self.angle += 0.4;
        }
        if pressed & wasm4::BUTTON_LEFT != 0 {
            self.angle -= 0.4;
        }

        if self.engine_on {
            self.velocity.x += self.thrust * self.angle.cos();
            self.velocity.y += self.thrust * self.angle.sin();
            self.fuel -= 0.2;
        }

        self.velocity.y += gravity;
        self.pos.x += self.velocity.x;
        self.pos.y += self.velocity.y;
    }

    pub fn set_engines(&mut self, state: bool) {
        self.engine_on = state;
    }

    pub fn get_fuel(&self) -> f64 {
        self.fuel
    }

    pub fn is_destroyed(&self) -> bool {
        self.destroyed
    }

    pub fn draw(&mut self, surface: &Surface) {
        let rot: cgmath::Basis2<f64> = cgmath::Rotation2::from_angle(Rad(self.angle));

        // Draw ship
        gfx::set_draw_color(0x3);
        for i in 0..2 {
            let p1 = rot.rotate_point(self.ship_points[i as usize]);
            let p2 = rot.rotate_point(self.ship_points[i + 1 as usize]);
            wasm4::line(
                (p1.x + self.pos.x) as i32,
                (p1.y + self.pos.y) as i32,
                (p2.x + self.pos.x) as i32,
                (p2.y + self.pos.y) as i32,
            );

            let c1 = surface.check_collision(p1.x + self.pos.x, p1.y + self.pos.y);
            if c1 {
                self.destroyed = true;
            }
            let c2 = surface.check_collision(p2.x + self.pos.x, p2.y + self.pos.y);
            if c2 {
                self.destroyed = true;
            }
        }

        // Thruster flame
        if self.engine_on {
            gfx::set_draw_color(0x4);
            let mut p1 = cgmath::Point2::new(-3.0, 0.0);
            p1 = rot.rotate_point(p1);
            wasm4::line(
                (p1.x + self.pos.x) as i32,
                (p1.y + self.pos.y) as i32,
                (self.pos.x) as i32,
                (self.pos.y) as i32,
            );
        }
    }
}
