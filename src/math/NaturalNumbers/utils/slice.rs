/// Safely splits a slice even if it is shorter than M
pub(crate) fn safe_split(slice: &[u128], m: usize) -> (&[u128], &[u128]) {
    match m >= slice.len() {
        true => (slice, &[]),
        false => slice.split_at(m),
    }
}

/// Removes trailing zeros from a vector
pub(crate) fn pop_leading_zeros(slice: &mut Vec<u128>) {
    while slice.last() == Some(&0) && slice.len() > 1 {
        slice.pop();
    }
}
