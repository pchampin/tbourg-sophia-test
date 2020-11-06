//! Provides type `InfGraph` (other utility types).

use sophia_api::graph::GTripleSource;
use sophia_api::graph::Graph;
use sophia_api::term::TTerm;
use sophia_api::triple::stream::TripleSource;
use sophia_api::triple::streaming_mode::{ByTermRefs, StreamedTriple};
use sophia_term::ArcTerm;

use std::convert::Infallible;

use crate::inferray::NodeDictionary;
use crate::inferray::TripleStore;
use crate::rules::*;
use crate::utils::*;

/// Implementation of `sophia_api::graph::Graph` that supports inferences.
pub struct InfGraph {
    dictionary: NodeDictionary,
    store: TripleStore,
}

impl Graph for InfGraph {
    type Triple = ByTermRefs<ArcTerm>;
    type Error = Infallible;

    fn triples(&self) -> GTripleSource<Self> {
        Box::from(
            self.store
                .chunks()
                .iter()
                .enumerate()
                .filter(|(_, chunk)| !chunk.so().is_empty())
                .map(move |(pi, chunk)| {
                    let p = self
                        .dictionary
                        .get_term(NodeDictionary::offset_to_prop_idx(pi));
                    chunk.so().iter().map(move |[si, oi]| {
                        Ok(StreamedTriple::by_term_refs(
                            self.dictionary.get_term(*si),
                            p,
                            self.dictionary.get_term(*oi),
                        ))
                    })
                })
                .flatten(),
        )
    }

    fn triples_with_s<'s, T>(&'s self, s: &'s T) -> GTripleSource<'s, Self>
    where
        T: TTerm + ?Sized,
    {
        if let Some(si) = self.dictionary.get_index(s) {
            let s = self.dictionary.get_term(si);
            Box::from(
                self.store
                    .chunks()
                    .iter()
                    .enumerate()
                    .filter(|(_, chunk)| !chunk.so().is_empty())
                    .map(move |(pi, chunk)| {
                        let p = self
                            .dictionary
                            .get_term(NodeDictionary::offset_to_prop_idx(pi));
                        let start_index = first_pair(&chunk.so(), si);
                        chunk.so()[start_index..]
                            .iter()
                            .take_while(move |[is, _]| si == *is)
                            .map(move |[_, oi]| {
                                Ok(StreamedTriple::by_term_refs(
                                    s,
                                    p,
                                    self.dictionary.get_term(*oi),
                                ))
                            })
                    })
                    .flatten(),
            )
        } else {
            Box::from(std::iter::empty())
        }
    }

    fn triples_with_p<'s, T>(&'s self, p: &'s T) -> GTripleSource<'s, Self>
    where
        T: TTerm + ?Sized,
    {
        if let Some(ip) = self.dictionary.get_index(p) {
            let idx = NodeDictionary::prop_idx_to_offset(ip);
            let chunk = &self.store.chunks()[idx];
            if !chunk.so().is_empty() {
                let p = self.dictionary.get_term(ip);
                Box::from(chunk.so().iter().map(move |[si, oi]| {
                    Ok(StreamedTriple::by_term_refs(
                        self.dictionary.get_term(*si),
                        p,
                        self.dictionary.get_term(*oi),
                    ))
                }))
            } else {
                Box::from(std::iter::empty())
            }
        } else {
            Box::from(std::iter::empty())
        }
    }

    fn triples_with_o<'s, T>(&'s self, o: &'s T) -> GTripleSource<'s, Self>
    where
        T: TTerm + ?Sized,
    {
        if let Some(oi) = self.dictionary.get_index(o) {
            let o = self.dictionary.get_term(oi);
            Box::from(
                self.store
                    .chunks()
                    .iter()
                    .enumerate()
                    .filter(|(_, chunk)| !chunk.os().is_empty())
                    .map(move |(pi, chunk)| {
                        let p = self
                            .dictionary
                            .get_term(NodeDictionary::offset_to_prop_idx(pi));
                        let start_index = first_pair(&chunk.os(), oi);
                        chunk.os()[start_index..]
                            .iter()
                            .take_while(move |[io, _]| oi == *io)
                            .map(move |[_, si]| {
                                Ok(StreamedTriple::by_term_refs(
                                    self.dictionary.get_term(*si),
                                    p,
                                    o,
                                ))
                            })
                    })
                    .flatten(),
            )
        } else {
            Box::from(std::iter::empty())
        }
    }

    fn triples_with_sp<'s, T, U>(&'s self, s: &'s T, p: &'s U) -> GTripleSource<'s, Self>
    where
        T: TTerm + ?Sized,
        U: TTerm + ?Sized,
    {
        if let (Some(si), Some(pi)) = (self.dictionary.get_index(s), self.dictionary.get_index(p)) {
            let idx = NodeDictionary::prop_idx_to_offset(pi);
            let chunk = &self.store.chunks()[idx];
            if !chunk.so().is_empty() {
                let s = self.dictionary.get_term(si);
                let p = self.dictionary.get_term(pi);
                let start_index = first_pair(&chunk.so(), si);
                Box::from(
                    chunk.so()[start_index..]
                        .iter()
                        .take_while(move |[is, _]| *is == si)
                        .map(move |[_, oi]| {
                            Ok(StreamedTriple::by_term_refs(
                                s,
                                p,
                                self.dictionary.get_term(*oi),
                            ))
                        }),
                )
            } else {
                Box::from(std::iter::empty())
            }
        } else {
            Box::from(std::iter::empty())
        }
    }

    fn triples_with_so<'s, T, U>(&'s self, s: &'s T, o: &'s U) -> GTripleSource<'s, Self>
    where
        T: TTerm + ?Sized,
        U: TTerm + ?Sized,
    {
        if let (Some(si), Some(oi)) = (self.dictionary.get_index(s), self.dictionary.get_index(o)) {
            let s = self.dictionary.get_term(si);
            let o = self.dictionary.get_term(oi);
            Box::from(self.store.chunks().iter().enumerate().filter_map(
                move |(pi, chunk)| {
                    if chunk.so().is_empty() {
                        None
                    } else {
                        if chunk.so().binary_search(&[si, oi]).is_ok() {
                            Some(Ok(StreamedTriple::by_term_refs(
                                s,
                                self.dictionary
                                    .get_term(NodeDictionary::offset_to_prop_idx(pi)),
                                o,
                            )))
                        } else {
                            None
                        }
                    }
                },
            ))
        } else {
            Box::from(std::iter::empty())
        }
    }

    fn triples_with_po<'s, T, U>(&'s self, p: &'s T, o: &'s U) -> GTripleSource<'s, Self>
    where
        T: TTerm + ?Sized,
        U: TTerm + ?Sized,
    {
        if let (Some(pi), Some(oi)) = (self.dictionary.get_index(p), self.dictionary.get_index(o)) {
            let idx = NodeDictionary::prop_idx_to_offset(pi);
            let chunk = &self.store.chunks()[idx];
            if !chunk.os().is_empty() {
                let p = self.dictionary.get_term(pi);
                let o = self.dictionary.get_term(oi);
                let start_index = first_pair(&chunk.os(), oi);
                Box::from(
                    chunk.os()[start_index..]
                        .iter()
                        .take_while(move |[io, _]| *io == oi)
                        .map(move |[_, si]| {
                            Ok(StreamedTriple::by_term_refs(
                                self.dictionary.get_term(*si),
                                p,
                                o,
                            ))
                        }),
                )
            } else {
                Box::from(std::iter::empty())
            }
        } else {
            Box::from(std::iter::empty())
        }
    }

    fn triples_with_spo<'s, T, U, V>(
        &'s self,
        s: &'s T,
        p: &'s U,
        o: &'s V,
    ) -> GTripleSource<'s, Self>
    where
        T: TTerm + ?Sized,
        U: TTerm + ?Sized,
        V: TTerm + ?Sized,
    {
        if let (Some(si), Some(pi), Some(oi)) = (
            self.dictionary.get_index(s),
            self.dictionary.get_index(p),
            self.dictionary.get_index(o),
        ) {
            let idx = NodeDictionary::prop_idx_to_offset(pi);
            let chunk = &self.store.chunks()[idx];
            if chunk.so().is_empty() {
                Box::from(std::iter::empty())
            } else {
                if chunk.so().binary_search(&[si, oi]).is_ok() {
                    let s = self.dictionary.get_term(si);
                    let o = self.dictionary.get_term(oi);
                    let p = self.dictionary.get_term(pi);
                    Box::from(vec![Ok(StreamedTriple::by_term_refs(s, p, o))].into_iter())
                } else {
                    Box::from(std::iter::empty())
                }
            }
        } else {
            Box::from(std::iter::empty())
        }
    }
}

impl InfGraph {
    /// Create a new `InfGraph` from the given triple source,
    /// to which the given inference regime (`profile`) is applied.
    pub fn new<TS>(ts: TS, profile: &RuleProfile) -> Result<Self, TS::Error>
    where
        TS: TripleSource,
    {
        let mut this = InfGraph::new_unprocessed(ts)?;
        this.process(profile);
        Ok(this)
    }

    /// Create a new `InfGraph` from the given triple source,
    /// with the RDFS rule profile.
    pub fn new_rdfs<TS>(ts: TS) -> Result<Self, TS::Error>
    where
        TS: TripleSource,
    {
        Self::new(ts, &RuleProfile::RDFS())
    }

    /// Create a new `InfGraph` from the given triple source,
    /// with the RhoDF rule profile.
    pub fn new_rhodf<TS>(ts: TS) -> Result<Self, TS::Error>
    where
        TS: TripleSource,
    {
        Self::new(ts, &RuleProfile::RhoDF())
    }

    /// Create a new `InfGraph` from the given triple source,
    /// with the RDFS+ rule profile.
    pub fn new_rdfs_plus<TS>(ts: TS) -> Result<Self, TS::Error>
    where
        TS: TripleSource,
    {
        Self::new(ts, &RuleProfile::RDFSPlus())
    }

    /// The total number of triples (explicit + inferred)
    /// in this graph.
    #[inline]
    pub fn size(&self) -> usize {
        self.store.size()
    }

    /// **for benchmarking purposes only**
    ///
    /// Create a new `InfGraph` from the given triple source,
    /// but *do not* finalize the processing.
    /// This graph is **unsuable**
    /// until the `process` method is called.
    ///
    /// This is useful for benchmarking the the loading time (without inferences).
    pub fn new_unprocessed<TS>(mut ts: TS) -> Result<Self, TS::Error>
    where
        TS: TripleSource,
    {
        let mut dictionary = NodeDictionary::new();
        let mut encoded = vec![];
        ts.for_each_triple(|t| {
            let rep = dictionary.encode_triple(&t);
            encoded.push(rep);
        })?;
        dictionary.remap_triples(&mut encoded);
        let store = TripleStore::new(encoded);
        Ok(Self { dictionary, store })
    }

    /// **for benchlarking purposes only**
    ///
    /// Finalizes the processing of a graph created with `new_unprocessed`.
    ///
    /// This is useful for benchmatking the processing time of inferences (without loading).
    pub fn process(&mut self, profile: &RuleProfile) {
        self.compute_transitive_closures(&profile.cl_profile);
        profile.before_rules.process(self);
        if profile.axiomatic_triples {
            self.init_axiomatic_triples();
        }
        profile.rules.process(self);
        match &profile.after_rules {
            Some(func) => {
                self.merge_store(TripleStore::new(func(self)));
            }
            None => (),
        }
        #[cfg(debug_assertions)]
        debug_assert!(self.store.is_sorted());
    }

    /// Borrow the NodeDictionary of this graph.
    #[inline]
    pub(crate) fn dict(&self) -> &NodeDictionary {
        &self.dictionary
    }

    /// Borrow the TripleStore of this graph.
    #[inline]
    pub(crate) fn store(&self) -> &TripleStore {
        &self.store
    }

    /// Merge triples from another store.
    ///
    /// IMPORTANT: there is no reasoning involved when adding those triples;
    /// they are dumpbly added to the ones alteady stored.
    /// Actually, this is the method used internally by rules to add their result.
    #[inline]
    pub(crate) fn merge_store(&mut self, other: TripleStore) {
        self.store.merge(other);
    }

    fn compute_transitive_closures(&mut self, profile: &ClosureProfile) {
        if profile.on_sco {
            self.store.transitive_closure(NodeDictionary::rdfssubClassOf);
        }
        if profile.on_spo {
            self.store.transitive_closure(NodeDictionary::rdfssubPropertyOf);
        }
        if profile.on_sa {
            self.store.transitive_closure(NodeDictionary::owlsameAs);
        }
        if profile.on_trp {
            for tr_idx in self.get_tr_idx() {
                self.store.transitive_closure(tr_idx);
            }
        }
    }

    fn get_tr_idx(&mut self) -> Vec<u32> {
        if let Some(pairs) = self
            .store
            .chunks()
            .get(NodeDictionary::prop_idx_to_offset(
                NodeDictionary::rdftype as u64,
            ))
        {
            pairs
                .so()
                .iter()
                .filter(|pair| pair[1] == NodeDictionary::owltransitiveProperty as u64)
                .map(|pair| pair[0] as u32)
                .collect()
        } else {
            vec![]
        }
    }

    fn init_axiomatic_triples(&mut self) {
        let axiomatic_triples = [[
            NodeDictionary::rdftype as u64,
            NodeDictionary::rdftype as u64,
            NodeDictionary::rdfProperty as u64,
            ],[
            NodeDictionary::rdfsubject as u64,
            NodeDictionary::rdftype as u64,
            NodeDictionary::rdfProperty as u64,
            ],[
            NodeDictionary::rdfpredicate as u64,
            NodeDictionary::rdftype as u64,
            NodeDictionary::rdfProperty as u64,
            ],[
            NodeDictionary::rdfobject as u64,
            NodeDictionary::rdftype as u64,
            NodeDictionary::rdfProperty as u64,
            ],[
            NodeDictionary::rdffirst as u64,
            NodeDictionary::rdftype as u64,
            NodeDictionary::rdfProperty as u64,
            ],[
            NodeDictionary::rdfrest as u64,
            NodeDictionary::rdftype as u64,
            NodeDictionary::rdfProperty as u64,
            ],[
            NodeDictionary::rdfValue as u64,
            NodeDictionary::rdftype as u64,
            NodeDictionary::rdfProperty as u64,
            ],[
            NodeDictionary::rdf_1 as u64,
            NodeDictionary::rdftype as u64,
            NodeDictionary::rdfProperty as u64,
            ],[
            NodeDictionary::rdfnil as u64,
            NodeDictionary::rdftype as u64,
            NodeDictionary::rdfList,
            ],[
        // Domain
            NodeDictionary::rdftype as u64,
            NodeDictionary::rdfsdomain as u64,
            NodeDictionary::rdfsResource,
            ],[
            NodeDictionary::rdfsdomain as u64,
            NodeDictionary::rdfsdomain as u64,
            NodeDictionary::rdfProperty as u64,
            ],[
            NodeDictionary::rdfsrange as u64,
            NodeDictionary::rdfsdomain as u64,
            NodeDictionary::rdfProperty as u64,
            ],[
            NodeDictionary::rdfssubClassOf as u64,
            NodeDictionary::rdfsdomain as u64,
            NodeDictionary::rdfProperty as u64,
            ],[
            NodeDictionary::rdfssubPropertyOf as u64,
            NodeDictionary::rdfsdomain as u64,
            NodeDictionary::rdfProperty as u64,
            ],[
            NodeDictionary::rdfsubject as u64,
            NodeDictionary::rdfsdomain as u64,
            NodeDictionary::rdfStatement,
            ],[
            NodeDictionary::rdfpredicate as u64,
            NodeDictionary::rdfsdomain as u64,
            NodeDictionary::rdfStatement,
            ],[
            NodeDictionary::rdfobject as u64,
            NodeDictionary::rdfsdomain as u64,
            NodeDictionary::rdfStatement,
            ],[
            NodeDictionary::rdfsMember as u64,
            NodeDictionary::rdfsdomain as u64,
            NodeDictionary::rdfsResource,
            ],[
            NodeDictionary::rdffirst as u64,
            NodeDictionary::rdfsdomain as u64,
            NodeDictionary::rdfList,
            ],[
            NodeDictionary::rdfrest as u64,
            NodeDictionary::rdfsdomain as u64,
            NodeDictionary::rdfList,
            ],[
            NodeDictionary::rdfsSeeAlso as u64,
            NodeDictionary::rdfsdomain as u64,
            NodeDictionary::rdfsResource,
            ],[
            NodeDictionary::rdfsisDefinedBy as u64,
            NodeDictionary::rdfsdomain as u64,
            NodeDictionary::rdfsResource,
            ],[
            NodeDictionary::rdfsComment as u64,
            NodeDictionary::rdfsdomain as u64,
            NodeDictionary::rdfsResource,
            ],[
            NodeDictionary::rdfsLabel as u64,
            NodeDictionary::rdfsdomain as u64,
            NodeDictionary::rdfsResource,
            ],[
            NodeDictionary::rdfValue as u64,
            NodeDictionary::rdfsdomain as u64,
            NodeDictionary::rdfsResource,
            ],[
            // Range
            NodeDictionary::rdftype as u64,
            NodeDictionary::rdfsrange as u64,
            NodeDictionary::rdfsClass,
            ],[
                NodeDictionary::rdfsdomain as u64,
            NodeDictionary::rdfsrange as u64,
            NodeDictionary::rdfsClass,
            ],[
            NodeDictionary::rdfsrange as u64,
            NodeDictionary::rdfsrange as u64,
            NodeDictionary::rdfsClass,
            ],[
            NodeDictionary::rdfssubClassOf as u64,
            NodeDictionary::rdfsrange as u64,
            NodeDictionary::rdfsClass,
            ],[
            NodeDictionary::rdfssubPropertyOf as u64,
            NodeDictionary::rdfsrange as u64,
            NodeDictionary::rdfProperty as u64,
            ],[
            NodeDictionary::rdfsubject as u64,
            NodeDictionary::rdfsrange as u64,
            NodeDictionary::rdfsResource,
            ],[
            NodeDictionary::rdfpredicate as u64,
            NodeDictionary::rdfsrange as u64,
            NodeDictionary::rdfsResource,
            ],[
            NodeDictionary::rdfobject as u64,
            NodeDictionary::rdfsrange as u64,
            NodeDictionary::rdfsResource,
            ],[
            NodeDictionary::rdfsMember as u64,
            NodeDictionary::rdfsrange as u64,
            NodeDictionary::rdfsResource,
            ],[
            NodeDictionary::rdffirst as u64,
            NodeDictionary::rdfsrange as u64,
            NodeDictionary::rdfsResource,
            ],[
            NodeDictionary::rdfrest as u64,
            NodeDictionary::rdfsrange as u64,
            NodeDictionary::rdfList,
            ],[
            NodeDictionary::rdfsSeeAlso as u64,
            NodeDictionary::rdfsrange as u64,
            NodeDictionary::rdfsResource,
            ],[
            NodeDictionary::rdfsisDefinedBy as u64,
            NodeDictionary::rdfsrange as u64,
            NodeDictionary::rdfsResource,
            ],[
            NodeDictionary::rdfsComment as u64,
            NodeDictionary::rdfsrange as u64,
            NodeDictionary::rdfsLiteral,
            ],[
            NodeDictionary::rdfsLabel as u64,
            NodeDictionary::rdfsrange as u64,
            NodeDictionary::rdfsLiteral,
            ],[
            NodeDictionary::rdfValue as u64,
            NodeDictionary::rdfsrange as u64,
            NodeDictionary::rdfsResource,
            ],[
            // MISC
            NodeDictionary::rdfAlt,
            NodeDictionary::rdfssubClassOf as u64,
            NodeDictionary::rdfsContainer,
            ],[
            NodeDictionary::rdfBag,
            NodeDictionary::rdfssubClassOf as u64,
            NodeDictionary::rdfsContainer,
            ],[
            NodeDictionary::rdfSeq,
            NodeDictionary::rdfssubClassOf as u64,
            NodeDictionary::rdfsContainer,
            ],[
            NodeDictionary::rdfsContainerMembershipProperty as u64,
            NodeDictionary::rdfssubClassOf as u64,
            NodeDictionary::rdfProperty as u64,
            ],[
            NodeDictionary::rdf_1 as u64,
            NodeDictionary::rdftype as u64,
            NodeDictionary::rdfsContainerMembershipProperty as u64,
            ],[
            NodeDictionary::rdf_1 as u64,
            NodeDictionary::rdfsdomain as u64,
            NodeDictionary::rdfsResource,
            ],[
            NodeDictionary::rdf_1 as u64,
            NodeDictionary::rdfsrange as u64,
            NodeDictionary::rdfsResource,
            ],[
            NodeDictionary::rdfsisDefinedBy as u64,
            NodeDictionary::rdfssubPropertyOf as u64,
            NodeDictionary::rdfsSeeAlso as u64,
            ],[
            NodeDictionary::rdfXMLLiteral,
            NodeDictionary::rdftype as u64,
            NodeDictionary::rdfsDatatype,
            ],[
            NodeDictionary::rdfXMLLiteral,
            NodeDictionary::rdfssubClassOf as u64,
            NodeDictionary::rdfsLiteral,
            ],[
            NodeDictionary::rdfsDatatype,
            NodeDictionary::rdfssubClassOf as u64,
            NodeDictionary::rdfsClass,
            ],[
            NodeDictionary::xsdnonNegativeInteger,
            NodeDictionary::rdftype as u64,
            NodeDictionary::rdfsDatatype,
            ],[
            NodeDictionary::xsdstring,
            NodeDictionary::rdftype as u64,
            NodeDictionary::rdfsDatatype,
            ],[
            NodeDictionary::rdftype as u64,
            NodeDictionary::rdfssubPropertyOf as u64,
            NodeDictionary::rdftype as u64,
            ],[
            NodeDictionary::rdfsdomain as u64,
            NodeDictionary::rdfssubPropertyOf as u64,
            NodeDictionary::rdfsdomain as u64,
            ],[
            NodeDictionary::rdfsrange as u64,
            NodeDictionary::rdfssubPropertyOf as u64,
            NodeDictionary::rdfsrange as u64,
            ],[
            NodeDictionary::rdfssubPropertyOf as u64,
            NodeDictionary::rdfssubPropertyOf as u64,
            NodeDictionary::rdfssubPropertyOf as u64,
            ],[
            NodeDictionary::rdfssubClassOf as u64,
            NodeDictionary::rdfssubPropertyOf as u64,
            NodeDictionary::rdfssubClassOf as u64,
        ]];
        self.merge_store(TripleStore::new(axiomatic_triples.iter().cloned()));
    }
}

