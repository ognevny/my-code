# Решения нескольких задач из следующего комплекта:
# https://mcko.ru/uploads/documents/informatika_-10-klass_komplekt_1-91f3befffaba6eed.zip


""" Номер 2:
Автомат обрабатывает натуральное число N по следующему алгоритму:
1) Строится двоичная запись числа N.
2) К полученной записи дописываются разряды по следующему принципу:
если число чётное, то справа дописывается 10, если нечётное – слева
дописывается 1 и справа 00.
3) Результат переводится в десятичную систему и выводится на экран.
В результате работы автомата на экране появилось число, большее 107.
Для какого наименьшего N данная ситуация возможна? В ответе найденное
число N запишите в десятичной системе.
"""


def n2(n: int) -> int:
    m = str(bin(n)[2:])
    if n % 2 == 0:
        m = f"{m}10"
    else:
        m = f"1{m}00"
    m = int(m, 2)
    if m > 107:
        return n
    else:
        return n2(n + 1)


""" Номер 10:
Редактор получает на вход строку цифр и преобразовывает её. Редактор
может выполнять две команды, в обеих командах v и w обозначают цепочки
цифр.
заменить (v, w) Эта команда заменяет в строке первое слева вхождение
цепочки v на цепочку w.
нашлось (v) Эта команда проверяет, встречается ли цепочка v в строке
исполнителя Редактор. Если она встречается, то команда
возвращает логическое значение «истина», в противном
случае возвращает значение «ложь». Строка при этом
не изменяется.
Дана программа для исполнителя Редактор:
НАЧАЛО
ПОКА нашлось (35) ИЛИ нашлось (355) ИЛИ нашлось (3444)
 ЕСЛИ нашлось (35)
 ТО заменить (35, 4)
 ИНАЧЕ
 ЕСЛИ нашлось (355)
 ТО заменить (355, 4)
 ИНАЧЕ заменить (3444, 3)
 КОНЕЦ ЕСЛИ
 КОНЕЦ ЕСЛИ
КОНЕЦ ПОКА
КОНЕЦ
Какая строка получится в результате применения приведённой выше
программы к строке вида 3…34…4 (6 троек и 75 четвёрок)? В ответе
запишите полученную строку.
"""


def n10() -> str:
    s = "333333444444444444444444444444444444444444444444444444444444444444444444444444444"
    while "35" in s or "355" in s or "3444" in s:
        if "35" in s:
            s = s.replace("35", "4", 1)
        elif "355" in s:
            s = s.replace("355", "4", 1)
        else:
            s = s.replace("3444", "3", 1)
    return s


""" Номер 11:
Операнды арифметического выражения записаны в системе счисления с основанием 12.
154x3_12 + 1x365_12
В записи чисел переменной x обозначена неизвестная цифра из алфавита
двенадцатеричной системы счисления. Определите значение x, при котором
значение данного арифметического выражения кратно 13. Для найденного
значения x вычислите частное от деления значения арифметического
выражения на 13 и укажите его в ответе в десятичной системе счисления.
Основание системы счисления в ответе указывать не нужно.
"""


def n11() -> int:
    for i in range(12):
        num1, num2 = int(f"154{i:X}3", 12), int(f"1{i:X}365", 12)
        if (num1 + num2) % 13 == 0:
            return (num1 + num2) // 13
    return 0


""" Номер 12:
В файле 12.txt содержится последовательность целых чисел. Элементы
последовательности могут принимать целые значения от –100 000 до 100 000
включительно. Пусть N – минимальное число в последовательности,
НЕ кратное 15. Определите количество пар элементов последовательности,
в которых оба числа кратны N. В ответе запишите количество найденных пар,
затем максимальную из сумм элементов таких пар. В данной задаче под парой
подразумевается два идущих подряд элемента последовательности.
"""


def n12() -> tuple:
    with open("12.txt") as f:
        data = [int(line.strip()) for line in f]

    min_ = min(filter(lambda x: x % 15 != 0, data))
    count, max_ = 0, 0

    for i in range(len(data) - 1):
        t1, t2 = data[i], data[i + 1]
        if t1 % min_ == 0 and t2 % min_ == 0:
            count += 1
            if t1 + t2 > max_:
                max_ = t1 + t2

    return count, max_


assert n2(1) == 11
assert n10() == "333333"
assert n11() == 4340
assert n12() == (157, 176024)
