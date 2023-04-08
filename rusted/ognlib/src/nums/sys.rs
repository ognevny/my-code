const SYS: [char; 16] = [
    '0', '1', '2', '3', '4', '5', '6', '7',
    '8', '9', 'A', 'B', 'C', 'D', 'E', 'F'];

pub fn num_sys(mut n: u64, k: u8) -> u64 {
    let mut res = String::new();
    while n != 0 {
        res.push(SYS[(n % (k as u64)) as usize]);
        n /= k as u64; }
    res.chars().rev().collect::<String>().parse().unwrap() }

pub fn num_sys_hex(mut n: u64, k: u8) -> String {
    let mut res = String::new();
    while n != 0 {
        res.push(SYS[(n % (k as u64)) as usize]);
        n /= k as u64; }
    res.chars().rev().collect::<String>() }

pub fn num_sys_dec(mut n: u64, k: u8) -> u64 { // translate into DEC number from k system (2 <= k <= 10)
    let mut dec: u64 = 0;
    let mut count = 0;
    while n != 0 {
        dec += (n % 10) * super::power::bin_pow(k as f64, count) as u64;
        n /= 10; 
        count += 1; }
    dec }
