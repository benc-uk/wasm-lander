use crate::particle::Particle;
use crate::polygon;
use crate::polygon::Point;
use crate::rand_tab;
use crate::surface::Surface;
use crate::wasm4;

const SHIP_Y: f64 = 50.0;

pub struct Ship {
    pub destroyed: bool,
    pub crash_reason: String,
    pub landed: bool,
    pub angle: f64,
    pub scale: f64,

    parts: Vec<polygon::Polygon>,
    pub pos: polygon::Point,
    velocity: polygon::Point,
    thrust: f64,
    engine_on: bool,
    fuel: f64,
    particles: [Particle; 30],
}

impl Ship {
    pub fn new() -> Self {
        let mut body = polygon::Polygon::new();
        body.add_point(-3.0, 4.0);
        body.add_point(4.0, 2.5);
        body.add_point(4.0, -2.5);
        body.add_point(-3.0, -4.0);

        let mut flame = polygon::Polygon::new();
        flame.add_point(-3.0, 0.0);

        let mut leg1 = polygon::Polygon::new();
        leg1.add_point(-3.0, 4.0);
        leg1.add_point(-6.0, 5.0);
        leg1.add_point(-3.0, 3.0);

        let mut leg2 = polygon::Polygon::new();
        leg2.add_point(-3.0, -4.0);
        leg2.add_point(-6.0, -5.0);
        leg2.add_point(-3.0, -3.0);

        let mut parts_vec = Vec::new();
        parts_vec.push(body);
        parts_vec.push(leg1);
        parts_vec.push(leg2);
        parts_vec.push(flame);

        Self {
            parts: parts_vec,
            pos: Point::new(200.0, 15.0),
            velocity: Point::new(0.29, 0.0),
            scale: 1.0,
            thrust: 0.002,
            engine_on: false,
            angle: 0.0,
            fuel: 250.0,
            destroyed: false,
            crash_reason: String::new(),
            landed: false,
            particles: [Particle::new(0.0, 0.0, 0.0, 0.0, 0.0); 30],
        }
    }

    pub fn update(&mut self, gravity: f64) {
        if self.engine_on {
            self.velocity.x += self.thrust * self.angle.cos();
            self.velocity.y += self.thrust * self.angle.sin();
            self.fuel -= 0.2;
            wasm4::tone((self.fuel + 80.0) as u32, 4, 20, wasm4::TONE_NOISE);
        }

        self.velocity.y += gravity;
        self.pos.x += self.velocity.x;
        self.pos.y += self.velocity.y;

        if self.pos.y < 10.0 {
            self.destroyed = true;
            self.crash_reason = String::from("You zoomed off\ninto space!");
        }
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

    pub fn get_velocity(&self) -> Point {
        self.velocity
    }

    pub fn get_speed(&self) -> f64 {
        let mag = self.velocity.x * self.velocity.x + self.velocity.y * self.velocity.y;
        mag.sqrt()
    }

    pub fn draw(&mut self, surface: &Surface) {
        // Draw the main parts of the ship
        self.draw_part(0, surface, 0x3);
        self.draw_part(1, surface, 0x2);
        self.draw_part(2, surface, 0x2);

        // Thruster effects
        if self.engine_on {
            let mut flame_point = self.parts.get(3).unwrap().clone();
            flame_point.scale(self.scale);
            flame_point.rotate(self.angle);
            flame_point.translate(80.0, SHIP_Y);

            // find a dead particle and replace it
            for particle in self.particles.iter_mut() {
                if particle.is_dead() {
                    *particle = Particle::new(
                        flame_point.points[0].x,
                        flame_point.points[0].y,
                        (self.angle - 3.14) + ((rand_tab::f64() - 0.5) * 0.6),
                        1.2 + rand_tab::f64(),
                        (6.0 + rand_tab::f64() * 5.0) * self.scale,
                    );
                    break;
                }
            }
        }

        // Update and draw particles
        for particle in self.particles.iter_mut() {
            particle.update();
            particle.draw(0x4);
        }
    }

    fn draw_part(&mut self, part: usize, surface: &Surface, color: u16) {
        let mut p = self.parts.get(part).unwrap().clone();

        p.scale(self.scale);
        p.rotate(self.angle);

        // Draw poly in screen coordinates
        p.translate(80.0, SHIP_Y);

        // Collision detection is done in screen coordinates now
        let c = p.check_collision(surface, self);
        if c >= 2 {
            self.destroyed = true;
            if c == 2 {
                self.crash_reason = String::from("Crashed into the\nlunar surface");
            }
            if c == 3 {
                self.crash_reason = String::from("Landed too fast!");
            }
            if c == 4 {
                self.crash_reason = String::from("Landed at too\nsteep an angle");
            }
            if c == 5 {
                self.crash_reason = String::from("Sheared landing\nlegs");
            }

            return;
        }

        if c == 1 {
            self.landed = true;
            return;
        }

        p.draw(color);
    }
}
