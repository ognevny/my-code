const RADIX: [char; 16] = [
    '0', '1', '2', '3', '4', '5', '6', '7',
    '8', '9', 'A', 'B', 'C', 'D', 'E', 'F'];


/// translate int number from DEC to radix (2 <= k <= 10)
///
/// # Examples
///
/// ```
/// use ognlib::nums::radix::from_int_to_radix;
///
/// assert_eq!(from_int_to_radix(123, 8), 173);
/// assert_eq!(from_int_to_radix(176, 2), 10110000)
/// ```
pub fn from_int_to_radix(mut n: u64, k: u8) -> u64 {
    let mut res = String::new();
    while n != 0 {
        res.push(RADIX[(n % (k as u64)) as usize]);
        n /= k as u64; }
    res.chars().rev().collect::<String>().parse().unwrap() }


/// translate int number from DEC to radix string (2 <= k <= 16)
///
/// # Examples
///
/// ```
/// use ognlib::nums::radix::from_int_to_radix_string;
///
/// assert_eq!(from_int_to_radix_string(123, 16), "7B");
/// assert_eq!(from_int_to_radix_string(176, 2), "10110000")
/// ```
pub fn from_int_to_radix_string(mut n: u64, k: u8) -> String {
    let mut res = String::new();
    while n != 0 {
        res.push(RADIX[(n % (k as u64)) as usize]);
        n /= k as u64; }
    res.chars().rev().collect::<String>() }


/// translate int number from radix to int DEC number (2 <= k <= 10)
///
/// # Examples
///
/// ```
/// use ognlib::nums::radix::from_radix_to_dec;
///
/// assert_eq!(from_radix_to_dec(10110000, 2), 176);
/// assert_eq!(from_radix_to_dec(444, 10), 444)
/// ```
pub fn from_radix_to_dec(mut n: u64, k: u8) -> u64 {
    let mut dec: u64 = 0;
    let mut count = 0;
    while n != 0 {
        dec += (n % 10) * super::power::bin_pow(k as f64, count) as u64;
        n /= 10; 
        count += 1; }
    dec }
