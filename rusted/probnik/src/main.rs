use itertools::Itertools;

fn main() {
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
    println!("{k}");
}
