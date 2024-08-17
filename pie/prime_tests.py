# task: if number is prime?

from math import ceil, factorial, log2, sqrt
from random import sample


def Miller_Rabin(number: int) -> str:
    if number < 2:
        return "ERROR"

    if number in (2, 3, 5):
        return "YES"

    if number % 2 == 0:
        return "NO"

    k: int = (ceil(log2(number))) ** 2
    t = number - 1
    s = 0
    while t % 2 == 0:
        t //= 2
        s += 1
    a = sample(range(2, number - 1), k)
    for i in range(k):
        x = pow(a[i], t, number)
        if x == 1 or x == number - 1:
            continue
        for _ in range(s - 1):
            x = pow(x, 2, number)
            if x == 1:
                return "NO"
            if x == number - 1:
                break

    return "PROBABLY"


def sqrtest(number: int) -> str:
    if number < 2:
        return "ERROR"

    if number == 2:
        return "YES"

    if number % 2 == 0:
        return "NO"

    for i in range(3, ceil(sqrt(number)), 2):
        if number % i == 0:
            return "YES"

    return "NO"


def Wilson(number: int) -> str:
    if number < 2:
        return "ERROR"

    if (factorial(number - 1) + 1) % number == 0:
        return "YES"

    return "NO"


n = int(input())
print(Miller_Rabin(n), sqrtest(n), Wilson(n))
