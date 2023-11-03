from math import atan, cos, pi, sin, sqrt


class C_numb:
    def __init__(self, real=1, im=0):
        self.real = real
        self.im = im
        self.mod = sqrt(real**2 + im**2)

        if real > 0 and im >= 0:
            self.arg = atan(im / real)

        elif real > 0 and im < 0:
            self.arg = atan(im / real) + 2 * pi

        elif real < 0 and im >= 0:
            self.arg = atan(im / real) + pi

        elif real < 0 and im < 0:
            self.arg = atan(im / real) + pi

        elif real == 0:
            if im < 0:
                self.arg = 2 * pi - 0, 5 * pi

            elif im > 0:
                self.arg = pi * 0, 5

    def __add__(self, a):
        if isinstance(a, C_numb):
            return C_numb(self.real + a.real, self.im + a.im)
        else:
            return C_numb(self.real + a, self.im)

    def __radd__(self, a):
        if isinstance(a, C_numb):
            return C_numb(self.real + a.real, self.im + a.im)
        else:
            return C_numb(self.real + a, self.im)

    def __mul__(self, a):
        if isinstance(a, C_numb):
            return C_numb(
                self.real * a.real - self.im * a.im, self.real * a.im + self.im * a.real
            )
        else:
            return C_numb(self.real * a, self.im * a)

    def __rmul__(self, a):
        if isinstance(a, C_numb):
            return C_numb(
                self.real * a.real - self.im * a.im, self.real * a.im + self.im * a.real
            )
        else:
            return C_numb(self.real * a, self.im * a)

    def con(self):
        return C_numb(self.real, -1 * self.im)

    def __truediv__(self, a):
        if isinstance(a, C_numb):
            num = self * a.con()
            denom = (a * a.con()).real
            result = num * (denom ** (-1))
            return result
        else:
            return self * (a ** (-1))

    def __sub__(self, a):
        if isinstance(a, C_numb):
            return C_numb(self.real - a.real, self.im - a.im)
        else:
            return C_numb(self.real - a, self.im)

    def __rsub__(self, a):
        if isinstance(a, C_numb):
            return C_numb(self.real - a.real, self.im - a.im)
        else:
            return C_numb(self.real - a, self.im)

    def __pow__(self, a):
        return C_numb(
            (self.mod**a) * (cos(atan(a * self.arg))),
            (self.mod**a) * (sin(atan(a * self.arg))),
        )

    def rad(self, a):
        resultc = []
        for j in range(a):
            resultc += [
                str((self.mod ** (1 / a)) * (cos(atan((self.arg + 2 * pi * j) / a))))  # type: ignore
                + "+"
                + str((self.mod ** (1 / a)) * (sin(atan((self.arg + 2 * pi * j) / a))))  # type: ignore
                + "i"
            ]
        return resultc

    def __str__(self):
        return str(self.real) + "+" + str(self.im) + "i"


n = C_numb(1, 2)
m = C_numb(1, 1)
print(n + m)
print(n * m)
print(n - m)
print(m**2)
print(m.rad(2))
