//! Power algorithms

use std::ops::MulAssign;

/// Algorithm for binary power. Remember that returned value has the same type, so it can fail with overflowing.
/// # Examples
///
/// ```
/// use ognlib::num::power::bin_pow;
///
/// assert_eq!(bin_pow(123, 3), 1860867);
/// assert_eq!(bin_pow(0.5, 4), 0.0625);
/// ```

pub fn bin_pow<N>(mut b: N, mut e: u16) -> N
where
    N: MulAssign + From<u16> + Copy + PartialEq + Sized,
{
    let mut v = N::from(1);
    while e != 0 {
        if N::from(e & 1) != N::from(0) {
            v *= b
        }
        b *= b;
        e >>= 1;
    }
    v
}
