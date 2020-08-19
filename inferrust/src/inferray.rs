//! In-memory structures to store the RDF graph

mod dictionary;
pub(crate) use self::dictionary::*;

mod graph;
pub use self::graph::*;

mod store;
pub(crate) use self::store::*;
