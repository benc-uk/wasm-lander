use crate::gfx;
use crate::ship;
use crate::surface;
use crate::wasm4;

pub struct Game {
    //rng: Rng,
    frame_count: u32,
    prev_gamepad: u8,
    is_game_over: bool,
    is_title_screen: bool,
    surface: surface::Surface,
    camera_x: i32,

    ship: ship::Ship,
}

const GRAV: f64 = 0.0006;

impl Game {
    pub fn new() -> Self {
        Self {
            frame_count: 0,
            prev_gamepad: 0,
            is_game_over: false,
            is_title_screen: false,
            surface: surface::Surface::new(),
            camera_x: 0,
            ship: ship::Ship::new(),
        }
    }

    pub fn new_game(&mut self) {
        self.is_game_over = false;
        self.frame_count = 0;
        self.prev_gamepad = 0;
        self.surface.set_heights();
        self.ship = ship::Ship::new();
    }

    pub fn update(&mut self) {
        let pressed = self.input();

        if self.is_title_screen {
            if pressed & wasm4::BUTTON_1 != 0 {
                self.new_game();
                self.is_title_screen = false;
            }

            gfx::shadow_text("WASM LANDER", 20, 20, 0x4, 0x2);
            return;
        }

        if self.is_game_over && !self.is_title_screen {
            if pressed & wasm4::BUTTON_1 != 0 {
                self.is_title_screen = true;
            }
            gfx::shadow_text("GAME OVER!", 20, 20, 0x4, 0x2);
            return;
        }

        self.frame_count += 1;

        self.ship.update(pressed, GRAV);
        self.surface.draw(self.camera_x, 0);
        self.ship.draw(&self.surface);

        if self.ship.get_fuel() <= 0.0 {
            self.is_game_over = true;
        }
        if self.ship.is_destroyed() {
            self.is_game_over = true;
        }

        gfx::set_draw_color(0x2);
        wasm4::text(format!("FUEL: {:.1}", self.ship.get_fuel()), 0, 0);
    }

    pub fn input(&mut self) -> u8 {
        let gamepad = unsafe { *wasm4::GAMEPAD1 };
        let just_pressed = gamepad & (gamepad ^ self.prev_gamepad);

        self.ship.set_engines(false);
        if gamepad & wasm4::BUTTON_1 != 0 {
            self.ship.set_engines(true);
        }

        self.prev_gamepad = gamepad;
        return just_pressed;
    }
}
