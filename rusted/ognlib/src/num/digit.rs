pub fn digit_sum(mut n: u64) -> u32 {
    let mut sum: u32 = 0;
    while n != 0 {
        sum += (n % 10) as u32;
        n /= 10; }
    sum }

pub fn digit_count(mut n: u64) -> u16 {
    let mut count: u16 = 0;
    while n != 0 {
        n /= 10;
        count += 1; }
    count }

pub fn rev(mut n: u64) -> u64 {
    let mut rev: u64 = 0;
    while n != 0 {
        rev = rev * 10 + (n % 10);
        n /= 10; }
    rev }

pub fn has_digit(mut n: u64, k: u8) -> bool {
    while n != 0 {
        if n % 10 == k.into() { return true; }
        n /= 10}
    false }
