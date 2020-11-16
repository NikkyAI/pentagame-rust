// hash implmentations
use super::errors::GraphErr;
use super::iterators::*;
use super::models::FIELD;
use hashbrown::{HashMap, HashSet};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::hash::{Hash, Hasher};

// Figures are simplified based on demoniation Rules
pub type Figure = u8;

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
    pub edges: HashSet<Edge>,
}

impl Hash for Edge {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.fid.hash(state);
        self.sid.hash(state);
    }
}

impl Edge {
    pub fn new(fid: FIELD, sid: FIELD) -> Edge {
        Edge { fid, sid }
    }

    pub fn matches(&self, f: &FIELD, s: &FIELD) -> bool {
        f == &self.fid && s == &self.sid
    }

    pub fn matches_any(&self, id: &FIELD) -> bool {
        id == &self.fid || id == &self.sid
    }

    /// Returns the second FIELD id
    pub fn second(&self) -> &FIELD {
        &self.sid
    }

    /// Returns the first FIELD id
    pub fn first(&self) -> &FIELD {
        &self.fid
    }
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
            edges: HashSet::new(),
        };
    }

    pub fn fetch(&self, id: &FIELD) -> Option<Field> {
        self.vertices.get(&id)
    }

    pub fn add_edge(&mut self, fid: FIELD, sid: FIELD) -> bool {
        self.edges.insert(Edge::new(fid, sid))
    }

    pub fn add_vertex(&mut self, id: FIELD, field: Field) -> FIELD {
        self.vertices.insert(id, field);

        id
    }

    pub fn dijkstra<'a>(&'a self, src: &'a FIELD, dest: &'a FIELD) -> VertexIter<'a> {
        if let Some(dijkstra) = Dijkstra::new(&self, src).ok() {
            if let Some(iter) = dijkstra.get_path_to(dest).ok() {
                iter
            } else {
                VertexIter(Box::new(iter::empty()))
            }
        } else {
            VertexIter(Box::new(iter::empty()))
        }
    }
}
