# a programm to test speed of each language (Python implementation)


def s() -> int:
    n = 1
    while n < 1_000_000_000:
        n += 1
    return n


if __name__ == "__main__":
    print(s())
