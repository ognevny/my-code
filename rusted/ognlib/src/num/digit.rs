//! Functions for operations with digits

use std::ops::{Add, AddAssign, Div, Mul, Rem};

/// Calculate sum of digits in number
/// # Examples
///
/// ```
/// use ognlib::num::digit::digit_sum;
///
/// assert_eq!(digit_sum(123), 6);
/// assert_eq!(digit_sum(444), 12);
/// ```

pub fn digit_sum<N>(mut n: N) -> N
where
    N: Rem<Output = N> + Div<Output = N> + AddAssign + From<u8> + Copy + PartialEq,
{
    let mut sum = N::from(0);
    while n != N::from(0) {
        sum += n % N::from(10);
        n = n / N::from(10);
    }
    sum
}

/// Calculate size of number (how many digits it contains)
/// # Examples
///
/// ```
/// use ognlib::num::digit::digit_count;
///
/// assert_eq!(digit_count(123), 3);
/// assert_eq!(digit_count(1337228), 7);
/// ```

pub fn digit_count<N>(mut n: N) -> u16
where
    N: Div<Output = N> + From<u8> + PartialEq + Copy,
{
    let mut count: u16 = 0;
    while n != N::from(0) {
        n = n / N::from(10);
        count += 1;
    }
    count
}

/// Reverse number
/// # Examples
///
/// ```
/// use ognlib::num::digit::rev;
///
/// assert_eq!(rev(123), 321);
/// assert_eq!(rev(444), 444);
/// ```

pub fn rev<N>(mut n: N) -> N
where
    N: Mul<Output = N>
        + Add<Output = N>
        + Rem<Output = N>
        + Div<Output = N>
        + From<u8>
        + Copy
        + PartialEq,
{
    let mut rev: N = N::from(0);
    while n != N::from(0) {
        rev = rev * N::from(10) + (n % N::from(10));
        n = n / N::from(10);
    }
    rev
}

/// Checks, if digit is in number
/// # Examples
///
/// ```
/// use ognlib::num::digit::has_digit;
///
/// assert_eq!(has_digit(123, 2), true);
/// assert_eq!(has_digit(444, 9), false);
/// ```

pub fn has_digit<N>(mut n: N, k: u8) -> bool
where
    N: Rem<Output = N> + Div<Output = N> + PartialEq + From<u8> + Copy,
{
    while n != N::from(0) {
        if n % N::from(10) == N::from(k) {
            return true;
        }
        n = n / N::from(10);
    }
    false
}
