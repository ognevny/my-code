from __future__ import annotations

import math


class Polynomials:
    def __init__(self, plnm: list) -> None:
        self.plnm = plnm

    def __add__(self, m: float | Polynomials) -> Polynomials:
        if isinstance(m, Polynomials):
            res = Polynomials(list(map(lambda x, y: x + y, self.plnm, m.plnm)))
            res.plnm.extend(
                self.plnm[len(m.plnm) :]
                if len(self.plnm) > len(m.plnm)
                else m.plnm[len(self.plnm) :],
            )
            return res

        self.plnm[0] += m
        return Polynomials(self.plnm)

    def __mul__(self, arg: float | Polynomials) -> Polynomials:
        if isinstance(arg, Polynomials):
            res = Polynomials([0 for i in range(len(self.plnm) + len(arg.plnm) - 1)])
            for i in range(len(self.plnm)):
                for j in range(len(arg.plnm)):
                    res.plnm[i + j] += self.plnm[i] * arg.plnm[j]
            return res

        return Polynomials([x * arg for x in self.plnm])

    def __rmul__(self, arg: float) -> Polynomials:
        return Polynomials([x * arg for x in self.plnm])

    def simplify(self) -> None:
        while self.plnm[-1] == 0 and len(self.plnm) > 1:
            self.plnm = self.plnm[:-1]

    def __sub__(self, m: float | Polynomials) -> Polynomials:
        return self + (-1) * m

    def __floordiv__(self, arg: Polynomials) -> Polynomials:
        if len(self.plnm) < len(arg.plnm):
            return Polynomials([0])

        res = Polynomials([0 for i in range(len(self.plnm) - len(arg.plnm) + 1)])
        buf = Polynomials(self.plnm)

        for i in range(len(res.plnm) - 1, -1, -1):
            res.plnm[i] = buf.plnm[-1] / arg.plnm[-1]
            s = Polynomials([0 for j in range(i + 1)])

            s.plnm[-1] = res.plnm[i]
            buf = buf - s * arg
            buf.plnm = buf.plnm[: len(buf.plnm) - 1]

        res.simplify()
        return res

    def prived(self) -> Polynomials:
        return Polynomials([x / self.plnm[-1] for x in self.plnm])

    def __mod__(self, arg: Polynomials) -> Polynomials:
        if len(self.plnm) < len(arg.plnm):
            return self

        res = self - arg * (self // arg)
        res.simplify()
        return res

    def derivative(self) -> Polynomials:
        res = Polynomials([0 for i in range(len(self.plnm) - 1)])

        for i in range(len(self.plnm) - 1):
            res.plnm[i] = self.plnm[i + 1] * (i + 1)

        return res

    def gcd(self, m: Polynomials) -> Polynomials:
        ret = self
        while self.plnm != [0] and m.plnm != [0]:
            if len(self.plnm) >= len(m.plnm):
                ret %= m
                ret.simplify()
            else:
                m %= self
                m.simplify()
        return ret + m

    def find_value(self, point: int) -> int:
        result = 0
        for i in range(len(self.plnm)):
            result += self.plnm[i] * (point**i)

        return result

    def standard(self) -> Polynomials:
        return Polynomials([self.plnm[i] for i in range(len(self.plnm) - 1, -1, -1)])

    def upper_border(self) -> int:
        self.prived()
        self.standard()
        m, b = 0, 0
        for i in range(len(self.plnm)):
            if self.plnm[i] < 0:
                m += i
            else:
                pass
        if m == 0:
            return 0

        minuses = []
        for i in range(len(self.plnm)):
            if self.plnm[i] < 0:
                minuses.append(self.plnm[i])
            else:
                pass
        b += abs(min(minuses))
        return 1 + math.pow(b, (1 / m))

    def lower_border(self) -> int:
        for i in range(len(0, self.plnm, 2)):
            self.plnm[i + 1] = -self.plnm[i + 1]
        return self.upper_border()

    def __str__(self) -> str:
        res = ""
        for i in range(len(self.plnm) - 1, 1, -1):
            res += str(self.plnm[i]) + "x^" + str(i) + " + "
        res += str(self.plnm[1]) + "x" + " + " + str(self.plnm[0])
        return res


def find_sturm(polynom: list) -> list:
    sturm_sys = []

    sturm_sys.append(polynom)
    sturm_sys.append(polynom.derivative())

    while True:
        k = -1 * (sturm_sys[-2] % sturm_sys[-1])

        sturm_sys.append(k)

        if len(k.coefs) == 1:
            break

    return sturm_sys


def root_count(sturm_sys: list, left: int, right: int) -> int:
    sign_left = []
    sign_right = []
    change_left = 0
    change_right = 0

    for i in sturm_sys:
        val_left = i.find_value(left)
        val_right = i.find_value(right)

        if val_left > 0:
            sign_left.append(1)
        elif val_left < 0:
            sign_left.append(-1)

        if val_right > 0:
            sign_right.append(1)
        elif val_right < 0:
            sign_right.append(-1)

    for i in range(1, len(sign_left)):
        if sign_left[i - 1] != sign_left[i]:
            change_left += 1

    for i in range(1, len(sign_right)):
        if sign_right[i - 1] != sign_right[i]:
            change_right += 1

    return change_left - change_right
