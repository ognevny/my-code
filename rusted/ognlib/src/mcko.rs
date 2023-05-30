//! Russian text ahead:
//! Данный код решает номера из диагностики МЦКО - для московских школьников.
//! Хоть и кажется, что лучше использовать Python для всего этого, все равно Rust достаточно
//! высокоуровненвый, чтобы у нас была возможность более-менее просто решать эти номера.
//! Номера взяты из
//! [комплекта](https://mcko.ru/uploads/documents/informatika_-10-klass_komplekt_1-91f3befffaba6eed.zip).
//! Здесь приведены решения лишь 4 номеров, так как остальные либо вообще не решаются программированием,
//! либо гораздо быстрее решаются руками.

/// #### Номер 2
/// Автомат обрабатывает натуральное число N по следующему алгоритму:
/// 1) Строится двоичная запись числа N.
/// 2) К полученной записи дописываются разряды по следующему принципу:
/// если число чётное, то справа дописывается 10, если нечётное – слева
/// дописывается 1 и справа 00.
/// 3) Результат переводится в десятичную систему и выводится на экран.
/// В результате работы автомата на экране появилось число, большее 107.
/// Для какого наименьшего N данная ситуация возможна? В ответе найденное
/// число N запишите в десятичной системе.
///
/// # Ответ
///
/// ```
/// use ognlib::mcko::n2;
///
/// assert_eq!(n2(1), 11);
/// ```

pub fn n2(n: u16) -> u16 {
    let m = format!("{n:b}");
    let m = if n % 2 == 0 {
        format!("{m}10")
    } else {
        format!("1{m}00")
    };
    let m = u32::from_str_radix(&m, 2).unwrap();
    if m > 107 {
        n
    } else {
        n2(n + 1)
    }
}

/// #### Номер 10
/// Редактор получает на вход строку цифр и преобразовывает её. Редактор
/// может выполнять две команды, в обеих командах v и w обозначают цепочки цифр.
/// заменить (v, w) Эта команда заменяет в строке первое слева вхождение
/// цепочки v на цепочку w.
/// нашлось (v) Эта команда проверяет, встречается ли цепочка v в строке
/// исполнителя Редактор. Если она встречается, то команда
/// возвращает логическое значение «истина», в противном
/// случае возвращает значение «ложь». Строка при этом
/// не изменяется.
/// Дана программа для исполнителя Редактор:
/// НАЧАЛО
/// ПОКА нашлось (35) ИЛИ нашлось (355) ИЛИ нашлось (3444)
///  ЕСЛИ нашлось (35)
///  ТО заменить (35, 4)
///  ИНАЧЕ
///  ЕСЛИ нашлось (355)
///  ТО заменить (355, 4)
///  ИНАЧЕ заменить (3444, 3)
///  КОНЕЦ ЕСЛИ
///  КОНЕЦ ЕСЛИ
/// КОНЕЦ ПОКА
/// КОНЕЦ
/// Какая строка получится в результате применения приведённой выше
/// программы к строке вида 3…34…4 (6 троек и 75 четвёрок)? В ответе
/// запишите полученную строку.
///
/// # Ответ
///
/// ```
/// use ognlib::mcko::n10;
///
/// assert_eq!(n10(), "333333");
/// ```

pub fn n10() -> String {
    let mut input = String::from(
        "333333444444444444444444444444444444444444444444444444444444444444444444444444444",
    );
    while input.contains("35") || input.contains("355") || input.contains("3444") {
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

/// #### Номер 11
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
/// ```

pub fn n11() -> u32 {
    for i in 0..12 {
        let (num1, num2) = (
            u32::from_str_radix(&format!("154{i:X}3"), 12).unwrap(),
            u32::from_str_radix(&format!("1{i:X}365"), 12).unwrap(),
        );
        if (num1 + num2) % 13 == 0 {
            return (num1 + num2) / 13;
        }
    }
    0
}

/// #### Номер 12
/// В файле 12.txt содержится последовательность целых чисел. Элементы
/// последовательности могут принимать целые значения от –100 000 до 100 000
/// включительно. Пусть N – минимальное число в последовательности,
/// НЕ кратное 15. Определите количество пар элементов последовательности,
/// в которых оба числа кратны N. В ответе запишите количество найденных пар,
/// затем максимальную из сумм элементов таких пар. В данной задаче под парой
/// подразумевается два идущих подряд элемента последовательности.
///
/// # Ответ
///
/// ```
/// use ognlib::mcko::n12;
///
/// assert_eq!(n12(), (157, 176024));
/// ```

pub fn n12() -> (i32, i32) {
    use std::fs;

    let data = fs::read_to_string("12.txt").unwrap();
    let data = data
        .lines()
        .map(|line| line.trim().parse().unwrap())
        .collect::<Vec<i32>>();

    let (min, mut count, mut max) = (*data.iter().filter(|x| *x % 15 != 0).min().unwrap(), 0, 0);

    for i in 0..data.len() - 1 {
        let (t1, t2) = (data[i], data[i + 1]);
        if t1 % min == 0 && t2 % min == 0 {
            count += 1;
            max = max.max(t1 + t2);
        }
    }
    (count, max)
}
