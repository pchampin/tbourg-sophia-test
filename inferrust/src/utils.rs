//! Bunch of utility functions

use itertools::Itertools;
use std::cmp::{Ord, Ordering};

/// Return the position of the first pair in `pairs` whose first element is `x`.
///
/// If no such pair is present, return the lenth of `pairs`.
///
/// # Pre-condition
/// `pairs` is sorted using the lexicographic order on pairs.
pub fn first_pair(pairs: &[[u64; 2]], x: u64) -> usize {
    let len = pairs.len();
    match pairs.binary_search(&[x, 0]) {
        Ok(i) => i,
        Err(i) if i < len && pairs[i][0] == x => i,
        _ => len,
    }
}

/// Merge sort a with b, without duplicate.
pub fn merge_sort<T>(a: Vec<T>, b: Vec<T>) -> Vec<T>
where
    T: Clone + Ord,
{
    if a.is_empty() {
        return b;
    }
    if b.is_empty() {
        return a;
    }
    let len_a = a.len();
    let len_b = b.len();
    let mut r = Vec::with_capacity(len_a.max(len_b));
    let mut ia = 0;
    let mut ib = 0;
    while ia < len_a && ib < len_b {
        match a[ia].cmp(&b[ib]) {
            Ordering::Less => {
                r.push(a[ia].clone());
                ia += 1;
            }
            Ordering::Equal => {
                r.push(a[ia].clone());
                ia += 1;
                ib += 1;
            }
            Ordering::Greater => {
                r.push(b[ib].clone());
                ib += 1;
            }
        }
    }
    if ia < len_a {
        r.extend(a[ia..].iter().cloned());
    }
    else if ib < len_b {
        r.extend(b[ib..].iter().cloned());
    }
    r
}

/// Sort the pairs and remove duplicates.
///
/// # Return value
/// Return the new size of pairs (once duplicates have been removed).
pub fn bucket_sort_pairs(pairs: &mut Vec<[u64; 2]>) -> usize {
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

fn build_hist(pairs: &[[u64; 2]], min: u64, pair_elem: usize, hist: &mut [usize]) {
    for pair in pairs {
        hist[(pair[pair_elem] - min) as usize] += 1;
    }
}

fn build_cumul(hist: &[usize], cumul: &mut [usize]) {
    for i in 1..hist.len() {
        cumul[i] = cumul[i - 1] + hist[i - 1];
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_merge_sort() {
        assert_eq!(
            merge_sort(vec![1, 3, 5, 6, 7], vec![2, 3, 4, 8]),
            vec![1, 2, 3, 4, 5, 6, 7, 8],
        );

        assert_eq!(
            merge_sort(vec![1, 3, 5, 6, 7], vec![]),
            vec![1, 3, 5, 6, 7],
        );

        assert_eq!(
            merge_sort(vec![], vec![1, 3, 5, 6, 7]),
            vec![1, 3, 5, 6, 7],
        );
    }

    #[test]
    fn test_bucket_sort() {
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