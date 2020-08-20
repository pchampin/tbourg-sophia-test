//! Bunch of utility functions

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
pub fn merge_sort_tmp<T>(a: &[T], b: &[T]) -> Vec<T>
where
    T: Clone + Ord,
{
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
}