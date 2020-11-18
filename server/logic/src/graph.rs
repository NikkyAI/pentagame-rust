// hash implmentations
use super::errors::GraphErr;
use super::models::FIELD;
use hashbrown::HashMap;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::fmt::Debug;

// Figures are simplified based on demoniation Rules
pub type Figure = u8;
type Vertex = (FIELD, Field);

// vertexmap
pub const BASE_VERTEX_MAP: [i16; 10] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]; // in case the naming changes these are statically mapped
pub const EDGE_MAP: [&[(i16, i16)]; 9] = [
    &[(1, 3), (4, 3), (5, 6), (6, 6)],
    &[(2, 3), (6, 6), (7, 6)],
    &[(3, 3), (8, 6), (7, 6)],
    &[(4, 3), (8, 6), (9, 6)],
    &[(6, 5), (9, 6)],
    &[(9, 3), (6, 3)],
    &[(7, 3)],
    &[(8, 3)],
    &[(9, 3)],
]; // the 10th node can be left out at this point

#[derive(Deserialize, Serialize, PartialEq, Eq, Debug, Clone, Copy)]
pub struct Field {
    pub occupied: bool,
    pub owner: Option<Figure>,
}

#[derive(Deserialize, Serialize, PartialEq, Eq, Debug, Clone, Copy)]
pub struct Edge {
    sid: FIELD,
    fid: FIELD,
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Graph {
    /// Mapping of vertex ids and vertex values
    pub vertices: HashMap<FIELD, Field>,

    // set for edges (doesn't require any weights)
    pub edges: HashMap<FIELD, Vec<FIELD>>,
}

impl Field {
    pub fn new(occupied: bool, owner: Option<Figure>) -> Field {
        Field { occupied, owner }
    }
}

impl Graph {
    pub fn new(size: usize) -> Graph {
        return Graph {
            vertices: HashMap::with_capacity(size),
            edges: HashMap::new(),
        };
    }

    pub fn shrink_to_fit(&mut self) {
        self.vertices.shrink_to_fit();
        self.edges.shrink_to_fit();
    }

    pub fn fetch(&self, id: FIELD) -> Option<&Field> {
        self.vertices.get(&id)
    }

    pub fn add_edge(&mut self, fid: FIELD, sid: FIELD) -> Result<(), GraphErr> {
        // get existing vertex from edgemap
        let old = match self.edges.get_mut(&fid) {
            Some(vertex) => {
                vertex.push(sid);
                vertex.to_owned()
            }
            None => {
                let sids = vec![sid];
                self.edges.insert(fid, sids.clone());
                sids.to_owned()
            }
        };

        // add new edge and update edgemap
        self.edges.insert(fid, old).unwrap();

        Ok(())
    }

    pub fn add_vertex(&mut self, id: FIELD, field: Field) -> Result<FIELD, GraphErr> {
        match self.vertices.insert(id, field) {
            Some(_) => Err(GraphErr::CannotAddVertex {}),
            None => Ok(id),
        }
    }

    pub fn validate<'a>(&'a self, src: &'a FIELD, dest: &'a FIELD) -> Result<(bool, u8), GraphErr> {
        // check if specified vertices exists
        let source = match self.vertices.get(src) {
            Some(vertex) => (vertex, src),
            None => {
                return Err(GraphErr::NoSuchVertex {});
            }
        };

        let destination = match self.vertices.get(dest) {
            Some(vertex) => (vertex, dest),
            None => {
                return Err(GraphErr::NoSuchVertex {});
            }
        };

        // test with dijkstra if there's a possible path
        match self.bfs(source, destination) {
            Some(collider) => Ok((true, collider)),
            None => Ok((false, 0)),
        }
    }

    fn bfs<'a>(&'a self, src: (&'a Field, &'a FIELD), dest: (&'a Field, &'a FIELD)) -> Option<u8> {
        // based off https://www.geeksforgeeks.org/shortest-path-unweighted-graph/
        // create static maps and lists for bfs algorithm
        // This uses hashmaps primarly to accelerate search
        let v = self.vertices.len();
        /*
        let mut predecessor: HashMap<FIELD, Vec<FIELD>> = HashMap::with_capacity(v);
        let mut distances: HashMap<FIELD, u16> = HashMap::with_capacity(v);
        */
        let mut visited: HashMap<FIELD, bool> = HashMap::with_capacity(v);
        let mut queue: VecDeque<Vertex> = VecDeque::with_capacity(v);

        self.vertices.iter().for_each(|(F, f)| {
            visited.insert(*F, f.occupied);
            /*
            distances.insert(*F, u16::MAX);
            */
            queue.push_back((F.clone(), f.clone()));
        });

        // mark src as already visited
        visited.insert(*src.1, true); // this is just in case
        queue.push_back((*src.1, *src.0));

        println!("Size: {}", queue.len());
        while !queue.is_empty() {
            let (F, _) = queue.pop_front().unwrap();
            let edges = self.edges.get(&F).unwrap();
            for edge in edges.clone() {
                let eq = edge == *dest.1;
                if !visited[&edge] || eq {
                    visited.insert(edge, true);
                    /*
                    distances[edge] = distances[&F] + 1;
                    predecessor.get_mut(edge).unwrap().push(F);
                    */

                    if eq {
                        return dest.0.owner;
                    }
                }
            }
        }

        None
    }
}
