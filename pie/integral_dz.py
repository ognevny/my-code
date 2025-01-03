# algorithms for oint

from collections.abc import Callable
from math import pi


def integral_rectangles(function: Callable[[float], float], left: int, right: int, n: int) -> float:
    dx = (right - left) / n
    integral = 0.0
    for i in range(n):
        integral += function((2 * left + dx * i + dx * (i + 1)) / 2) * dx
    return integral


def integral_parabolas(function: Callable[[float], float], left: int, right: int, n: int) -> float:
    dx = (right - left) / n
    integral = 0
    for i in range(n):
        integral += (dx / 6) * (
            4 * function((2 * left + dx * i + dx * (i + 1)) / 2)
            + function(left + dx * i)
            + function(left + dx * (i + 1))
        )
    return integral


def integral_trapezoids(function: Callable[[float], float], left: int, right: int, n: int) -> float:
    dx = (right - left) / n
    integral = 0
    for i in range(n):
        integral += (dx / 2) * (function(left + dx * i) + function(left + dx * (i + 1)))
    return integral


def f(x: float) -> float:
    return 1 / (1 + x**2)


print(integral_rectangles(f, -1, 1, 10000))
print(integral_parabolas(f, -1, 1, 10000))
print(integral_trapezoids(f, -1, 1, 10000))
print(pi / 2)
