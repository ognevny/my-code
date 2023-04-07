from math import pow


class Polynomials:
    def __init__(self, plnm):
        self.plnm = plnm

    def __add__(self, m):
        if isinstance(m, Polynomials):
            res = Polynomials(
                list(map(lambda x, y: x + y, self.plnm, m.plnm)))  # type: ignore
            res.plnm.extend(self.plnm[len(m.plnm):] if len(
                self.plnm) > len(m.plnm) else m.plnm[len(self.plnm):])
            return res
        else:
            self.plnm[0] += m
            return Polynomials(self.plnm)

    def __mul__(self, arg):
        if isinstance(arg, int) or isinstance(arg, float):
            return Polynomials(list(map(lambda x: x * arg, self.plnm)))

        else:
            res = Polynomials(
                [0 for i in range(len(self.plnm) + len(arg.plnm) - 1)])
            for i in range(len(self.plnm)):
                for j in range(len(arg.plnm)):
                    res.plnm[i + j] += self.plnm[i] * arg.plnm[j]
            return res

    def __rmul__(self, arg):
        if isinstance(arg, int) or isinstance(arg, float):
            return Polynomials(list(map(lambda x: x * arg, self.plnm)))

    def simplify(self):
        while self.plnm[-1] == 0 and len(self.plnm) > 1:
            self.plnm = self.plnm[:-1]

    def __sub__(self, m):
        return self + (-1) * m

    def __floordiv__(self, arg):

        if len(self.plnm) < len(arg.plnm):
            return Polynomials([0])

        res = Polynomials(
            [0 for i in range(len(self.plnm) - len(arg.plnm) + 1)])
        buf = Polynomials(self.plnm)

        for i in range(len(res.plnm) - 1, -1, -1):
            res.plnm[i] = buf.plnm[-1] / arg.plnm[-1]
            s = Polynomials([0 for j in range(i+1)])

            s.plnm[-1] = res.plnm[i]
            buf = buf - s * arg
            buf.plnm = buf.plnm[:len(buf.plnm) - 1]

        res.simplify()
        return res

    def prived(self):
        return Polynomials(list(map(lambda x: x / self.plnm[-1], self.plnm)))

    def __mod__(self, arg):
        if len(self.plnm) < len(arg.plnm):
            return self

        else:
            res = self - arg * (self // arg)
            res.simplify()
            return res

    def derivative(self):
        res = Polynomials([0 for i in range(len(self.plnm) - 1)])

        for i in range(len(self.plnm) - 1):
            res.plnm[i] = self.plnm[i+1] * (i+1)

        return res

    def gcd(self, m):
        while self.plnm != [0] and m.plnm != [0]:
            if len(self.plnm) >= len(m.plnm):
                self %= m
                self.simplify()
            else:
                m %= self
                m.simplify()
        return self + m

    def FindValue(self, point):
        result = 0
        for i in range(len(self.plnm)):
            result += self.plnm[i] * (point**i)

        return result

    def standard(self):
        standard_plnm = []
        for i in range(len(self.plnm) - 1, -1, -1):
            standard_plnm.append(self.plnm[i])
        return Polynomials(standard_plnm)

    def UpperBorder(self):
        self.prived()
        self.standard()
        m, B = 0, 0
        for i in range(len(self.plnm)):
            if self.plnm[i] < 0:
                m += i
            else:
                pass
        if m == 0:
            return 0
        else:
            minuses = []
            for i in range(len(self.plnm)):
                if self.plnm[i] < 0:
                    minuses.append(self.plnm[i])
                else:
                    pass
            B += abs(min(minuses))
            return 1 + pow(B, (1 / m))

    def LowerBorder(self):
        for i in range(len(0, self.plnm, 2)):  # type: ignore
            self.plnm[i + 1] = -self.plnm[i + 1]
        return self.UpperBorder()

    def __str__(self):
        res = ''
        for i in range(len(self.plnm) - 1, 1, -1):
            res += str(self.plnm[i]) + 'x^' + str(i) + ' + '
        res += str(self.plnm[1]) + 'x' + ' + ' + str(self.plnm[0])
        return res


def FindSturm(polynom):
    SturmSys = []

    SturmSys.append(polynom)
    SturmSys.append(polynom.derivative())

    while True:

        k = -1 * (SturmSys[-2] % SturmSys[-1])

        SturmSys.append(k)

        if len(k.coefs) == 1:
            break

    return SturmSys


def RootCount(SturmSys, left, right):
    SignLeft = []
    SignRight = []
    ChangeLeft = 0
    ChangeRight = 0

    for i in SturmSys:
        ValLeft = i.FindValue(left)
        ValRight = i.FindValue(right)

        if ValLeft > 0:
            SignLeft.append(1)
        elif ValLeft < 0:
            SignLeft.append(-1)

        if ValRight > 0:
            SignRight.append(1)
        elif ValRight < 0:
            SignRight.append(-1)

    for i in range(1, len(SignLeft)):
        if SignLeft[i - 1] != SignLeft[i]:
            ChangeLeft += 1

    for i in range(1, len(SignRight)):
        if SignRight[i - 1] != SignRight[i]:
            ChangeRight += 1

    return ChangeLeft - ChangeRight
