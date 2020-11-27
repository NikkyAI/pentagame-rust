use super::models::Game;
#[cfg(feature = "wasm")]
use wasm_bindgen::{prelude::*, JsCast};
#[cfg(feature = "wasm")]
use web_sys::{ErrorEvent, MessageEvent, WebSocket};

#[cfg(feature = "wasm")]
macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[cfg(feature = "wasm")]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn create_game() -> JsValue {
    let new_game = Game { id: 1, state: 0 };

    return JsValue::from_serde(&new_game).unwrap();
}

// This is like the `main` function, except for JavaScript.
#[cfg(feature = "wasm")]
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    // Your code goes here!
    console_log!("Hello world!");

    Ok(())
}
