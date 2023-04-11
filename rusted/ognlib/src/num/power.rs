/// Algorithm for binary power
/// # Examples
/// 
/// ```
/// use ognlib::num::power::bin_pow;
///
/// assert_eq!(bin_pow(123.0, 3), 1860867.0);
/// assert_eq!(bin_pow(0.5, 4), 0.0625);
/// ```
pub fn bin_pow(mut b: f64, mut e: u16) -> f64 {
    let mut v: f64 = 1.0;
    while e != 0 {
        if (e & 1) != 0 { v *= b }
        b *= b;
        e >>= 1; }
v }
