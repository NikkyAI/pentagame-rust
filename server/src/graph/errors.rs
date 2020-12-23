use derive_more::Display;
use diesel::result::Error as DBError;

#[derive(Clone, Debug, PartialEq, Display)]
// Graph operation error
pub enum GraphErr {
    // There is no vertex with the given id in the graph
    NoSuchVertex,

    /*
    Kept for extendability
    Meant for: There is no such edge in the graph
    NoSuchEdge,
    */
    // Could not add an edge to the graph
    CannotAddEdge,

    // Could not add an vertex to the graph
    CannotAddVertex,

    // Couldn't construct State from database
    CannotConstructState(String),
}

impl From<DBError> for GraphErr {
    fn from(error: DBError) -> GraphErr {
        GraphErr::CannotConstructState(error.to_string())
    }
}
