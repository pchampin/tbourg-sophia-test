//! This crate is an adaptation of [Inferray] in Rust \[1], based on the crate [Sophia] \[2].
//!
//! [Inferray]:http://www.vldb.org/pvldb/vol9/p468-subercaze.pdf
//! [Sophia]:https://github.com/pchampin/sophia_rs
//!
//! # Getting started
//!
//! Here a quick example on how to build a graph (using [Sophia parser]), and launch the reasoner.
//!
//! ```
//!
//! use inferrust::*;
//!
//! let rep = r#"
//! @prefix : <http://example.org/> .
//! @prefix rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#> .
//! @prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
//! @prefix owl: <http://www.w3.org/2002/07/owl#> .
//!
//! :bart rdf:type :Boy .
//! :lisa rdf:type :Girl .
//! :Boy rdfs:subClassOf :Male .
//! :Girl rdfs:subClassOf :Female .
//! :Female rdfs:subClassOf :Person .
//! :Male rdfs:subClassOf :Person .
//! "#;
//!
//! let mut graph = InfGraph::new(
//!     sophia::parser::turtle::parse_str(rep),
//!     &mut RuleProfile::RDFS(),
//! );
//! ```
//!
//! [Sophia parser]:https://docs.rs/sophia/0.6.1/sophia/parser/index.html
//!
//! ## References
//! \[1] Julien Subercaze, Christophe Gravier, Jules Chevalier, Frédérique Laforest:
//! Inferray: fast in-memory RDF inference. PVLDB 9(6): 468-479 (2016)
//!
//! \[2] Champin, P.-A. (2020) ‘Sophia: A Linked Data and Semantic Web toolkit for Rust’, in Wilde, E. and Amundsen, M. (eds).
//! The Web Conference 2020: Developers Track, Taipei, TW.

mod closure;
mod inferray;
mod rules;
mod utils;

pub use inferray::InfGraph;
pub use rules::RuleProfile;

#[cfg(test)]
mod test;