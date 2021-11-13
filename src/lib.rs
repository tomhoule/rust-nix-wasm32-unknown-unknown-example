use wasm_bindgen::prelude::*;

/// Computes the [greatest common divisor](https://en.wikipedia.org/wiki/Greatest_common_divisor)
/// of two integers using Euclid's algorithm.
///
/// ```
/// # use gcd::*;
/// assert_eq!(gcd(3, 3), 3);
/// assert_eq!(gcd(4, 2), 2);
/// assert_eq!(gcd(20, 15), 5);
/// assert_eq!(gcd(-20, 15), 5);
/// assert_eq!(gcd(-20, -15), 5);
/// assert_eq!(gcd(252, 105), 21);
/// assert_eq!(gcd(105, 252), 21);
/// assert_eq!(gcd(-48, 18), 6);
/// ```
#[wasm_bindgen]
pub fn gcd(a: i64, b: i64) -> u64 {
    if b == 0 {
        a.unsigned_abs()
    } else {
        gcd(b, a % b)
    }
}
