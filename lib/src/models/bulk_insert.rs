use crate::{util::Key, EdgeKey, Identifier, Vertex};

/// An item to insert, as part of a bulk insert request.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BulkInsertItem {
    /// A vertex to insert.
    Vertex(Vertex),
    /// An edge to insert.
    Edge(EdgeKey),
    /// A vertex property to insert.
    VertexProperty(Key, Identifier, Vec<u8>),
    /// An edge property to insert.
    EdgeProperty(EdgeKey, Identifier, Vec<u8>),
}
