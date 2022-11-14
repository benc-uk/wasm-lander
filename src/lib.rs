#[cfg(feature = "buddy-alloc")]
mod alloc;
mod game;
mod gfx;
mod polygon;
mod ship;
mod surface;
mod wasm4;

use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    static ref GAME: Mutex<game::Game> = Mutex::new(game::Game::new());
}

#[no_mangle]
fn start() {
    gfx::set_palette([0x000000, 0x004400, 0x008800, 0x00ff00]);
    GAME.lock().expect("").new_game();
}

#[no_mangle]
fn update() {
    GAME.lock().expect("").update();
}
