pub fn bin_pow(mut b: f64, mut e: u16) -> f64 {
    let mut v: f64 = 1.0;
    while e != 0 {
        if (e & 1) != 0 { v *= b }
        b *= b;
        e >>= 1; }
v }