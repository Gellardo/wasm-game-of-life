use wasm_bindgen::prelude::*;
use web_sys::console;

use crate::gameoflife::GameOfLife;

mod gameoflife;

// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
        console_error_panic_hook::set_once();


    // Your code goes here!
    console::log_1(&JsValue::from_str("Autostarted main_js!"));

    Ok(())
}

#[wasm_bindgen]
pub fn game() -> GameOfLife {
    let mut game = gameoflife::GameOfLife::new();
    game.set_alive(1, 2);
    game.set_alive(2, 3);
    game.set_alive(3, 1);
    game.set_alive(3, 2);
    game.set_alive(3, 3);
    game
}
