#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
use sophia::ns::*;
use sophia::term::factory::{ArcTermFactory, TermFactory};
use sophia::term::TTerm;
use sophia::term::{ArcTerm, RefTerm, StaticTerm, Term, TermData};
use sophia::triple::Triple;

use std::borrow::Borrow;
use std::collections::HashMap;

pub struct NodeDictionary {
    factory: ArcTermFactory,
    resources: Vec<ArcTerm>,
    properties: Vec<ArcTerm>,
    indexes: HashMap<StaticTerm, u64>,
    remapped: Vec<[u64; 2]>,
}

impl NodeDictionary {
    pub const START_INDEX: u32 = u32::max_value();
    pub const rdfsResource: u64 = Self::START_INDEX as u64 + 1;
    pub const rdfsClass: u64 = Self::START_INDEX as u64 + 2;
    pub const rdfsDatatype: u64 = Self::START_INDEX as u64 + 3;
    pub const rdfsLiteral: u64 = Self::START_INDEX as u64 + 4;
    pub const rdfsContainer: u64 = Self::START_INDEX as u64 + 5;
    pub const rdfsdomain: u32 = Self::START_INDEX - 1;
    pub const rdfsrange: u32 = Self::START_INDEX - 2;
    pub const rdfssubClassOf: u32 = Self::START_INDEX - 3;
    pub const rdfssubPropertyOf: u32 = Self::START_INDEX - 4;
    pub const rdfsSeeAlso: u32 = Self::START_INDEX - 5;
    pub const rdfsisDefinedBy: u32 = Self::START_INDEX - 6;
    pub const rdfsComment: u32 = Self::START_INDEX - 7;
    pub const rdfsMember: u32 = Self::START_INDEX - 8;
    pub const rdfsContainerMembershipProperty: u32 = Self::START_INDEX - 9;
    pub const rdfsLabel: u32 = Self::START_INDEX - 10;
    pub const rdfList: u64 = Self::START_INDEX as u64 + 6;
    pub const rdfAlt: u64 = Self::START_INDEX as u64 + 7;
    pub const rdfBag: u64 = Self::START_INDEX as u64 + 8;
    pub const rdfSeq: u64 = Self::START_INDEX as u64 + 9;
    pub const rdfXMLLiteral: u64 = Self::START_INDEX as u64 + 10;
    pub const rdfStatement: u64 = Self::START_INDEX as u64 + 11;
    pub const rdfnil: u64 = Self::START_INDEX as u64 + 12;
    pub const rdfProperty: u32 = Self::START_INDEX - 11;
    pub const rdftype: u32 = Self::START_INDEX - 12;
    pub const rdfsubject: u32 = Self::START_INDEX - 13;
    pub const rdfobject: u32 = Self::START_INDEX - 14;
    pub const rdfpredicate: u32 = Self::START_INDEX - 15;
    pub const rdffirst: u32 = Self::START_INDEX - 16;
    pub const rdfrest: u32 = Self::START_INDEX - 17;
    pub const rdfValue: u32 = Self::START_INDEX - 18;
    pub const rdf_1: u32 = Self::START_INDEX - 19;
    pub const xsdnonNegativeInteger: u64 = Self::START_INDEX as u64 + 13;
    pub const xsdstring: u64 = Self::START_INDEX as u64 + 14;
    pub const owlthing: u32 = Self::START_INDEX - 20;
    pub const owltransitiveProperty: u32 = Self::START_INDEX - 21;
    pub const owlequivalentClass: u32 = Self::START_INDEX - 22;
    pub const owlequivalentProperty: u32 = Self::START_INDEX - 23;
    pub const owlobjectProperty: u32 = Self::START_INDEX - 24;
    pub const owldataTypeProperty: u32 = Self::START_INDEX - 25;
    pub const owlsameAs: u32 = Self::START_INDEX - 26;
    pub const owlinverseOf: u32 = Self::START_INDEX - 27;
    pub const owlpropertyDisjointWith: u32 = Self::START_INDEX - 28;
    pub const owldifferentFrom: u32 = Self::START_INDEX - 29;
    pub const owlallDifferent: u32 = Self::START_INDEX - 30;
    pub const owlallDisjointClasses: u32 = Self::START_INDEX - 31;
    pub const owlallValuesFrom: u32 = Self::START_INDEX - 32;
    pub const owlannotationProperty: u32 = Self::START_INDEX - 33;
    pub const owlassertionProperty: u32 = Self::START_INDEX - 34;
    pub const owlclass: u64 = Self::START_INDEX as u64 + 15;
    pub const owlcomplementOf: u32 = Self::START_INDEX - 35;
    pub const owldisjoinWith: u32 = Self::START_INDEX - 36;
    pub const owldistinctmembers: u32 = Self::START_INDEX - 37;
    pub const owlfunctionalProperty: u32 = Self::START_INDEX - 38;
    pub const intersectionOf: u32 = Self::START_INDEX - 39;
    pub const unionOf: u32 = Self::START_INDEX - 40;
    pub const owlinverseFunctionalProperty: u32 = Self::START_INDEX - 41;
    pub const irreflexiveProperty: u32 = Self::START_INDEX - 42;
    pub const maxCardinality: u32 = Self::START_INDEX - 43;
    pub const members: u32 = Self::START_INDEX - 44;
    pub const nothing: u32 = Self::START_INDEX - 45;
    pub const onClass: u32 = Self::START_INDEX - 46;
    pub const onProperty: u32 = Self::START_INDEX - 47;
    pub const oneOf: u32 = Self::START_INDEX - 48;
    pub const propertyChainAxiom: u32 = Self::START_INDEX - 49;
    pub const owlsomeValuesFrom: u32 = Self::START_INDEX - 50;
    pub const sourceIndividual: u32 = Self::START_INDEX - 51;
    pub const owlsymmetricProperty: u32 = Self::START_INDEX - 52;
    pub const owltargetIndividual: u32 = Self::START_INDEX - 53;
    pub const targetValue: u32 = Self::START_INDEX - 54;
    pub const maxQualifiedCardinality: u32 = Self::START_INDEX - 55;
    const res_start: u64 = Self::START_INDEX as u64 + 15;
    const prop_start: u32 = Self::START_INDEX - 55;

    pub fn new() -> Self {
        let mut me = Self {
            factory: ArcTermFactory::new(),
            resources: Vec::with_capacity((Self::res_start - Self::START_INDEX as u64) as usize),
            properties: Vec::with_capacity((Self::START_INDEX - Self::prop_start) as usize),
            indexes: HashMap::new(),
            remapped: vec![],
        };
        me.init_const();
        me
    }

    #[inline]
    pub(super) fn remapped(&self) -> &[[u64; 2]] {
        &self.remapped[..]
    }

    pub(super) fn encode_triple<T>(&mut self, t: &T) -> [u64; 3]
    where
        T: Triple,
        // T::Term: ?Sized,
    {
        let mut s: u64 = 0;
        let mut o: u64 = 0;
        let p: u32;
        let ts = t.s();
        let to = t.o();
        let tp = t.p();
        // Property will always be property
        p = self.add_property(tp);
        let prop_in_s_or_o = contains_prop_in_s_or_o(p);
        if prop_in_s_or_o != -1 {
            match prop_in_s_or_o {
                1 => {
                    s = self.add_property(ts).into();
                    o = self.add(to);
                }
                3 => {
                    s = self.add_property(ts).into();
                    o = self.add_property(to).into();
                }
                _ => (),
            }
        } else {
            // Add a regular triple
            s = self.add(ts);
            o = self.add(to);
        }
        [s, p as u64, o]
    }

    fn add<T>(&mut self, term: &T) -> u64
    where
        T: TTerm + ?Sized,
    {
        let term: RefTerm = RefTerm::from(term);
        match self.indexes.get(&term) {
            Some(idx) => *idx,
            None => {
                // NB: we could not use self.index.entry,
                // because we do not want to allocate the term before we need it
                let arcterm = self.factory.convert_term(term);
                let refterm = unsafe { fake_static(&arcterm) };
                self.resources.push(arcterm);
                let idx = self.resources.len() as u64 + Self::START_INDEX as u64;
                self.indexes.insert(refterm, idx);
                idx
            }
        }
    }

    fn add_property<T>(&mut self, term: &T) -> u32
    where
        T: TTerm + ?Sized,
    {
        let term: RefTerm = RefTerm::from(term);
        let old_idx = self.indexes.get(&term).cloned();
        if let Some(idx) = old_idx {
            if idx < Self::START_INDEX as u64 {
                return idx as u32
            }
        }
        let arcterm = match old_idx {
            Some(old_idx) => self.resources[(old_idx - Self::START_INDEX as u64 - 1) as usize].clone(),
            None => self.factory.convert_term(term),
        };
        let refterm = unsafe { fake_static(&arcterm) };
        self.properties.push(arcterm);
        let idx = Self::START_INDEX as u32 - self.properties.len() as u32;
        self.indexes.insert(refterm, idx as u64);
        if let Some(old_idx) = old_idx {
            self.remapped.push([old_idx, idx as u64]);
        }
        idx
    }

    #[inline]

    fn add_with<T>(&mut self, term: &T, id: u64)
    where
        T: TTerm + ?Sized,
    {
        let idx = self.add(term);
        debug_assert_eq!(idx, id);
    }
    #[inline]

    fn add_property_with<T>(&mut self, term: &T, id: u32)
    where
        T: TTerm + ?Sized,
    {
        let idx = self.add_property(term);
        debug_assert_eq!(idx, id);
    }

    pub(super) fn get_term(&self, index: u64) -> &ArcTerm {
        if index < Self::START_INDEX as u64 {
            &self.properties[Self::START_INDEX as usize - index as usize - 1]
        } else {
            &self.resources[index as usize - Self::START_INDEX as usize - 1]
        }
    }

    #[inline]
    pub(super) fn get_index<T>(&self, t: &T) -> Option<u64>
    where
        T: TTerm + ?Sized,
    {
        self.indexes.get(&RefTerm::from(t)).cloned()
    }

    pub fn was_remapped(&self, res: u64) -> bool {
        self.remapped.iter().any(|[o, _]| *o == res)
    }

    pub fn get_res_ctr(&self) -> u64 {
        self.resources.len() as u64 + Self::START_INDEX as u64
    }

    pub fn prop_idx_to_idx(prop_idx: u64) -> usize {
        Self::START_INDEX as usize - prop_idx as usize - 1
    }

    pub fn idx_to_prop_idx(idx: usize) -> u64 {
        Self::START_INDEX as u64 - idx as u64 - 1
    }

    fn init_const(&mut self) {
        // ---------------RDFS
        self.add_with(&rdfs::Resource, Self::rdfsResource);
        self.add_with(&rdfs::Class, Self::rdfsClass);
        self.add_with(&rdfs::Datatype, Self::rdfsDatatype);
        self.add_with(&rdfs::Literal, Self::rdfsLiteral);
        self.add_with(&rdfs::Container, Self::rdfsContainer);

        self.add_property_with(&rdfs::domain, Self::rdfsdomain);
        self.add_property_with(&rdfs::range, Self::rdfsrange);
        self.add_property_with(&rdfs::subClassOf, Self::rdfssubClassOf);
        self.add_property_with(&rdfs::subPropertyOf, Self::rdfssubPropertyOf);
        self.add_property_with(&rdfs::seeAlso, Self::rdfsSeeAlso);
        self.add_property_with(&rdfs::isDefinedBy, Self::rdfsisDefinedBy);
        self.add_property_with(&rdfs::comment, Self::rdfsComment);
        self.add_property_with(&rdfs::member, Self::rdfsMember);
        self.add_property_with(
            &rdfs::ContainerMembershipProperty,
            Self::rdfsContainerMembershipProperty,
        );
        self.add_property_with(&rdfs::label, Self::rdfsLabel);

        // -----------------RDF

        self.add_with(&rdf::List, Self::rdfList);
        self.add_with(&rdf::Alt, Self::rdfAlt);
        self.add_with(&rdf::Bag, Self::rdfBag);
        self.add_with(&rdf::Seq, Self::rdfSeq);
        self.add_with(&rdf::XMLLiteral, Self::rdfXMLLiteral);
        self.add_with(&rdf::Statement, Self::rdfStatement);
        self.add_with(&rdf::nil, Self::rdfnil);

        self.add_property_with(&rdf::Property, Self::rdfProperty);
        self.add_property_with(&rdf::type_, Self::rdftype);
        self.add_property_with(&rdf::subject, Self::rdfsubject);
        self.add_property_with(&rdf::object, Self::rdfobject);
        self.add_property_with(&rdf::predicate, Self::rdfpredicate);
        self.add_property_with(&rdf::first, Self::rdffirst);
        self.add_property_with(&rdf::rest, Self::rdfrest);
        self.add_property_with(&rdf::value, Self::rdfValue);
        // TODO: add rdf1 to sophia
        self.add_property_with(
            &sophia::ns::Namespace::new("http://www.w3.org/1999/02/22-rdf-syntax-ns#")
                .unwrap()
                .get("_1")
                .unwrap(),
            Self::rdf_1,
        );

        // ------------------XSD

        self.add_with(&xsd::nonNegativeInteger, Self::xsdnonNegativeInteger);
        self.add_with(&xsd::string, Self::xsdstring);

        // ------------------OWL

        self.add_property_with(&owl::Thing, Self::owlthing);
        self.add_property_with(&owl::TransitiveProperty, Self::owltransitiveProperty);
        self.add_property_with(&owl::equivalentClass, Self::owlequivalentClass);
        self.add_property_with(&owl::equivalentProperty, Self::owlequivalentProperty);
        self.add_property_with(&owl::ObjectProperty, Self::owlobjectProperty);
        self.add_property_with(&owl::DatatypeProperty, Self::owldataTypeProperty);
        self.add_property_with(&owl::sameAs, Self::owlsameAs);

        self.add_property_with(&owl::inverseOf, Self::owlinverseOf);
        self.add_property_with(&owl::propertyDisjointWith, Self::owlpropertyDisjointWith);
        self.add_property_with(&owl::differentFrom, Self::owldifferentFrom);
        self.add_property_with(&owl::AllDifferent, Self::owlallDifferent);
        self.add_property_with(&owl::AllDisjointClasses, Self::owlallDisjointClasses);
        self.add_property_with(&owl::allValuesFrom, Self::owlallValuesFrom);
        self.add_property_with(&owl::AnnotationProperty, Self::owlannotationProperty);
        self.add_property_with(&owl::assertionProperty, Self::owlassertionProperty);
        self.add_with(&owl::Class, Self::owlclass);
        self.add_property_with(&owl::complementOf, Self::owlcomplementOf);
        self.add_property_with(&owl::disjointWith, Self::owldisjoinWith);
        self.add_property_with(&owl::distinctMembers, Self::owldistinctmembers);
        self.add_property_with(&owl::FunctionalProperty, Self::owlfunctionalProperty);
        self.add_property_with(&owl::intersectionOf, Self::intersectionOf);
        self.add_property_with(&owl::unionOf, Self::unionOf);
        self.add_property_with(
            &owl::InverseFunctionalProperty,
            Self::owlinverseFunctionalProperty,
        );
        self.add_property_with(&owl::IrreflexiveProperty, Self::irreflexiveProperty);
        self.add_property_with(&owl::maxCardinality, Self::maxCardinality);
        self.add_property_with(&owl::members, Self::members);
        self.add_property_with(&owl::Nothing, Self::nothing);
        self.add_property_with(&owl::onClass, Self::onClass);
        self.add_property_with(&owl::onProperty, Self::onProperty);
        self.add_property_with(&owl::oneOf, Self::oneOf);
        self.add_property_with(&owl::propertyChainAxiom, Self::propertyChainAxiom);
        self.add_property_with(&owl::someValuesFrom, Self::owlsomeValuesFrom);
        self.add_property_with(&owl::sourceIndividual, Self::sourceIndividual);
        self.add_property_with(&owl::SymmetricProperty, Self::owlsymmetricProperty);
        self.add_property_with(&owl::targetIndividual, Self::owltargetIndividual);
        self.add_property_with(&owl::targetValue, Self::targetValue);
        self.add_property_with(&owl::maxQualifiedCardinality, Self::maxQualifiedCardinality);
    }
}

/// Unsafely converts a term into a StaticTerm.
/// This is to be used *only* when we can guarantee that the produced StaticTerm
/// will not outlive the source term.
/// We use this for keys in TermIndexMapU::t2i, when the owning term is in TermIndexMapU::i2t.
#[inline]
unsafe fn fake_static<S, T>(t: &T) -> StaticTerm
where
    S: TermData,
    T: Borrow<Term<S>>,
{
    t.borrow().clone_map(|txt| &*(txt as *const str))
}

// Should return -1 if both s and o are res,
// 1 if s is prop and o is res,
// and 3 if both s and o are prop
fn contains_prop_in_s_or_o(property_index: u32) -> i8 {
    let prop_in_s = vec![NodeDictionary::rdfsdomain, NodeDictionary::rdfsrange];
    let prop_in_s_and_o = vec![
        NodeDictionary::owlequivalentProperty,
        NodeDictionary::owlinverseOf,
        NodeDictionary::rdfssubPropertyOf,
    ];
    if prop_in_s_and_o.contains(&property_index) {
        3
    } else if prop_in_s.contains(&property_index) {
        1
    } else {
        -1
    }
}
