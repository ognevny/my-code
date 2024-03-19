from math import acos, sqrt


class Rn:
    def __init__(self, body):
        self.body = body.copy()
        self.dlina = len(body)

    def __add__(self, vec):
        if self.dlina == vec.dlina:
            return Rn(list(map(lambda x, y: x + y, self.body, vec.body)))
        else:
            return "ошибка размерности"

    def __mul__(self, vec):
        if isinstance(vec, Rn):
            if self.dlina == vec.dlina:
                return sum(list(map(lambda x: x[0] * x[1], zip(self.body, vec.body))))
            else:
                return "ошибка размерности"
        else:
            return Rn(list(map(lambda x: x * vec, self.body)))

    def __rmul__(self, vec):
        if isinstance(vec, Rn):
            if self.dlina == vec.dlina:
                return sum(list(map(lambda x: x[0] * x[1], zip(self.body, vec.body))))
            else:
                return "ошибка размерности"
        else:
            return Rn(list(map(lambda x: x * vec, self.body)))

    def __sub__(self, vec):
        return self + vec * (-1)

    def norm(self):
        return sqrt(self * self)

    def angle(self, vec):
        return acos((self * vec) / (self.norm() * vec.norm()))

    def __str__(self):
        return str(self.body)


a = Rn([3, 5, 4])
b = Rn([4, 2, 1])
print(a + b)
print(a * b)
print(a * 2)
print(a - b)
print(a.norm())
print(a.angle(b))
