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
pub struct Chunk {
    /// subject-object list
    so: Vec<[u64; 2]>,
    /// object-subject list (built lazily)
    os: OnceCell<Vec<[u64; 2]>>,
    /// dirty-flag, indicating that so is not correctly sorted
    /// TODO remove it once the API has been cleaned
    so_dirty: bool,
}

impl Chunk {
    // type-invariant:
    // * so must be sorted
    // * so_dirty must be false

    pub fn empty() -> Chunk {
        Chunk {
            so: vec![],
            os: OnceCell::new(),
            so_dirty: false,
        }
    }

    pub fn len(&self) -> usize {
        debug_assert!(!self.so_dirty);
        self.so.len()
    }

    pub fn so(&self) -> &Vec<[u64; 2]> {
        debug_assert!(!self.so_dirty);
        &self.so
    }

    pub fn os(&self) -> &Vec<[u64; 2]> {
        debug_assert!(!self.so_dirty);
        self.os.get_or_init(|| {
            let mut v = self.so.clone();
            reverse_pairs(&mut v);
            bucket_sort_pairs(&mut v);
            v
        })
    }

    pub fn add_pairs(&mut self, pairs: &[[u64; 2]]) {
        debug_assert!(!self.so_dirty);
        if pairs.is_empty() {
            return;
        }
        self.so.extend_from_slice(pairs);
        bucket_sort_pairs(&mut self.so);
        self.os = OnceCell::new();
    }

    pub fn merge(&mut self, other: Chunk) {
        debug_assert!(!self.so_dirty);
        debug_assert!(!other.so_dirty);
        let old_so = std::mem::replace(&mut self.so, vec![]);
        self.so = merge_sort(old_so, other.so);
        self.os = OnceCell::new();
    }

    // TODO change the contract of this method
    // + maybe expect a dict, or at least a sorted map
    // + probably sort in the end if some changes have been done
    // + then make it public
    pub(super) fn remap_res_to_prop(&mut self, map: &[[u64; 2]]) {
        for [old, new] in map {
            for pair in self.so.iter_mut() {
                for val in pair.iter_mut() {
                    if *val == *old {
                        self.so_dirty = true;
                        *val = *new;
                    }
                }
            }
        }
    }

    // TODO remove this method
    pub(super) fn so_sort(&mut self) -> usize {
        if self.so_dirty {
            self.so_dirty = false;
            self.os = OnceCell::new();
            bucket_sort_pairs(&mut self.so)
        } else {
            self.so.len()
        }
    }

    // TODO remove this method
    pub(super) fn add_so(&mut self, so: [u64; 2]) {
        self.so_dirty = true;
        self.so.push(so);
    }
}

fn reverse_pairs(pairs: &mut [[u64; 2]]) {
    for pair in pairs.iter_mut() {
        pair.swap(0, 1);
    }
}
