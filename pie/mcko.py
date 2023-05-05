# same as mcko.rs
# https://mcko.ru/uploads/documents/informatika_-10-klass_komplekt_1-91f3befffaba6eed.zip


def n11() -> int:
    RADIX: list = ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "A", "B"]
    for i in RADIX:
        num1, num2 = int(f"154{i}3", 12), int(f"1{i}365", 12)
        if (num1 + num2) % 13 == 0:
            break
    return (num1 + num2) / 13


def n12() -> tuple:
    file = open("C:\\msys64\\home\\maksa\\my-code\\12.txt")

    data = []
    min, count, max = 100_000, 0, 0

    for i in file:
        i = int(i.strip())
        data.append(i)
        if i < min and i % 15 != 0:
            min = i
    file.close()

    for i in range(len(data) - 1):
        t1, t2 = data[i], data[i + 1]
        if t1 % min == 0 and t2 % min == 0:
            count += 1
            if t1 + t2 > max:
                max = t1 + t2

    return (count, max)


assert n11() == 4340
assert n12() == (157, 176024)
