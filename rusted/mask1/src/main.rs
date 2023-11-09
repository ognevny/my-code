use {
    regex::Regex,
    std::io::{self, BufWriter, Write},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mask = Regex::new(r"^123.*567.?$")?;
    let mut handle = BufWriter::new(io::stdout());
    for num in (169..1000000000usize).step_by(169) {
        if mask.is_match(&num.to_string()) {
            writeln!(handle, "{num} {}", num / 169)?;
        }
    }
    handle.flush()?;
    Ok(())
}
