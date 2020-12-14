use super::graph::{Figure, Graph};
use crate::api::errors::APIError;
use hashbrown::HashMap;
use serde::{Deserialize, Serialize};
use serde_json::from_str;

// types
// i16 is used to be translatable to PG SMALL INT
pub type MOVE = ([i16; 6], Figure);
pub type AMOVE = [i16; 7]; // absolute move
pub type FIELD = [i16; 3];
pub type LOCATION = ([i16; 3], Figure);

// game struct is more or less only for internal reflection as graphs don't need optional metadata
#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub struct Game {
    pub id: i32,
    pub state: i16,
}

#[derive(Deserialize, Serialize, Debug, PartialOrd, PartialEq)]
pub struct Move {
    pub action: MOVE,
}

impl Game {
    pub fn new(id: i32, state: i16) -> Game {
        Game { id, state }
    }

    pub fn test() {
        let mut g = Graph::construct_graph().expect("This shouldn't crash");

        println!("E: {:?}", g.edges.len());
        println!("V: {:?}", g.vertices.len());
        let mut keys = g.edges.iter().collect::<Vec<(&FIELD, &Vec<FIELD>)>>();
        keys.sort();
        for key in keys {
            println!("{:?}: {:?}", key.0, key.1);
        }

        let (first, second) = ([0, 0, 0], [6, 0, 0]);
        let sv = g.vertices.get_mut(&second).unwrap();
        sv.owner = Some(1_u8);

        let res = g
            .validate(&first, &second)
            .expect("This shouldn't throw any error");

        println!("Result was {:?} and collided with {}", res.0, res.1);
    }
}

impl Move {
    pub fn from_action(action: HashMap<String, String>) -> Result<Move, APIError> {
        let mut data: MOVE = ([0_i16; 6], u8::MAX);

        data.1 = match action.get("figure") {
            Some(raw_id) => match raw_id.parse::<u8>() {
                Ok(id) => id,
                Err(_) => {
                    return Err(APIError::ValidationError(
                        "Value for field figure doesn't fit into u8".to_owned(),
                    ));
                }
            },
            None => {
                return Err(APIError::ValidationError("Missing field figure".to_owned()));
            }
        };

        data.0 = match action.get("move") {
            Some(raw_move) => match from_str::<[i16; 6]>(raw_move) {
                Ok(parsed_move) => parsed_move,
                Err(_) => {
                    return Err(APIError::ValidationError(
                        "Value for field move doesn't fit into [i16; 6]".to_owned(),
                    ));
                }
            },
            None => {
                return Err(APIError::ValidationError("Missing field move".to_owned()));
            }
        };

        Ok(Move { action: data })
    }
}
