use super::graph::Graph;
use serde::{Deserialize, Serialize};

// types
pub type MOVE = [i16; 7];
pub type FIELD = [i16; 3];

// game struct is more or less only for internal reflection as graphs don't need optional metadata
#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub struct Game {
    pub id: i32,
    pub state: i16,
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
