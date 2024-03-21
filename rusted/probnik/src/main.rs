use itertools::Itertools;

#[allow(dead_code)]
fn probnik() -> usize {
    let mut k = 0;
    for word in "ПРОБНИК".chars().permutations(7) {
        let len = word.len();
        if !"ОИ".contains(word[0]) && !"ОИ".contains(word[len - 1]) {
            let mut f = true;
            for i in 0..len - 1 {
                if ["ОИ", "ИО"]
                    .into_iter()
                    .any(|s| s.contains(&word[i..i + 2].iter().collect::<String>()))
                {
                    f = false;
                    break;
                }
            }
            if f {
                k += 1;
            }
        }
    }
    k
}

fn main() {
    println!("probnik: run `cargo test` instead");
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_all() {
        assert_eq!(probnik(), 1440);
    }
}
