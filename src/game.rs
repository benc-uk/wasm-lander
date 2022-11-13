use crate::gfx;
use crate::ship;
use crate::surface;
use crate::wasm4;

pub struct Game {
    frame_count: u32,
    prev_gamepad: u8,
    is_game_over: bool,
    is_title_screen: bool,
    surface: surface::Surface,
    //camera_x: i32,
    ship: ship::Ship,
}

const GRAV: f64 = 0.0007;

impl Game {
    pub fn new() -> Self {
        Self {
            frame_count: 0,
            prev_gamepad: 0,
            is_game_over: false,
            is_title_screen: true,
            surface: surface::Surface::new(0),
            //camera_x: 0,
            ship: ship::Ship::new(),
        }
    }

    pub fn new_game(&mut self) {
        self.is_game_over = false;
        self.prev_gamepad = 0;
        self.ship = ship::Ship::new();
        self.surface = surface::Surface::new(self.frame_count);
    }

    pub fn update(&mut self) {
        let pressed = self.input();

        if self.is_title_screen {
            if pressed & wasm4::BUTTON_1 != 0 {
                self.new_game();
                self.is_title_screen = false;
            }

            gfx::shadow_text("WASM LANDER", 40, 20, 0x4, 0x2);
            gfx::shadow_text("Press X for thrust", 3, 40, 0x4, 0x2);
            gfx::shadow_text("Left/Right to turn", 3, 50, 0x4, 0x2);
            gfx::shadow_text("PRESS X TO START", 3, 70, 0x4, 0x2);
            return;
        }

        if self.is_game_over && !self.is_title_screen {
            if pressed & wasm4::BUTTON_1 != 0 {
                self.is_title_screen = true;
            }

            gfx::shadow_text("YOU CRASHED!", 30, 60, 0x4, 0x1);
            gfx::shadow_text("GAME OVER!", 40, 70, 0x4, 0x1);
            return;
        }

        self.frame_count += 1;

        gfx::set_draw_color(0x2);
        wasm4::text(format!("FUEL: {:.1}", self.ship.get_fuel()), 0, 0);
        wasm4::text(format!("SPED: {:.1}", self.ship.get_speed() * 100.0), 0, 10);
        wasm4::text(
            format!("ANG: {:.1}", self.ship.angle.to_degrees() + 90.0),
            90,
            0,
        );

        self.ship.update(GRAV);
        self.surface.draw(self.ship.get_pos().x as i32 - 80, 0);
        self.ship.draw(&self.surface);

        if self.ship.destroyed {
            wasm4::tone(200, 50, 80, wasm4::TONE_NOISE);
            self.is_game_over = true;
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
            self.ship.angle += 0.01;
        }

        if gamepad & wasm4::BUTTON_LEFT != 0 {
            self.ship.angle -= 0.01;
        }

        self.prev_gamepad = gamepad;
        just_pressed
    }
}
