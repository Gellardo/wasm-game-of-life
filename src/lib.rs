use wasm_bindgen::__rt::core::time::Duration;
use wasm_bindgen::__rt::std::thread::sleep;
use wasm_bindgen::prelude::*;
use web_sys::{console, set_timeout_with_callback_and_timeout_and_arguments_0};
use web_sys::Element;

use crate::gameoflife::GameOfLife;

mod gameoflife;

// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

fn get_element_for_drawing() -> Result<Element, JsValue> {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    document.get_element_by_id("game").ok_or(JsValue::from_str("did not find 'game' element"))
}

#[wasm_bindgen]
fn renderloop(element: &Element, mut life: &mut GameOfLife) {
    element.set_text_content(Some(&life.prettier_state()));
    life.tick();
    let window = web_sys::window().unwrap();
    window.set_timeout_with_callback_and_timeout_and_arguments_0(Function::from(|| renderloop(element, life)), 1000);
}

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
        console_error_panic_hook::set_once();


    // Your code goes here!
    console::log_1(&JsValue::from_str("Autostarted main_js!"));
    let mut life = game();
    let game_element = get_element_for_drawing()?;
    renderloop(&game_element, &mut life);
    for i in 0..100 {
        renderloop(&game_element, &mut life);
    }

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
