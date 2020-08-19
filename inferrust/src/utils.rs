//! Bunch of utility functions

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
