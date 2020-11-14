use serde::{Deserialize, Serialize};

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
