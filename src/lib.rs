#[cfg(feature = "buddy-alloc")]
// Game modules
mod game;
mod gfx;
mod particle;
mod polygon;
mod rand_tab;
mod ship;
mod surface;

// These were auto-generated by WASM4
mod alloc;
mod wasm4;

use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    static ref GAME: Mutex<game::Game> = Mutex::new(game::Game::new());
}

#[no_mangle]
fn start() {
    gfx::set_palette([0x000000, 0x004400, 0x008800, 0x00ff00]);
    GAME.lock().expect("").new_game(0);
}

#[no_mangle]
fn update() {
    GAME.lock().expect("").update();
}
