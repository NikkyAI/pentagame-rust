use super::graph::Field;
use super::models::Game;
#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;
#[cfg(feature = "wasm")]
use web_sys::console;

#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn create_game() -> JsValue {
    let new_game = Game { id: 1, state: 0 };

    return JsValue::from_serde(&new_game).unwrap();
}

#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn create_stop(fid: i16, counter: i16, sid: i16) -> JsValue {
    let new_stop = Field {
        owner: Some(1u8),
        occupied: false
    };

    return JsValue::from_serde(&new_stop).unwrap();
}

// This is like the `main` function, except for JavaScript.
#[cfg(feature = "wasm")]
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
