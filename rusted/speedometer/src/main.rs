fn s() -> u32 {
    let mut n = 1;
    while n < 1_000_000_000 { n += 1 }
    n
}

fn main() {
    let n: u32 = s();
    print!("{n}");
}
