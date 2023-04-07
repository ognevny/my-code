from math import log2, ceil, sqrt, factorial
from random import sample

def Miller_Rabin(number):
    if number < 2:
        return "ERROR"
    elif number == 2 or number == 3:
        return "YES"
    elif number % 2 == 0:
        return "NO"
    else:
        if number < 6:
            k = number - 3
        else:
            k = (ceil(log2(number))) ** 2
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
            for _ in range (s - 1):
                x = pow(x, 2, number)
                if x == 1:
                    return "NO"
                if x == number - 1:
                    break
        return "PROBABLY"

def sqrtest(number):
    if number < 2:
        return "ERROR"
    elif number == 2:
        return "YES"
    else:
        counter = 0
        for i in range(1, ceil(sqrt(number)) + 1):
            if number % i == 0:
                counter += 1
        if counter == 1:
            return "YES"
        else:
            return "NO"

def Wilson(number):
    if number < 2:
        return "ERROR"
    elif (factorial(number - 1) + 1) % number == 0:
        return "YES"
    else:
        return "NO"


n = int(input())
print(Miller_Rabin(n), sqrtest(n), Wilson(n))