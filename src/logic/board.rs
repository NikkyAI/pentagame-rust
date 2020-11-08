use graphlib::{Graph, VertexId};
use serde::{Deserialize, Serialize};

// vertexmap
const BASE_VERTEX_MAP: [i16; 10] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]; // in case the naming changes these are statically mapped
const EDGE_MAP: [Vec<(i16, i16)>; 9] = [
    vec![(1, 3), (4, 3), (5, 6), (6, 6)],
    vec![(2, 3), (6, 6), (7, 6)],
    vec![(3, 3), (8, 6), (7, 6)],
    vec![(4, 3), (8, 6), (9, 6)],
    vec![(6, 5), (9, 6)],
    vec![(9, 3), (6, 3)],
    vec![(7, 3)],
    vec![(8, 3)],
    vec![(9, 3)],
]; // the '9' node can be left out at this point

#[derive(Deserialize, Serialize, PartialEq, Debug)]
struct Field {
    id: [i16; 3], //  uses i16 to circumvent conversion when constructing moves
    size: u8,     // 1: stop 2: junction; corner
}

pub fn construct_graph() -> Graph<Field> {
    let mut graph: Graph<Field> = Graph::with_capacity(100_usize);
    let bmap: [VertexId; 10] = [VertexId::random(); 10];

    // the base nodes (junction, corners) need to be preinserted to do effective EDGE and stop mapping
    for i in 0..BASE_VERTEX_MAP.len() {
        bmap[i] = graph.add_vertex(Field {
            size: 2,
            id: [BASE_VERTEX_MAP[i], 0, 0],
        });
    }

    for index in 0..EDGE_MAP.len() {
        let base_vertex = BASE_VERTEX_MAP[index];
        let fid = bmap[index];
        for (svertex, vcounter) in EDGE_MAP[index] {
            for count in 0..vcounter {
                let sid = graph.add_vertex(Field {
                    size: 1,
                    id: [base_vertex, count, svertex],
                });
                graph.add_edge(&fid, &sid);
            }
            graph.add_edge(&fid);
        }
    }

    return graph;
}
