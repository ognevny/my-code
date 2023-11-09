use {rayon::prelude::*, regex::Regex};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mask = Regex::new(r"^123.*567.?$")?;
    let res: Vec<usize> = (169..1000000000usize)
        .into_par_iter()
        .filter(|x| x % 169 == 0 && mask.is_match(&x.to_string()))
        .collect();
    println!("{res:?}");
    Ok(())
}
