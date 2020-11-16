use super::graph::{Field, Graph, BASE_VERTEX_MAP, EDGE_MAP};
use serde::{Deserialize, Serialize};
use std::error::Error;

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

    pub fn construct_graph() -> Result<Graph, Box<dyn Error>> {
        let mut graph: Graph = Graph::new(100_usize);
        let mut bmap: [FIELD; 10] = [[0, 0, 0]; 10];

        // the base nodes (junction, corners) need to be preinserted to do effective EDGE and stop mapping
        for i in 0..BASE_VERTEX_MAP.len() {
            bmap[i] = graph.add_vertex([BASE_VERTEX_MAP[i], 0, 0], Field::new(false, None));
        }

        for index in 0..EDGE_MAP.len() {
            let base_vertex = BASE_VERTEX_MAP[index];
            let f_id = bmap[index];
            for (svertex, vcounter) in EDGE_MAP[index] {
                for count in 0..*vcounter {
                    let s_id =
                        graph.add_vertex([base_vertex, count, *svertex], Field::new(false, None));
                    graph.add_edge(f_id, s_id);
                }
            }
        }

        return Ok(graph);
    }

    pub fn test() {
        let g = Game::construct_graph().expect("This shouldn't crash");

        println!("V: {:?}", g.vertices.len());
        println!("E: {:?}", g.edges.len());
        let keys = g.vertices.keys().into_iter();
        let mut buffer: Vec<[i16; 3]> = Vec::new();
        buffer.extend(keys);
        buffer.sort();
        for key in buffer {
            println!("{:?}", key);
        }
    }
}
