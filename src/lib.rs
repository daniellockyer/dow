#![no_std]

/// Lookup table for day indices.
const LOOKUP: [usize; 12] = [0, 3, 2, 5, 0, 3, 5, 1, 4, 6, 2, 4];

/// Return the index of the day for which the given date corresponds to.
///
/// # Arguments
/// 
/// * `y`: Year
/// * `m`: Month
/// * `d`: Day
///
/// # Example
///
/// ```
/// extern crate dow;
///
/// let day_index = dow::dow(2017, 7, 2);
/// assert_eq!(day_index, 0); // 2017/07/02 is a Sunday
/// ```
///
/// # Note
///
///	Sunday is `0`, Monday is `1` etc.
pub fn dow(y: usize, m: usize, d: usize) -> usize {
    let mut yy = y;

    if m < 3 {
        yy -= 1;
    }

    (yy + yy / 4 - yy / 100 + yy / 400 + LOOKUP[m - 1] + d) % 7
}

#[test]
fn it_works() {
    assert_eq!(dow(2017, 7, 2), 0);
}
