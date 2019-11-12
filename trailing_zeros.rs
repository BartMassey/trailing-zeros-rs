// Copyright © 2019 Bart Massey
// [This program is licensed under the "MIT License"]
// Please see the file LICENSE in the source
// distribution of this software for license terms.

/// Return the count of "trailing zeros" in a 64-bit binary
/// number.
pub fn trailing_zeros(mut n: u64) -> u64 {
    let mut nmask = 32;
    let mut result = 0;
    for _ in 0..6 {
        let mask = (1 << nmask) - 1;
        if n & mask == 0 {
            result += nmask;
            n >>= nmask;
        }
        nmask >>= 1;
    }
    if n == 0 {
        result += 1;
    }
    result
}
