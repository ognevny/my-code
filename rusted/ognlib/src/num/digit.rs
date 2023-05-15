//! Functions for operations with digits

use std::ops::{Add, AddAssign, DivAssign, Mul, Rem};

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
    N: Rem<Output = N> + DivAssign + AddAssign + From<u8> + Copy + Eq,
{
    let mut sum = N::from(0);
    while n != N::from(0) {
        sum += n % N::from(10);
        n /= N::from(10);
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

pub fn digit_count<N>(mut n: N) -> u8
where
    N: DivAssign + From<u8> + Eq + Copy,
{
    let mut count = 0;
    while n != N::from(0) {
        n /= N::from(10);
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
    N: Mul<Output = N> + Add<Output = N> + Rem<Output = N> + DivAssign + From<u8> + Copy + Eq,
{
    let mut rev = N::from(0);
    while n != N::from(0) {
        rev = rev * N::from(10) + (n % N::from(10));
        n /= N::from(10);
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
    N: Rem<Output = N> + DivAssign + Eq + From<u8> + Copy,
{
    while n != N::from(0) {
        if n % N::from(10) == N::from(k) {
            return true;
        }
        n /= N::from(10);
    }
    false
}
