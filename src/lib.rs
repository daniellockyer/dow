#![no_std]

/// Lookup table for day indices.
const LOOKUP: [usize; 12] = [0, 3, 2, 5, 0, 3, 5, 1, 4, 6, 2, 4];

#[derive(Debug, PartialEq)]
pub enum Day {
    Sunday,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday
}

pub fn dow(y: usize, m: usize, d: usize) -> Day {
    /// Return the day which the given date corresponds to.
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
    /// use dow::Day;
    ///
    /// let day = dow::dow(2017, 7, 2);
    /// assert_eq!(day, Day::Sunday);
    /// ```

    let mut yy = y;

    if m < 3 {
        yy -= 1;
    }

    let day = (yy + yy / 4 - yy / 100 + yy / 400 + LOOKUP[m - 1] + d) % 7;
    match day {
        0 => Day::Sunday,
        1 => Day::Monday,
        2 => Day::Tuesday,
        3 => Day::Wednesday,
        4 => Day::Thursday,
        5 => Day::Friday,
        6 => Day::Saturday,
        _ => unreachable!()
    }
}


#[test]
fn it_works() {
    assert_eq!(dow(2017, 7, 2), Day::Sunday);
}
