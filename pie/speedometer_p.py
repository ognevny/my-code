# programm to test speed of each language (Python implementation with mlp)

import multiprocessing as mlp


def s() -> int:
    n = 1
    while n < 1_000_000_000:
        n += 1
    return n


if __name__ == "__main__":
    with mlp.Pool(processes=mlp.cpu_count()):
        pass
    n = s()
    print(n)
