# same as mcko.rs


def n11() -> list:
    res = []
    RADIX: list = ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "A", "B"]
    for i, j in enumerate(RADIX):
        num1, num2 = "1" + "5" + "4" + j + "3", "1" + j + "3" + "6" + "5"
        if (int(num1, 12) + int(num2, 12)) % 13 == 0:
            res.append(i)
    return res


assert n11() == [3]
