// Copyright © 2019 Bart Massey
// [This program is licensed under the "MIT License"]
// Please see the file LICENSE in the source
// distribution of this software for license terms.

use trailing_zeros::trailing_zeros;
use rand::prelude::*;

#[test]
fn test_extremes() {
    assert_eq!(64, trailing_zeros(0));
    assert_eq!(63, trailing_zeros(1 << 63));
    assert_eq!(1, trailing_zeros(!0 - 1));
    assert_eq!(0, trailing_zeros(!0));
}

fn trailing_zeros_naive(n: u64) -> u64 {
    for i in 0..64 {
        if n & (1 << i) != 0 {
            return i;
        }
    }
    return 64;
}

#[test]
fn test_random() {
    let mut rng = rand::thread_rng();
    for _ in 0..100000 {
        let n: u64 = rng.gen();
        assert_eq!(trailing_zeros_naive(n), trailing_zeros(n));
    }
}

