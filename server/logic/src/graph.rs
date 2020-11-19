// hash implmentations
use super::errors::GraphErr;
use super::models::FIELD;
use hashbrown::{HashMap, HashSet};
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::fmt::Debug;

// Figures are simplified based on demoniation Rules
pub type Figure = u8;

// vertexmap
pub const BASE_VERTEX_MAP: [i16; 10] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]; // in case the naming changes these are statically mapped
pub const EDGE_MAP: [&[(i16, i16)]; 10] = [
    &[(1, 3), (4, 3), (5, 6), (6, 6)],
    &[(2, 3), (6, 6), (7, 6)],
    &[(3, 3), (8, 6), (7, 6)],
    &[(4, 3), (8, 6)],
    &[(6, 5), (9, 6)],
    &[(9, 3), (6, 3)],
    &[(7, 3)],
    &[(8, 3)],
    &[(9, 3)],
    &[(6, 6)],
];

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
        // Simple Breadth first search
        // recommended explanation: https://www.programiz.com/dsa/graph-bfs

        // WARNING: This will fail if dest.1.owner == None
        let v = self.vertices.len();
        /*
        Add this if you want to use this with more information
        I may move to logic to an indepent crate and this will be included in a
        seprate method
        let mut predecessor: HashMap<FIELD, Vec<FIELD>> = HashMap::with_capacity(v);
        let mut distances: HashMap<FIELD, u16> = HashMap::with_capacity(v);
        */
        let mut visited: HashSet<FIELD> = HashSet::with_capacity(v);
        let mut queue: VecDeque<FIELD> = VecDeque::with_capacity(v);

        // This step can't be skipped as the 'real' game field may have already occupied fields
        self.vertices.iter().for_each(|(id, f)| {
            // mark occupied fields as already visited to prevent them from counting
            if f.occupied {
                visited.insert(*id);
            }

            /*
            distances.insert(*id, u16::MAX);
            */
        });

        // mark src as already visited
        visited.insert(*src.1);
        // ^ this is just in case occupation on the source node wasn't handled
        queue.push_back(*src.1);

        while !queue.is_empty() {
            let id = queue.pop_front().unwrap();
            println!("{:?}", id);
            let edges = self.edges.get(&id).unwrap();
            for edge in edges {
                let eq = edge == dest.1;
                println!("QSize: {}", queue.len());
                println!("Visited: {:?}", visited);
                if eq {
                    return dest.0.owner;
                } else if !visited.contains(edge) {
                    visited.insert(*edge);
                    queue.push_back(*edge);
                    /*
                    distances[edge] = distances[id] + 1;
                    predecessor.get_mut(edge).unwrap().push(F);
                    */
                }
            }
        }

        None
    }

    pub fn construct_graph() -> Result<Graph, GraphErr> {
        let mut graph: Graph = Graph::new(100_usize);
        let mut bmap: [FIELD; 10] = [[0, 0, 0]; 10];

        // the base nodes (junction, corners) need to be preinserted to do effective EDGE and stop mapping
        for i in 0..BASE_VERTEX_MAP.len() {
            bmap[i] = graph.add_vertex([BASE_VERTEX_MAP[i], 0, 0], Field::new(false, None))?;
        }

        // construct edges from edgemap. See pentagraph (python)
        for index in 0..EDGE_MAP.len() {
            let base_vertex = BASE_VERTEX_MAP[index];
            let f_id = bmap[index];
            for (svertex, vcounter) in EDGE_MAP[index] {
                let mut s_id =
                    graph.add_vertex([base_vertex, 1, *svertex], Field::new(false, None))?;
                let mut t_id = s_id; // This value is just to prevent warnings
                graph.add_edge(f_id, s_id)?;
                for count in 2..vcounter + 1 {
                    t_id = graph
                        .add_vertex([base_vertex, count, *svertex], Field::new(false, None))?;
                    graph.add_edge(t_id, s_id)?;
                    graph.add_edge(s_id, t_id)?;
                    s_id = t_id;
                }
                graph.add_edge(t_id, [*svertex, 0, 0])?;
            }
        }

        graph.shrink_to_fit();

        return Ok(graph);
    }
}
