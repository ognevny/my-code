from __future__ import annotations

from math import atan, cos, pi, sin, sqrt


class CNumb:
    def __init__(self, real: float = 1.0, im: float = 0.0) -> None:
        self.real = real
        self.im = im

    def mod(self) -> float:
        return sqrt(self.real**2 + self.im**2)

    def arg(self) -> float:
        if self.real > 0 and self.im >= 0:
            return atan(self.im / self.real)

        if self.real > 0 and self.im < 0:
            return atan(self.im / self.real) + 2 * pi

        if self.real < 0:
            return atan(self.im / self.real) + pi

        if self.im < 0:
            return 1.5 * pi

        if self.im > 0:
            return pi * 0.5

        return 0.0

    def __add__(self, a: float | CNumb) -> CNumb:
        if isinstance(a, CNumb):
            return CNumb(self.real + a.real, self.im + a.im)

        return CNumb(self.real + a, self.im)

    def __radd__(self, a: float | CNumb) -> CNumb:
        if isinstance(a, CNumb):
            return CNumb(self.real + a.real, self.im + a.im)

        return CNumb(self.real + a, self.im)

    def __mul__(self, a: float | CNumb) -> CNumb:
        if isinstance(a, CNumb):
            return CNumb(self.real * a.real - self.im * a.im, self.real * a.im + self.im * a.real)

        return CNumb(self.real * a, self.im * a)

    def __rmul__(self, a: float | CNumb) -> CNumb:
        if isinstance(a, CNumb):
            return CNumb(self.real * a.real - self.im * a.im, self.real * a.im + self.im * a.real)

        return CNumb(self.real * a, self.im * a)

    def con(self) -> CNumb:
        return CNumb(self.real, -1 * self.im)

    def __truediv__(self, a: float | CNumb) -> CNumb:
        if isinstance(a, CNumb):
            num = self * a.con()
            denom = (a * a.con()).real
            return num * (denom ** (-1))

        return self * (a ** (-1))

    def __sub__(self, a: float | CNumb) -> CNumb:
        if isinstance(a, CNumb):
            return CNumb(self.real - a.real, self.im - a.im)

        return CNumb(self.real - a, self.im)

    def __rsub__(self, a: float | CNumb) -> CNumb:
        if isinstance(a, CNumb):
            return CNumb(self.real - a.real, self.im - a.im)

        return CNumb(self.real - a, self.im)

    def __pow__(self, a: float) -> CNumb:
        return CNumb(
            (self.mod() ** a) * (cos(atan(a * self.arg()))),
            (self.mod() ** a) * (sin(atan(a * self.arg()))),
        )

    def rad(self, a: int) -> list[str]:
        return [
            str((self.mod() ** (1 / a)) * (cos(atan((self.arg() + 2 * pi * j) / a))))
            + "+"
            + str((self.mod() ** (1 / a)) * (sin(atan((self.arg() + 2 * pi * j) / a))))
            + "i"
            for j in range(a)
        ]

    def __str__(self) -> str:
        return f"{self.real}+{self.im}i"


n = CNumb(1, 2)
m = CNumb(1, 1)
print(n + m)
print(n * m)
print(n - m)
print(m**2)
print(m.rad(2))
