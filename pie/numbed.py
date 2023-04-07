from numba import njit


@njit(fastmath=True)
def s() -> int:
    n = 1
    while n < 1_000_000_000:
        n += 1
    return n


if __name__ == "__main__":
    n = s()
    print(n)
