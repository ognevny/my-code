const SYS: [char; 16] = [
    '0', '1', '2', '3', '4', '5', '6', '7',
    '8', '9', 'A', 'B', 'C', 'D', 'E', 'F' ];

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