use crate::polygon;
use crate::polygon::Point;
use crate::surface::Surface;
use crate::wasm4;
use std::collections::HashMap;

pub struct Ship {
    pub destroyed: bool,
    pub angle: f64,

    parts: HashMap<String, polygon::Polygon>,
    pos: polygon::Point,
    velocity: polygon::Point,
    thrust: f64,
    engine_on: bool,
    fuel: f64,
}

impl Ship {
    pub fn new() -> Self {
        let mut body = polygon::Polygon::new();
        body.add_point(-3.0, 4.0);
        body.add_point(4.0, 2.5);
        body.add_point(4.0, -2.5);
        body.add_point(-3.0, -4.0);

        let mut flame = polygon::Polygon::new();
        flame.add_point(-4.0, 1.5);
        flame.add_point(-10.0, 0.0);
        flame.add_point(-4.0, -1.5);

        let mut leg1 = polygon::Polygon::new();
        leg1.add_point(-3.0, 4.0);
        leg1.add_point(-6.0, 5.0);
        leg1.add_point(-3.0, 3.0);

        let mut leg2 = polygon::Polygon::new();
        leg2.add_point(-3.0, -4.0);
        leg2.add_point(-6.0, -5.0);
        leg2.add_point(-3.0, -3.0);

        let mut parts_map = HashMap::new();
        parts_map.insert("body".to_string(), body);
        parts_map.insert("leg1".to_string(), leg1);
        parts_map.insert("leg2".to_string(), leg2);
        parts_map.insert("flame".to_string(), flame);

        Self {
            parts: parts_map,
            pos: Point::new(5.0, 20.0),
            velocity: Point::new(0.12, 0.0),
            thrust: 0.002,
            engine_on: false,
            angle: 0.0,
            fuel: 180.0,
            destroyed: false,
        }
    }

    pub fn update(&mut self, gravity: f64) {
        if self.engine_on {
            self.velocity.x += self.thrust * self.angle.cos();
            self.velocity.y += self.thrust * self.angle.sin();
            self.fuel -= 0.2;
            wasm4::tone(100, 4, 20, wasm4::TONE_NOISE);
        }

        self.velocity.y += gravity;
        self.pos.x += self.velocity.x;
        self.pos.y += self.velocity.y;
    }

    pub fn set_engines(&mut self, state: bool) {
        self.engine_on = state;
        if self.fuel <= 0.0 {
            self.engine_on = false;
            self.fuel = 0.0;
        }
    }

    pub fn get_fuel(&self) -> f64 {
        self.fuel
    }

    pub fn get_pos(&self) -> Point {
        self.pos
    }

    pub fn get_speed(&self) -> f64 {
        let mag = self.velocity.x * self.velocity.x + self.velocity.y * self.velocity.y;
        mag.sqrt()
    }

    pub fn draw(&mut self, surface: &Surface) {
        self.draw_part("body", surface, 0x3);
        self.draw_part("leg1", surface, 0x2);
        self.draw_part("leg2", surface, 0x2);

        // Thruster flame
        if self.engine_on {
            let mut flame = self.parts.get("flame").unwrap().clone();

            flame.scale(1.0);
            flame.rotate(self.angle);
            flame.translate(80.0, self.pos.y);
            flame.draw(0x4);
        }
    }

    fn draw_part(&mut self, name: &str, surface: &Surface, color: u16) {
        let mut p = self.parts.get(name).unwrap().clone();
        p.scale(1.0);
        p.rotate(self.angle);

        // Check collision with surface in "world" coordinates
        p.translate(self.pos.x, self.pos.y);
        if p.check_collision(surface) {
            self.destroyed = true;
        }

        // Draw in "screen" coordinates
        p.translate(-self.pos.x + 80.0, 0.0);

        p.draw(color);
    }
}
