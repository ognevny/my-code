# same as mcko.rs


def n11() -> int:
    RADIX: list = ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "A", "B"]
    for i in RADIX:
        num1, num2 = int(f"154{i}3", 12), int(f"1{i}365", 12)
        if (num1 + num2) % 13 == 0:
            break
    return (num1 + num2) / 13


assert n11() == 4340
