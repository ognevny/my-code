/// Calculate sum of digits in number
/// # Examples
///
/// ```
/// use ognlib::num::digit::digit_sum;
///
/// assert_eq!(digit_sum(123), 6);
/// assert_eq!(digit_sum(444), 12);
/// ```

pub fn digit_sum(mut n: u64) -> u32 {
    let mut sum: u32 = 0;
    while n != 0 {
        sum += (n % 10) as u32;
        n /= 10; }
    sum }


/// Calculate size of number (how many digits it contains)
/// # Examples
///
/// ```
/// use ognlib::num::digit::digit_count;
///
/// assert_eq!(digit_count(123), 3);
/// assert_eq!(digit_count(1337228), 7);
/// ```
 
pub fn digit_count(mut n: u64) -> u16 {
    let mut count: u16 = 0;
    while n != 0 {
        n /= 10;
        count += 1; }
    count }


/// Reverse number
/// # Examples
///
/// ```
/// use ognlib::num::digit::rev;
///
/// assert_eq!(rev(123), 321);
/// assert_eq!(rev(444), 444);
/// ```

pub fn rev(mut n: u64) -> u64 {
    let mut rev: u64 = 0;
    while n != 0 {
        rev = rev * 10 + (n % 10);
        n /= 10; }
    rev }


/// Checks, if digit is in number
/// # Examples
///
/// ```
/// use ognlib::num::digit::has_digit;
///
/// assert_eq!(has_digit(123, 2), true);
/// assert_eq!(has_digit(444, 9), false);
/// ```

pub fn has_digit(mut n: u64, k: u8) -> bool {
    while n != 0 {
        if n % 10 == k.into() { return true; }
        n /= 10}
    false }
