//! Russian text below
//! Данный код решает номера из диагнстики МЦКО - для московских школьников.
//! Наверное, лучше это всё решать на питоне, но Rust достаточно высокоуровненвый, чтобы решать эти номера.
//! Номера взяты из комплекта:
//! https://mcko.ru/uploads/documents/informatika_-10-klass_komplekt_1-91f3befffaba6eed.zip

// TODO: решить ещё больше номеров

/// #### Номер 11:
/// Операнды арифметического выражения записаны в системе счисления с основанием 12.
/// 154x3_12 + 1x365_12
/// В записи чисел переменной x обозначена неизвестная цифра из алфавита
/// двенадцатеричной системы счисления. Определите значение x, при котором
/// значение данного арифметического выражения кратно 13. Для найденного
/// значения x вычислите частное от деления значения арифметического
/// выражения на 13 и укажите его в ответе в десятичной системе счисления.
/// Основание системы счисления в ответе указывать не нужно.
///
/// # Ответ
///
/// ```
/// use ognlib::mcko::n11;
///
/// assert_eq!(n11(), 4340);

pub fn n11() -> usize {
    const RADIX: &[char] = &['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B'];

    let (mut num1, mut num2): (usize, usize) = (0, 0);
    for num in RADIX {
        (num1, num2) = (
            usize::from_str_radix(&format!("154{num}3"), 12).unwrap(),
            usize::from_str_radix(&format!("1{num}365"), 12).unwrap(),
        );
        if (num1 + num2) % 13 == 0 {
            break;
        }
    }
    (num1 + num2) / 13
}

pub fn n12() -> (usize, i32) {
    use std::{fs::File, io::Read};

    let mut file = File::open("C:\\msys64\\home\\maksa\\my-code\\12.txt").unwrap();

    let mut data: Vec<i32> = Vec::new();
    let (mut min, mut count, mut max, mut contents) = (100_000, 0, 0, String::new());

    file.read_to_string(&mut contents).unwrap();

    for i in contents.lines() {
        let num = i.parse::<i32>().unwrap();
        data.push(num);
        if num < min {
            min = num;
        }
    }
    for i in 0..data.len() - 1 {
        let t1 = data[i];
        let t2 = data[i + 1];
        if t1 % min == 0 && t2 % min == 0 {
            count += 1;
            if t1 + t2 > max {
                max = t1 + t2;
            }
        }
    }
    (count, max)
}
