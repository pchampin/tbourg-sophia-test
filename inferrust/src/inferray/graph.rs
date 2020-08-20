use sophia::graph::GTripleSource;
use sophia::graph::Graph;
use sophia::term::{ArcTerm, TTerm};
use sophia::triple::stream::TripleSource;
use sophia::triple::streaming_mode::{ByTermRefs, StreamedTriple};

use std::convert::Infallible;

use crate::closure::*;
use crate::inferray::NodeDictionary;
use crate::inferray::TripleStore;
use crate::rules::*;
use crate::utils::*;

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
                .elem()
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
                    .elem()
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
            let chunk = &self.store.elem()[idx];
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
                    .elem()
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
            let chunk = &self.store.elem()[idx];
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
            Box::from(self.store.elem().iter().enumerate().filter_map(
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
            let chunk = &self.store.elem()[idx];
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
            let chunk = &self.store.elem()[idx];
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
    #[inline]
    pub fn dict(&self) -> &NodeDictionary {
        &self.dictionary
    }

    #[inline]
    pub fn dict_mut(&mut self) -> &mut NodeDictionary {
        &mut self.dictionary
    }

    #[inline]
    pub fn store(&self) -> &TripleStore {
        &self.store
    }
    #[inline]
    pub fn store_mut(&mut self) -> &mut TripleStore {
        &mut self.store
    }
    #[inline]
    pub fn merge_store(&mut self, mut other: TripleStore) {
        other.sort();
        self.store.merge(other);
        self.store.remap_res_to_prop(self.dictionary.remapped());
    }

    pub fn size(&mut self) -> usize {
        self.store.size()
    }

    pub fn process(&mut self, profile: &mut RuleProfile) {
        self.store.sort();
        self.close(&mut profile.cl_profile);
        profile.before_rules.process(self);
        if profile.axiomatic_triples {
            self.init_axiomatic_triples();
        }
        profile.rules.process(self);
        match &profile.after_rules {
            Some(rule) => {
                rule(self);
                self.store.sort();
            }
            None => (),
        }
    }

    fn close(&mut self, profile: &mut ClosureProfile) {
        if profile.on_sco {
            self.close_on(NodeDictionary::rdfssubClassOf);
        }
        if profile.on_spo {
            self.close_on(NodeDictionary::rdfssubPropertyOf);
        }
        if profile.on_sa {
            self.close_on(NodeDictionary::owlsameAs);
        }
        if profile.on_trp {
            for tr_idx in self.get_tr_idx() {
                self.close_on(tr_idx);
            }
        }
    }

    fn close_on(&mut self, index: u32) {
        let offset = NodeDictionary::prop_idx_to_offset(index as u64);
        let pairs = self.store.elem().get(offset);
        if pairs == None {
            return;
        }
        let pairs = pairs.unwrap().so().clone();
        if pairs.is_empty() {
            return;
        }
        let mut tc_g = ClosureGraph::from(pairs);
        let closure = tc_g.close();
        for (s, os) in closure.iter() {
            for o in os.iter() {
                self.store.add_triple_raw(*s, offset, *o);
            }
        }
        self.store.sort();
    }

    fn get_tr_idx(&mut self) -> Vec<u32> {
        if let Some(pairs) = self
            .store
            .elem()
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

    pub fn init_axiomatic_triples(&mut self) {
        self.store.add_triple([
            NodeDictionary::rdftype as u64,
            NodeDictionary::rdftype as u64,
            NodeDictionary::rdfProperty as u64,
        ]);
        self.store.add_triple([
            NodeDictionary::rdfsubject as u64,
            NodeDictionary::rdftype as u64,
            NodeDictionary::rdfProperty as u64,
        ]);
        self.store.add_triple([
            NodeDictionary::rdfpredicate as u64,
            NodeDictionary::rdftype as u64,
            NodeDictionary::rdfProperty as u64,
        ]);
        self.store.add_triple([
            NodeDictionary::rdfobject as u64,
            NodeDictionary::rdftype as u64,
            NodeDictionary::rdfProperty as u64,
        ]);
        self.store.add_triple([
            NodeDictionary::rdffirst as u64,
            NodeDictionary::rdftype as u64,
            NodeDictionary::rdfProperty as u64,
        ]);
        self.store.add_triple([
            NodeDictionary::rdfrest as u64,
            NodeDictionary::rdftype as u64,
            NodeDictionary::rdfProperty as u64,
        ]);
        self.store.add_triple([
            NodeDictionary::rdfValue as u64,
            NodeDictionary::rdftype as u64,
            NodeDictionary::rdfProperty as u64,
        ]);
        self.store.add_triple([
            NodeDictionary::rdf_1 as u64,
            NodeDictionary::rdftype as u64,
            NodeDictionary::rdfProperty as u64,
        ]);
        self.store.add_triple([
            NodeDictionary::rdfnil as u64,
            NodeDictionary::rdftype as u64,
            NodeDictionary::rdfList,
        ]);
        // Domain
        self.store.add_triple([
            NodeDictionary::rdftype as u64,
            NodeDictionary::rdfsdomain as u64,
            NodeDictionary::rdfsResource,
        ]);
        self.store.add_triple([
            NodeDictionary::rdfsdomain as u64,
            NodeDictionary::rdfsdomain as u64,
            NodeDictionary::rdfProperty as u64,
        ]);
        self.store.add_triple([
            NodeDictionary::rdfsrange as u64,
            NodeDictionary::rdfsdomain as u64,
            NodeDictionary::rdfProperty as u64,
        ]);
        self.store.add_triple([
            NodeDictionary::rdfssubClassOf as u64,
            NodeDictionary::rdfsdomain as u64,
            NodeDictionary::rdfProperty as u64,
        ]);
        self.store.add_triple([
            NodeDictionary::rdfssubPropertyOf as u64,
            NodeDictionary::rdfsdomain as u64,
            NodeDictionary::rdfProperty as u64,
        ]);
        self.store.add_triple([
            NodeDictionary::rdfsubject as u64,
            NodeDictionary::rdfsdomain as u64,
            NodeDictionary::rdfStatement,
        ]);
        self.store.add_triple([
            NodeDictionary::rdfpredicate as u64,
            NodeDictionary::rdfsdomain as u64,
            NodeDictionary::rdfStatement,
        ]);
        self.store.add_triple([
            NodeDictionary::rdfobject as u64,
            NodeDictionary::rdfsdomain as u64,
            NodeDictionary::rdfStatement,
        ]);
        self.store.add_triple([
            NodeDictionary::rdfsMember as u64,
            NodeDictionary::rdfsdomain as u64,
            NodeDictionary::rdfsResource,
        ]);
        self.store.add_triple([
            NodeDictionary::rdffirst as u64,
            NodeDictionary::rdfsdomain as u64,
            NodeDictionary::rdfList,
        ]);
        self.store.add_triple([
            NodeDictionary::rdfrest as u64,
            NodeDictionary::rdfsdomain as u64,
            NodeDictionary::rdfList,
        ]);
        self.store.add_triple([
            NodeDictionary::rdfsSeeAlso as u64,
            NodeDictionary::rdfsdomain as u64,
            NodeDictionary::rdfsResource,
        ]);
        self.store.add_triple([
            NodeDictionary::rdfsisDefinedBy as u64,
            NodeDictionary::rdfsdomain as u64,
            NodeDictionary::rdfsResource,
        ]);
        self.store.add_triple([
            NodeDictionary::rdfsComment as u64,
            NodeDictionary::rdfsdomain as u64,
            NodeDictionary::rdfsResource,
        ]);
        self.store.add_triple([
            NodeDictionary::rdfsLabel as u64,
            NodeDictionary::rdfsdomain as u64,
            NodeDictionary::rdfsResource,
        ]);
        self.store.add_triple([
            NodeDictionary::rdfValue as u64,
            NodeDictionary::rdfsdomain as u64,
            NodeDictionary::rdfsResource,
        ]);
        // Range
        self.store.add_triple([
            NodeDictionary::rdftype as u64,
            NodeDictionary::rdfsrange as u64,
            NodeDictionary::rdfsClass,
        ]);
        self.store.add_triple([
            NodeDictionary::rdfsdomain as u64,
            NodeDictionary::rdfsrange as u64,
            NodeDictionary::rdfsClass,
        ]);
        self.store.add_triple([
            NodeDictionary::rdfsrange as u64,
            NodeDictionary::rdfsrange as u64,
            NodeDictionary::rdfsClass,
        ]);
        self.store.add_triple([
            NodeDictionary::rdfssubClassOf as u64,
            NodeDictionary::rdfsrange as u64,
            NodeDictionary::rdfsClass,
        ]);
        self.store.add_triple([
            NodeDictionary::rdfssubPropertyOf as u64,
            NodeDictionary::rdfsrange as u64,
            NodeDictionary::rdfProperty as u64,
        ]);
        self.store.add_triple([
            NodeDictionary::rdfsubject as u64,
            NodeDictionary::rdfsrange as u64,
            NodeDictionary::rdfsResource,
        ]);
        self.store.add_triple([
            NodeDictionary::rdfpredicate as u64,
            NodeDictionary::rdfsrange as u64,
            NodeDictionary::rdfsResource,
        ]);
        self.store.add_triple([
            NodeDictionary::rdfobject as u64,
            NodeDictionary::rdfsrange as u64,
            NodeDictionary::rdfsResource,
        ]);
        self.store.add_triple([
            NodeDictionary::rdfsMember as u64,
            NodeDictionary::rdfsrange as u64,
            NodeDictionary::rdfsResource,
        ]);
        self.store.add_triple([
            NodeDictionary::rdffirst as u64,
            NodeDictionary::rdfsrange as u64,
            NodeDictionary::rdfsResource,
        ]);
        self.store.add_triple([
            NodeDictionary::rdfrest as u64,
            NodeDictionary::rdfsrange as u64,
            NodeDictionary::rdfList,
        ]);
        self.store.add_triple([
            NodeDictionary::rdfsSeeAlso as u64,
            NodeDictionary::rdfsrange as u64,
            NodeDictionary::rdfsResource,
        ]);
        self.store.add_triple([
            NodeDictionary::rdfsisDefinedBy as u64,
            NodeDictionary::rdfsrange as u64,
            NodeDictionary::rdfsResource,
        ]);
        self.store.add_triple([
            NodeDictionary::rdfsComment as u64,
            NodeDictionary::rdfsrange as u64,
            NodeDictionary::rdfsLiteral,
        ]);
        self.store.add_triple([
            NodeDictionary::rdfsLabel as u64,
            NodeDictionary::rdfsrange as u64,
            NodeDictionary::rdfsLiteral,
        ]);
        self.store.add_triple([
            NodeDictionary::rdfValue as u64,
            NodeDictionary::rdfsrange as u64,
            NodeDictionary::rdfsResource,
        ]);
        // MISC
        self.store.add_triple([
            NodeDictionary::rdfAlt,
            NodeDictionary::rdfssubClassOf as u64,
            NodeDictionary::rdfsContainer,
        ]);
        self.store.add_triple([
            NodeDictionary::rdfBag,
            NodeDictionary::rdfssubClassOf as u64,
            NodeDictionary::rdfsContainer,
        ]);
        self.store.add_triple([
            NodeDictionary::rdfSeq,
            NodeDictionary::rdfssubClassOf as u64,
            NodeDictionary::rdfsContainer,
        ]);
        self.store.add_triple([
            NodeDictionary::rdfsContainerMembershipProperty as u64,
            NodeDictionary::rdfssubClassOf as u64,
            NodeDictionary::rdfProperty as u64,
        ]);
        self.store.add_triple([
            NodeDictionary::rdf_1 as u64,
            NodeDictionary::rdftype as u64,
            NodeDictionary::rdfsContainerMembershipProperty as u64,
        ]);
        self.store.add_triple([
            NodeDictionary::rdf_1 as u64,
            NodeDictionary::rdfsdomain as u64,
            NodeDictionary::rdfsResource,
        ]);
        self.store.add_triple([
            NodeDictionary::rdf_1 as u64,
            NodeDictionary::rdfsrange as u64,
            NodeDictionary::rdfsResource,
        ]);
        self.store.add_triple([
            NodeDictionary::rdfsisDefinedBy as u64,
            NodeDictionary::rdfssubPropertyOf as u64,
            NodeDictionary::rdfsSeeAlso as u64,
        ]);
        self.store.add_triple([
            NodeDictionary::rdfXMLLiteral,
            NodeDictionary::rdftype as u64,
            NodeDictionary::rdfsDatatype,
        ]);
        self.store.add_triple([
            NodeDictionary::rdfXMLLiteral,
            NodeDictionary::rdfssubClassOf as u64,
            NodeDictionary::rdfsLiteral,
        ]);
        self.store.add_triple([
            NodeDictionary::rdfsDatatype,
            NodeDictionary::rdfssubClassOf as u64,
            NodeDictionary::rdfsClass,
        ]);
        self.store.add_triple([
            NodeDictionary::xsdnonNegativeInteger,
            NodeDictionary::rdftype as u64,
            NodeDictionary::rdfsDatatype,
        ]);
        self.store.add_triple([
            NodeDictionary::xsdstring,
            NodeDictionary::rdftype as u64,
            NodeDictionary::rdfsDatatype,
        ]);
        self.store.add_triple([
            NodeDictionary::rdftype as u64,
            NodeDictionary::rdfssubPropertyOf as u64,
            NodeDictionary::rdftype as u64,
        ]);
        self.store.add_triple([
            NodeDictionary::rdfsdomain as u64,
            NodeDictionary::rdfssubPropertyOf as u64,
            NodeDictionary::rdfsdomain as u64,
        ]);
        self.store.add_triple([
            NodeDictionary::rdfsrange as u64,
            NodeDictionary::rdfssubPropertyOf as u64,
            NodeDictionary::rdfsrange as u64,
        ]);
        self.store.add_triple([
            NodeDictionary::rdfssubPropertyOf as u64,
            NodeDictionary::rdfssubPropertyOf as u64,
            NodeDictionary::rdfssubPropertyOf as u64,
        ]);
        self.store.add_triple([
            NodeDictionary::rdfssubClassOf as u64,
            NodeDictionary::rdfssubPropertyOf as u64,
            NodeDictionary::rdfssubClassOf as u64,
        ]);
        self.store.sort();
    }
}

impl<TS> From<TS> for InfGraph
where
    TS: TripleSource,
{
    fn from(mut ts: TS) -> Self {
        let mut dictionary = NodeDictionary::new();
        let mut store = TripleStore::default();
        ts.for_each_triple(|t| {
            let rep = dictionary.encode_triple(&t);

            store.add_triple(rep);
        })
        .expect("Streaming error");
        store.remap_res_to_prop(dictionary.remapped());
        Self { dictionary, store }
    }
}
