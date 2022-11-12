use crate::wasm4;

pub fn set_palette(palette: [u32; 4]) {
    unsafe {
        *wasm4::PALETTE = palette;
    }
}

pub fn set_draw_color(idx: u16) {
    unsafe { *wasm4::DRAW_COLORS = idx }
}

pub fn shadow_text(text_v: &str, x: i32, y: i32, sc: u16, dc: u16) {
    set_draw_color(dc);
    wasm4::text(text_v, x - 1, y + 1);
    set_draw_color(sc);
    wasm4::text(text_v, x, y);
}
