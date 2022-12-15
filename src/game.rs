use crate::gfx;
use crate::polygon::Point;
use crate::rand_tab;
use crate::ship;
use crate::surface;
use crate::wasm4;

pub struct Game {
    frame_count: u32,
    prev_gamepad: u8,
    is_game_over: bool,
    is_title_screen: bool,
    is_landed: bool,
    surface: surface::Surface,
    ship: ship::Ship,
    score: u32,
    stars: [Point; 60],
}

const GRAV: f64 = 0.00070;

impl Game {
    pub fn new() -> Self {
        let mut game = Game {
            frame_count: 0,
            prev_gamepad: 0,
            is_game_over: false,
            is_title_screen: true,
            is_landed: false,
            surface: surface::Surface::new(666666),
            ship: ship::Ship::new(),
            score: 0,
            stars: [Point::new(0.0, 0.0); 60],
        };

        game.init_stars();
        game
    }

    pub fn new_game(&mut self, score: u32) {
        self.is_game_over = false;
        self.is_landed = false;
        self.prev_gamepad = 0;
        self.ship = ship::Ship::new();
        self.surface = surface::Surface::new(self.frame_count);
        self.ship.scale = 1.0;
        self.score = score;
        self.init_stars();
    }

    pub fn update(&mut self) {
        self.frame_count += 1;

        let pressed = self.input();

        let mut scale = self.ship.pos.y / 50.0;
        if scale > 2.0 {
            scale = 2.0;
        }
        self.ship.scale = scale;
        self.surface.scale = scale as f32;

        if self.is_title_screen {
            if pressed & wasm4::BUTTON_1 != 0 || pressed & wasm4::BUTTON_2 != 0 {
                self.new_game(0);
                self.is_title_screen = false;
            }

            self.update_stars_title();
            self.draw_stars();

            gfx::shadow_text("** WASM LANDER ** ", 12, 15, 0x4, 0x2);
            gfx::set_draw_color(0x2);
            wasm4::line(0, 33, 160, 33);
            wasm4::line(0, 92, 160, 92);
            gfx::shadow_text("Press X for thrust", 8, 40, 0x4, 0x2);
            gfx::shadow_text("Left/Right to turn", 8, 50, 0x4, 0x2);
            gfx::shadow_text("Try to land on the\nlanding pads!", 8, 70, 0x4, 0x2);
            gfx::shadow_text("PRESS BUTTON\n  TO START", 33, 130, 0x3, 0x2);
            gfx::shadow_text("v12", 3, 150, 0x3, 0x2);

            return;
        }

        if self.is_game_over && !self.is_title_screen {
            if pressed & wasm4::BUTTON_1 != 0 {
                self.is_title_screen = true;
                self.init_stars();
            }

            self.draw_stars();

            gfx::shadow_text(&self.ship.crash_reason, 10, 30, 0x4, 0x2);
            gfx::shadow_text("GAME OVER!", 40, 70, 0x3, 0x2);
            gfx::shadow_text(
                (String::from("Final score: ") + self.score.to_string().as_str()).as_str(),
                10,
                90,
                0x4,
                0x2,
            );
            return;
        }

        if self.is_landed {
            let score = self.score + self.ship.get_fuel() as u32;

            if pressed & wasm4::BUTTON_1 != 0 {
                self.new_game(score);
                self.is_landed = false;
            }

            gfx::shadow_text("GREAT LANDING!", 30, 30, 0x4, 0x2);
            gfx::shadow_text(
                (String::from("Fuel left: ") + self.ship.get_fuel().round().to_string().as_str())
                    .as_str(),
                20,
                50,
                0x3,
                0x2,
            );

            gfx::shadow_text(
                (String::from("Score: ") + score.to_string().as_str()).as_str(),
                20,
                70,
                0x3,
                0x2,
            );

            return;
        }

        // Update stars based on ship position
        let mut i = 0;
        for star in self.stars.iter_mut() {
            star.x -= (self.ship.get_velocity().x * 0.20)
                * (1.0 / self.surface.scale as f64)
                * (i % 4) as f64;
            star.y -= (self.ship.get_velocity().y * 0.20)
                * (1.0 / self.surface.scale as f64)
                * (i % 4) as f64;
            if star.x < 0.0 {
                star.x = 160.0;
            }
            if star.x > 160.0 {
                star.x = 0.0;
            }
            if star.y < 0.0 {
                star.y = 160.0;
            }
            if star.y > 160.0 {
                star.y = 0.0;
            }
            i += 1;
        }
        self.draw_stars();

        self.ship.update(GRAV);
        self.surface
            .draw(self.ship.get_pos().x as f32, self.ship.get_pos().y as f32);
        self.ship.draw(&self.surface);

        let angle_str =
            String::from("A: ") + (self.ship.angle.to_degrees() + 90.0).to_string().as_str();
        let fuel_str = String::from("F: ") + self.ship.get_fuel().round().to_string().as_str();
        let speed_str =
            String::from("S: ") + (self.ship.get_speed() * 100.0).round().to_string().as_str();
        gfx::shadow_text(fuel_str.as_str(), 0, 0, 0x3, 0x2);
        gfx::shadow_text(speed_str.as_str(), 0, 10, 0x3, 0x2);
        gfx::shadow_text(angle_str.as_str(), 90, 0, 0x3, 0x2);

        if self.ship.destroyed {
            wasm4::tone(160, 50, 50, wasm4::TONE_NOISE);
            self.is_game_over = true;
        }

        if self.ship.landed {
            wasm4::tone(260, 80, 50, wasm4::TONE_TRIANGLE);
            self.is_landed = true;
        }
    }

    pub fn input(&mut self) -> u8 {
        let gamepad = unsafe { *wasm4::GAMEPAD1 };
        let just_pressed = gamepad & (gamepad ^ self.prev_gamepad);

        self.ship.set_engines(false);
        if gamepad & wasm4::BUTTON_1 != 0 {
            self.ship.set_engines(true);
        }

        if gamepad & wasm4::BUTTON_RIGHT != 0 {
            self.ship.angle += 0.03 * (0.5 / self.ship.scale);
        }

        if gamepad & wasm4::BUTTON_LEFT != 0 {
            self.ship.angle -= 0.03 * (0.5 / self.ship.scale);
        }

        self.prev_gamepad = gamepad;
        just_pressed
    }

    fn init_stars(&mut self) {
        let mut i = 0;
        while i < self.stars.len() {
            let star = &mut self.stars[i];
            star.x = rand_tab::f64() * 160.0;
            star.y = rand_tab::f64() * 160.0;
            i += 1;
        }
    }

    fn draw_stars(&mut self) {
        // Draw stars
        let mut i = 0;
        while i < self.stars.len() {
            let star = self.stars[i];
            gfx::set_draw_color(((i % 4) + 1) as u16);
            wasm4::rect(star.x as i32, star.y as i32, 1, 1);
            i += 1;
        }
    }

    fn update_stars_title(&mut self) {
        let mut i = 0;
        while i < self.stars.len() {
            let star = &mut self.stars[i];
            star.x += 0.5;
            star.y += 0.1;

            if star.x > 160.0 {
                star.x = 0.0;
            }

            if star.y > 160.0 {
                star.y = 0.0;
            }

            i += 1;
        }
    }
}
