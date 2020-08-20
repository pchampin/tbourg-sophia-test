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

use itertools::Itertools;
use once_cell::sync::OnceCell;
use rayon::prelude::*;

use super::NodeDictionary;
use crate::rules::*;
use crate::utils::merge_sort;

/// See [module documentation](./index.html).
#[derive(Default, PartialEq, Debug, Clone)]
pub struct TripleStore {
    /// each chunk represents triples with a given predicate
    elem: Vec<Chunk>,
    /// total number of triples in all the chunks
    size: usize,
}

/// See [module documentation](./index.html).
#[derive(PartialEq, Debug, Clone)]
pub struct Chunk {
    // subject-object list
    so: Vec<[u64; 2]>,
    // object-subject list (built lazily)
    os: OnceCell<Vec<[u64; 2]>>,
    // dirty-flag, indicating that so is not correctly sorted
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

    pub fn len(&self) -> usize {
        debug_assert!(!self.so_dirty);
        self.so.len()
    }

    pub fn merge(&mut self, other: Chunk) {
        debug_assert!(!self.so_dirty);
        debug_assert!(!other.so_dirty);
        let old_so = std::mem::replace(&mut self.so, vec![]);
        self.so = merge_sort(old_so, other.so);
        self.os = OnceCell::new();
    }

    fn so_sort(&mut self) -> usize {
        if self.so_dirty {
            self.so_dirty = false;
            self.os = OnceCell::new();
            bucket_sort_pairs(&mut self.so)
        } else {
            self.so.len()
        }
    }

    fn remap_res_to_prop(&mut self, map: &[[u64; 2]]) {
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

    fn add_so(&mut self, so: [u64; 2]) {
        self.so_dirty = true;
        self.so.push(so);
    }
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

/// Sort the pairs and remove duplicates
fn bucket_sort_pairs(pairs: &mut Vec<[u64; 2]>) -> usize {
    if pairs.is_empty() {
        return 0;
    }
    let (min, max) = pairs
        .iter()
        .map(|pair| pair[0])
        .minmax()
        .into_option()
        .unwrap_or((0, 0));
    let width = (max - min + 1) as usize;
    let mut hist: Vec<usize> = vec![0; width];
    let mut cumul: Vec<usize> = vec![0; width];
    build_hist(pairs, min, 0, &mut hist);
    let hist2 = hist.to_vec();
    build_cumul(&hist, &mut cumul);
    let len = pairs.len();
    let mut objects = vec![0; len];
    for val in pairs.iter() {
        let val_s = val[0];
        let val_o = val[1];
        let idx = (val_s - min) as usize;
        let pos = cumul[idx];
        let remaining = hist[idx];
        let obj_idx = (pos + remaining - 1) as usize;
        hist[idx] -= 1;
        objects[obj_idx] = val_o;
    }

    for i in 0..(width - 1) {
        quickersort::sort(&mut objects[cumul[i]..cumul[i + 1]]);
    }
    quickersort::sort(&mut objects[cumul[width - 1]..len]);
    let mut j = 0;
    let mut l = 0;
    let mut last = 0;
    for (i, val) in hist2.iter().enumerate() {
        let s = min + i as u64;
        for k in 0..*val {
            let o = objects[l];
            l += 1;
            if k == 0 || o != last {
                pairs[j] = [s, o];
                j += 1;
            }
            last = o;
        }
    }
    pairs.truncate(j);
    j
}

#[inline]

fn build_hist(pairs: &[[u64; 2]], min: u64, pair_elem: usize, hist: &mut [usize]) {
    for pair in pairs {
        hist[(pair[pair_elem] - min) as usize] += 1;
    }
}

#[inline]

fn build_cumul(hist: &[usize], cumul: &mut [usize]) {
    for i in 1..hist.len() {
        cumul[i] = cumul[i - 1] + hist[i - 1];
    }
}


fn reverse_pairs(pairs: &mut [[u64; 2]]) {
    for pair in pairs.iter_mut() {
        pair.swap(0, 1);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sort() {
        let mut pairs = vec![[2, 1], [1, 3]];
        bucket_sort_pairs(&mut pairs);
        let expected = [[1, 3], [2, 1]];
        assert_eq!(pairs, expected);
        let mut pairs = vec![[2, 1], [1, 3], [2, 1]];
        bucket_sort_pairs(&mut pairs);
        let expected = [[1, 3], [2, 1]];
        assert_eq!(pairs, expected);
        let mut pairs = vec![[2, 1], [1, 3], [1, 3]];
        bucket_sort_pairs(&mut pairs);
        let expected = [[1, 3], [2, 1]];
        assert_eq!(pairs, expected);
        let mut pairs = vec![[2, 3], [2, 1]];
        bucket_sort_pairs(&mut pairs);
        let expected = [[2, 1], [2, 3]];
        assert_eq!(pairs, expected);
    }    
}
