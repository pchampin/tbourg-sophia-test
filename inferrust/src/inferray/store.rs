/// A [`TripleStore`] stores integer-triples in a compact and efficient structure:
///
/// * each predicate is represented by a [`Chunk`];
/// * a [`Chunk`] maintains two lists of pairs (subject-object and object-subject),
///   sorted in lexicographical order.
///
/// This allows to efficiently check if a given triple is present,
/// or to iterate over
///
/// * all triples with a given predicate;
/// * all triples with a given predicate and subject;
/// * all triples with a given predicate and object.
///
/// Note also that when a [`Chunk`] is built, only the (subject-object) list is built;
/// the (object-subject) list is built lazily whenever it is required.
///
/// [`TripleStore`]: ./struct.TripleStore.html
/// [`Chunk`]: ./struct.Chunk.html

use rayon::prelude::*;

use super::Chunk;
use super::NodeDictionary;
use crate::rules::*;

/// See [module documentation](./index.html).
#[derive(Default, PartialEq, Debug, Clone)]
pub struct TripleStore {
    /// each chunk represents triples with a given predicate
    elem: Vec<Chunk>,
    /// total number of triples in all the chunks
    size: usize,
}

impl TripleStore {
    #[inline]
    pub fn elem(&self) -> &Vec<Chunk> {
        &self.elem
    }

    pub(super) fn add_triple(&mut self, triple: [u64; 3]) {
        let [is, ip, io] = triple;
        let ip_to_store = NodeDictionary::prop_idx_to_offset(ip);
        self.ensure_prop(ip_to_store);
        self.add_triple_raw(is, ip_to_store, io);
    }

    pub fn add_all(&mut self, others: Vec<RuleResult>) {
        for other in others.into_iter() {
            for t in other {
                self.add_triple(t);
            }
        }
    }

    /// Ensure that `self.elem` has an array at index `ip`
    #[inline]
    pub fn ensure_prop(&mut self, ip: usize) {
        if ip >= self.elem.len() {
            self.elem.resize_with(ip + 1, Chunk::empty);
        }
    }

    /// # Pre-condition
    /// `self.elem` must have an element at index `ip`
    #[inline]
    pub fn add_triple_raw(&mut self, is: u64, ip: usize, io: u64) {
        self.size += 1;
        self.elem[ip].add_so([is, io]);
    }

    pub(super) fn sort(&mut self) {
        if self.elem.is_empty() {
            return;
        }
        self.size = self.elem.par_iter_mut().map(|chunk| chunk.so_sort()).sum();
    }

    pub(super) fn remap_res_to_prop(&mut self, map: &[[u64; 2]]) {
        for chunk in &mut self.elem {
            chunk.remap_res_to_prop(map);
        }
    }

    pub fn size(&mut self) -> usize {
        self.size
    }

    pub(super) fn merge(&mut self, mut other: Self) {
        if other.size == 0 {
            return;
        }
        let s_len = self.elem.len();
        let o_len = other.elem.len();
        self.size = 0;
        let mut other_chunks = other.elem.drain(..);
    
        for i in 0..s_len.min(o_len) {
            let o_chunk = other_chunks.next().unwrap();
            self.elem[i].merge(o_chunk);
            self.size += self.elem[i].len();
        }
        if s_len > o_len {
            for chunk in &self.elem[o_len..] {
                self.size += chunk.so().len();
            }
        } else if s_len < o_len {
            self.elem.reserve(o_len-s_len);
            for chunk in other_chunks {
                self.size += chunk.len();
                self.elem.push(chunk);
            }
        }
    }
}
