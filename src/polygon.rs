use crate::gfx;
use crate::surface::Surface;
use crate::wasm4;

#[derive(Copy, Clone, Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug)]
pub struct Polygon {
    pub points: Vec<Point>,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

impl Polygon {
    pub fn new() -> Self {
        Self { points: Vec::new() }
    }

    pub fn add_point(&mut self, x: f64, y: f64) {
        self.points.push(Point { x, y });
    }

    pub fn draw(&self, color: u16) {
        gfx::set_draw_color(color);

        let mut i = 0;
        while i < self.points.len() {
            let p1 = self.points[i];
            let p2 = self.points[(i + 1) % self.points.len()];
            wasm4::line(p1.x as i32, p1.y as i32, p2.x as i32, p2.y as i32);
            i += 1;
        }
    }

    pub fn translate(&mut self, x: f64, y: f64) {
        let mut i = 0;
        while i < self.points.len() {
            self.points[i].x += x;
            self.points[i].y += y;
            i += 1;
        }
    }

    pub fn rotate(&mut self, angle: f64) {
        let mut i = 0;
        while i < self.points.len() {
            let p = self.points[i];
            self.points[i].x = p.x * angle.cos() - p.y * angle.sin();
            self.points[i].y = p.x * angle.sin() + p.y * angle.cos();
            i += 1;
        }
    }

    pub fn scale(&mut self, scale: f64) {
        let mut i = 0;
        while i < self.points.len() {
            self.points[i].x *= scale;
            self.points[i].y *= scale;
            i += 1;
        }
    }

    pub fn check_collision(&self, surface: &Surface) -> bool {
        let mut i = 0;
        while i < self.points.len() {
            let p = self.points[i];
            if surface.check_collision(p.x, p.y) {
                return true;
            }
            i += 1;
        }
        false
    }

    pub fn clone(&self) -> Polygon {
        let mut new_poly = Polygon::new();
        for point in self.points.iter() {
            new_poly.add_point(point.x, point.y);
        }
        new_poly
    }
}
