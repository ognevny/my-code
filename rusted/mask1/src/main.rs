use {rayon::prelude::*, regex::Regex};

#[allow(dead_code)]
fn fn_match() -> Result<Vec<usize>, Box<dyn std::error::Error>> {
    let mask = Regex::new("^123.*567.?$")?;
    Ok((169..1_000_000_000usize)
        .into_par_iter()
        .filter(|x| x % 169 == 0 && mask.is_match(&x.to_string()))
        .collect())
}

fn main() {
    println!("mask1: run `cargo test` instead");
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_all() {
        let mask = fn_match().unwrap();
        assert_eq!(mask, [
            12325677, 12385672, 123157567, 123165679, 123225674, 123326567, 123495567, 123515678,
            123575673, 123664567, 123833567, 123865677, 123925672
        ])
    }
}
