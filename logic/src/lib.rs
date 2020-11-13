use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use web_sys::console;

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    // Your code goes here!
    console::log_1(&JsValue::from_str("Hello world!"));

    Ok(())
}

// game struct is more or less only for internal reflection as graphs don't need optional metadata
#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub struct Game {
    pub id: i32,
    pub state: i16,
}

#[wasm_bindgen]
pub fn create_game() -> JsValue {
    let new_game = Game { id: 1, state: 0 };

    return JsValue::from_serde(&new_game).unwrap();
}
