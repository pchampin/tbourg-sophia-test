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
/// [`TripleStore`]: ./struct.TripleStore.html
/// [`Chunk`]: ../chunk/index.html

use super::Chunk;
use super::NodeDictionary;
use crate::closure::*;

/// See [module documentation](./index.html).
#[derive(Default, PartialEq, Debug, Clone)]
pub(crate) struct TripleStore {
    /// each chunk represents triples with a given predicate
    chunks: Vec<Chunk>,
    /// total number of triples in all the chunks
    size: usize,
}

impl TripleStore {
    /// Collect integer-triples into a sorted TripleSTore.
    pub fn new<'a, I>(triples: I) -> Self
    where
        I: IntoIterator<Item=&'a [u64; 3]>
    {
        let mut proto_chunks = vec![];
        for triple in triples {
            let [is, ip, io] = triple;
            let op = NodeDictionary::prop_idx_to_offset(*ip);
            if op >= proto_chunks.len() {
                proto_chunks.resize_with(op+1, Vec::new);
            }
            proto_chunks[op].push([*is, *io]);
        }
        let chunks: Vec<Chunk> = proto_chunks.into_iter()
            .map(|v| v[..].into())
            .collect();
        let size = chunks.iter().map(|c| c.len()).sum();
        #[cfg(debug_assertions)]
        debug_assert!(chunks.iter().map(Chunk::is_sorted).all(|b| b));
        Self { chunks, size }
    }

    /// The total number of triples in this store.
    #[inline]
    pub fn size(&self) -> usize {
        self.size
    }

    /// Borrow the chunks of this store.
    #[inline]
    pub fn chunks(&self) -> &[Chunk] {
        &self.chunks
    }

    #[cfg(debug_assertions)]
    /// For tests only. Checks that this store is sorted.
    pub fn is_sorted(&self) -> bool {
        self.chunks.iter().map(Chunk::is_sorted).all(|b| b)
        &&
        self.chunks.iter().map(Chunk::len).sum::<usize>() == self.size
    }

    /// Computes the transitive closure of the given property
    pub(super) fn transitive_closure(&mut self, ip: u32) {
        let offset = NodeDictionary::prop_idx_to_offset(ip as u64);
        if offset >= self.chunks.len() || self.chunks[offset].is_empty() {
            return
        }
        let old_chunk = &self.chunks[offset];
        let old_len = old_chunk.len();
        let closure = ClosureGraph::from(old_chunk.so().to_vec()).close();
        let new_chunk: Chunk = closure.iter()
            .flat_map(|(s, os)| os.iter().map(move |o| [*s, *o]))
            .collect::<Vec<_>>()[..]
            .into();
        let new_len = new_chunk.len();
        self.chunks[offset] = new_chunk;
        self.size += new_len - old_len;
    }

    /// Merge triples from another store into this one.
    pub(super) fn merge(&mut self, mut other: Self) {
        if other.size == 0 {
            return;
        }
        let s_len = self.chunks.len();
        let o_len = other.chunks.len();
        self.size = 0;
        let mut other_chunks = other.chunks.drain(..);
    
        for i in 0..s_len.min(o_len) {
            let o_chunk = other_chunks.next().unwrap();
            self.chunks[i].merge(o_chunk);
            self.size += self.chunks[i].len();
        }
        if s_len > o_len {
            for chunk in &self.chunks[o_len..] {
                self.size += chunk.so().len();
            }
        } else if s_len < o_len {
            self.chunks.reserve(o_len-s_len);
            for chunk in other_chunks {
                self.size += chunk.len();
                self.chunks.push(chunk);
            }
        }
    }

    /// Update this chunk with the given translation map/
    ///
    /// This is used when resources (index > START_INDEX)
    /// have been requalified as properties (index < START_INDEX).
    ///
    /// IMPORTANT: this method is not used yet,
    /// but will be required later when rules such as EQ-REP-P
    /// are correctly implemented.
    #[allow(dead_code)]
    pub(super) fn remap(&mut self, map: &[[u64; 2]]) {
        for chunk in &mut self.chunks {
            chunk.remap(map);
        }
    }
}
