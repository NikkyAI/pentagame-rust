use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

// move (as written in )
type MOVE = [i16; 7];

#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub struct Field {
    pub id: [i16; 3], //  uses i16 to circumvent conversion when constructing moves
    pub size: u8,     // 1: stop 2: junction; corner
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

#[wasm_bindgen]
pub fn create_stop(fid: i16, counter: i16, sid: i16) -> JsValue {
    let new_stop = Field {
        id: [fid, counter, sid],
        size: 1,
    };

    return JsValue::from_serde(&new_stop).unwrap();
}
