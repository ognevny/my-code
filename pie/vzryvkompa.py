from random import randint
from time import perf_counter

a = [randint(-10000, 10000) for _ in range(10000000)]

start = perf_counter()
a.sort()
end = perf_counter()
print(end - start)
