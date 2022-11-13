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
    gfx::set_palette([0x051f39, 0x4a2480, 0xc53a9d, 0xff8e80]);
    GAME.lock().expect("").new_game();
}

#[no_mangle]
fn update() {
    GAME.lock().expect("").update();
}
