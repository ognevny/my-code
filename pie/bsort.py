# bubble sort

from random import randint
from time import perf_counter

a = []

for r in range(10000):
    a.append(randint(-1000, 1000))


def b_sort(a):
    i = 0
    while True:
        w = 0
        for j in range(0, len(a) - i - 1):
            if a[j] > a[j + 1]:
                q = a[j + 1]
                a[j + 1] = a[j]
                a[j] = q
                w += 1
        if w == 0:
            break
        i += 1
    return a


start = perf_counter()
b = b_sort(a)
print(perf_counter() - start)
