use super::models::Field;
use graphlib::{Graph, GraphErr, VertexId};

// vertexmap
const BASE_VERTEX_MAP: [i16; 10] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]; // in case the naming changes these are statically mapped
const EDGE_MAP: [&[(i16, i16)]; 9] = [
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

pub fn construct_graph() -> Result<Graph<Field>, GraphErr> {
    let mut graph: Graph<Field> = Graph::with_capacity(100_usize);
    let mut bmap: [VertexId; 10] = [VertexId::random(); 10];

    // the base nodes (junction, corners) need to be preinserted to do effective EDGE and stop mapping
    for i in 0..BASE_VERTEX_MAP.len() {
        bmap[i] = graph.add_vertex(Field {
            size: 2,
            id: [BASE_VERTEX_MAP[i], 0, 0],
        });
    }

    for index in 0..EDGE_MAP.len() {
        let base_vertex = BASE_VERTEX_MAP[index];
        let f_id = bmap[index];
        for (svertex, vcounter) in EDGE_MAP[index] {
            for count in 0..*vcounter {
                let s_id = graph.add_vertex(Field {
                    size: 1,
                    id: [base_vertex, count, *svertex],
                });
                graph.add_edge(&f_id, &s_id)?;
            }
        }
    }

    return Ok(graph);
}
