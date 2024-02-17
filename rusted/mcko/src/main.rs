//! Russian text ahead: данный код решает номера из диагностики МЦКО - для московских школьников.
//! Хоть и кажется, что лучше использовать Python для всего этого, Rust достаточно высокоуровневый,
//! чтобы у нас была возможность более-менее просто решать эти номера. Номера взяты из
//! [комплекта](https://mcko.ru/uploads/documents/informatika_-10-klass_komplekt_1-91f3befffaba6eed.zip).
//! Здесь приведены решения лишь 4 номеров, так как остальные либо вообще не решаются
//! программированием, либо гораздо быстрее решаются руками.

use {
    rayon::prelude::*,
    std::{
        fs::File,
        io::{BufRead, BufReader},
    },
};

// Номер 2
// Автомат обрабатывает натуральное число N по следующему алгоритму:
// 1) Строится двоичная запись числа N.
// 2) К полученной записи дописываются разряды по следующему принципу: если число чётное, то справа
// дописывается 10, если нечётное – слева дописывается 1 и справа 00.
// 3) Результат переводится в десятичную систему и выводится на экран. В результате работы автомата
// на экране появилось число, большее 107. Для какого наименьшего N данная ситуация возможна? В
// ответе найденное число N запишите в десятичной системе.
//
// Ответ: 11

pub fn n2(mut n: u16) -> u16 {
    loop {
        let m = u32::from_str_radix(
            &if n % 2 == 0 { format!("{n:b}10") } else { format!("1{n:b}00") },
            2,
        )
        .unwrap();
        if m > 107 {
            return n;
        }
        n += 1;
    }
}

// Номер 10
// Редактор получает на вход строку цифр и преобразовывает её. Редактор может выполнять две
// команды, в обеих командах v и w обозначают цепочки цифр. Заменить (v, w) Эта команда заменяет
// в строке первое слева вхождение цепочки v на цепочку w. Нашлось (v) Эта команда
// проверяет, встречается ли цепочка v в строке исполнителя Редактор. Если она встречается, то
// команда возвращает логическое значение «истина», в противном случае возвращает значение «ложь».
// Строка при этом не изменяется. Дана программа для исполнителя Редактор:
// НАЧАЛО
// ПОКА нашлось (35) ИЛИ нашлось (355) ИЛИ нашлось (3444)
//  ЕСЛИ нашлось (35)
//  ТО заменить (35, 4)
//  ИНАЧЕ
//  ЕСЛИ нашлось (355)
//  ТО заменить (355, 4)
//  ИНАЧЕ заменить (3444, 3)
//  КОНЕЦ ЕСЛИ
//  КОНЕЦ ЕСЛИ
// КОНЕЦ ПОКА
// КОНЕЦ
// Какая строка получится в результате применения приведённой выше программы к строке вида 3…34…4
// (6 троек и 75 четвёрок)? В ответе запишите полученную строку.
//
// Ответ: 333333

pub fn n10() -> String {
    let mut input = String::from(
        "333333444444444444444444444444444444444444444444444444444444444444444444444444444",
    );
    while ["35", "355", "3444"].iter().any(|&s| input.contains(s)) {
        if input.contains("35") {
            input = input.replace("35", "4");
        } else if input.contains("355") {
            input = input.replace("355", "4");
        } else {
            input = input.replace("3444", "3");
        }
    }
    input
}

// Номер 11
// Операнды арифметического выражения записаны в системе счисления с основанием 12:
// 154x3_12 + 1x365_12. В записи чисел переменной x обозначена неизвестная цифра из алфавита
// двенадцатеричной системы счисления. Определите значение x, при котором значение данного
// арифметического выражения кратно 13. Для найденного значения x вычислите частное от деления
// значения арифметического выражения на 13 и укажите его в ответе в десятичной системе счисления.
// Основание системы счисления в ответе указывать не нужно.
//
// Ответ: 4340

pub fn n11() -> u32 {
    (0..12)
        .find_map(|i| {
            let sum = u32::from_str_radix(&format!("154{i:X}3"), 12).unwrap()
                + u32::from_str_radix(&format!("1{i:X}365"), 12).unwrap();
            if sum % 13 == 0 { Some(sum / 13) } else { None }
        })
        .unwrap()
}

// Номер 12
// В файле 12.txt содержится последовательность целых чисел. Элементы последовательности могут
// принимать целые значения от –100 000 до 100 000 включительно. Пусть N – минимальное число в
// последовательности, НЕ кратное 15. Определите количество пар элементов последовательности, в
// которых оба числа кратны N. В ответе запишите количество найденных пар, затем максимальную из
// сумм элементов таких пар. В данной задаче под парой подразумевается два идущих подряд элемента
// последовательности.
//
// Ответ: (157, 176024)

pub fn n12() -> (i32, i32) {
    let data: Vec<i32> = BufReader::new(File::open("12.txt").unwrap())
        .lines()
        .map(|line| line.unwrap().trim().parse().unwrap())
        .collect();

    let min = *data.par_iter().filter(|&&x| x % 15 != 0).min().unwrap();

    data.par_windows(2)
        .filter(|t| t[0] % min == 0 && t[1] % min == 0)
        .fold(|| (0, 0), |(count, max), t| (count + 1, max.max(t[0] + t[1])))
        .reduce(|| (0, 0), |(count1, max1), (count2, max2)| (count1 + count2, max1.max(max2)))
}

fn main() {
    assert_eq!(n2(1), 11);
    assert_eq!(n10(), "333333");
    assert_eq!(n11(), 4340);
    assert_eq!(n12(), (157, 176024));
}
