#![no_std]

const LOOKUP: [usize; 12] = [0, 3, 2, 5, 0, 3, 5, 1, 4, 6, 2, 4];

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
