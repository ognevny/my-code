from __future__ import annotations

from math import acos, sqrt


class Rn:
    def __init__(self, body: list) -> None:
        self.body = body.copy()

    def len(self) -> int:
        return len(self.body)

    def __add__(self, vec: Rn) -> Rn | None:
        if self.len() == vec.len():
            return Rn(list(map(lambda x, y: x + y, self.body, vec.body)))

        return None

    def __mul__(self, vec: Rn | int) -> int | Rn | None:
        if isinstance(vec, Rn):
            if self.len() == vec.len():
                return sum([x[0] * x[1] for x in zip(self.body, vec.body)])

            return None

        return Rn([x * vec for x in self.body])

    def __rmul__(self, vec: Rn | int) -> int | Rn | None:
        if isinstance(vec, Rn):
            if self.len() == vec.len():
                return sum([x[0] * x[1] for x in zip(self.body, vec.body)])

            return None

        return Rn([x * vec for x in self.body])

    def __sub__(self, vec: Rn) -> Rn:
        return self + vec * (-1)

    def norm(self) -> float:
        return sqrt(self * self)

    def angle(self, vec: Rn) -> float:
        return acos((self * vec) / (self.norm() * vec.norm()))

    def __str__(self) -> str:
        return str(self.body)


a = Rn([3, 5, 4])
b = Rn([4, 2, 1])
print(a + b)
print(a * b)
print(a * 2)
print(a - b)
print(a.norm())
print(a.angle(b))
