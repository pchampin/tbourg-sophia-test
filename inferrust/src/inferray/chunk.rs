//! A chunk store the triples for a single predicate.
//! The predicate is not stored by the chunk,
//! and subjects and objkects are only represented by indexes (u64).
//!
//! Triples are stored as two lists:
//! - `so` is ordered by subject, then objects
//! - `os` is ordered by object, then subject
//!
//! Note that `os` is generated lazily when required.
//! 
/// See [module documentation](./index.html).

use crate::utils::{bucket_sort_pairs, merge_sort};
use once_cell::sync::OnceCell;

#[derive(PartialEq, Debug, Clone)]
pub(crate) struct Chunk {
    /// subject-object list
    so: Vec<[u64; 2]>,
    /// object-subject list (built lazily)
    os: OnceCell<Vec<[u64; 2]>>,
}

impl Chunk {
    // type-invariant:
    // * so must be sorted
    // * so_dirty must be false

    /// Create an empty `Chunk`.
    ///
    /// See also implementation of `From<&[[u64; 2]]>
    pub fn empty() -> Chunk {
        Chunk {
            so: vec![],
            os: OnceCell::new(),
        }
    }

    /// The number of triples in this chunk.
    pub fn len(&self) -> usize {
        self.so.len()
    }

    /// Whether this chunk contains no triple.
    pub fn is_empty(&self) -> bool {
        self.so.is_empty()
    }

    /// The list of (subject-object) pairs.
    pub fn so(&self) -> &[[u64; 2]] {
        &self.so
    }

    /// The list of (object-subject) pairs.
    ///
    /// # Performance
    /// This list is generated lazily,
    /// so the first call to this method after building
    /// (or modifying) a `Chunk` can be costly.
    pub fn os(&self) -> &[[u64; 2]] {
        &self.os.get_or_init(|| {
            let mut v = self.so.clone();
            reverse_pairs(&mut v);
            bucket_sort_pairs(&mut v);
            v
        })
    }

    /// Add new pairs to this `Chunk`, and re-sort the underlying data.
    pub fn add_pairs(&mut self, pairs: &[[u64; 2]]) {
        if pairs.is_empty() {
            return;
        }
        self.so.extend_from_slice(pairs);
        bucket_sort_pairs(&mut self.so);
        self.os = OnceCell::new();
    }

    /// Merge `other` into this `Chunk`,
    /// ensuring that it remains sorted after the operation.
    pub fn merge(&mut self, other: Chunk) {
        let old_so = std::mem::replace(&mut self.so, vec![]);
        self.so = merge_sort(old_so, other.so);
        self.os = OnceCell::new();
    }

    #[cfg(debug_assertions)]
    /// For tests only; check that this chunk is sorted.
    pub fn is_sorted(&self) -> bool {
        for i in 1..self.so.len() {
            if self.so[i-1] > self.so[i] {
                return false;
            }
        }
        true
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
        let mut dirty = false;
        for pair in self.so.iter_mut() {
            for val in pair.iter_mut() {
                for [old, new] in map {
                    if *val == *old {
                        dirty = true;
                        *val = *new;
                        break;
                    }
                }
            }
        }
        if dirty {
            bucket_sort_pairs(&mut self.so);
            self.os = OnceCell::new();
        }
    }
}

impl From<&[[u64; 2]]> for Chunk {
    fn from(other: &[[u64; 2]]) -> Chunk {
        let mut ret = Chunk::empty();
        ret.add_pairs(other);
        ret
    }
}

/// Reverse every pair in `pairs`.
fn reverse_pairs(pairs: &mut [[u64; 2]]) {
    for pair in pairs.iter_mut() {
        pair.swap(0, 1);
    }
}
