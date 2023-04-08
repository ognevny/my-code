// same list of useful functions in Rust

pub mod num_operation {
    pub mod num_sys {
        const SYS: [char; 16] = [
            '0', '1', '2', '3', '4', '5', '6', '7',
            '8', '9', 'A', 'B', 'C', 'D', 'E', 'F'
        ];

        pub fn num_sys(mut n: u64, k: u8) -> u64 {
            let mut res = String::new();
            while n != 0 {
                res.push(SYS[(n % (k as u64)) as usize]);
                n /= k as u64; }
            res.chars().rev().collect::<String>().parse().unwrap() }

        pub fn num_sys_hex(mut n: u64, k: u8) -> String {
            let mut res = String::new();
            while n != 0 {
                res.push(SYS[((n % (k as u64)) as usize)]);
                n /= k as u64; }
            res.chars().rev().collect::<String>() } }

    pub mod num_digits {
        pub fn num_sum(mut n: u64) -> u32 {
            let mut sum: u32 = 0;
            while n != 0 {
                sum += (n % 10) as u32;
                n /= 10; }
            sum }

        pub fn num_count(mut n: u64) -> u16 {
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
        rev } }

    pub mod bin_pow {
        pub fn bin_pow(mut b: f64, mut e: u16) -> f64 {
            let mut v: f64 = 1.0;
            while e != 0 {
                if (e & 1) != 0 { v *= b }
                b *= b;
                e >>= 1; }
        v } } }
