#[derive(Clone, Debug, PartialEq)]
// Graph operation error
pub enum GraphErr {
    // There is no vertex with the given id in the graph
    NoSuchVertex,

    // There is no such edge in the graph
    NoSuchEdge,

    // Could not add an edge to the graph
    CannotAddEdge,

    // Could not add an vertex to the graph
    CannotAddVertex,
}
