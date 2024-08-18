from __future__ import annotations

import functools
import secrets
from math import cos, pi, sqrt

import matplotlib.pyplot as plt


class Matrix:
    def __init__(self, rows: int = 0, cols: int = 0, matr: list | None = None) -> None:
        self.rows = rows
        self.cols = cols
        if matr is not None:
            self.matr = matr
        else:
            self.matr = []

    def __add__(self, m: Matrix) -> Matrix | None:
        if self.rows != m.rows or self.cols != m.cols:
            return None

        res = Matrix(self.rows, self.cols)
        for i in range(self.rows):
            res.matr.append(list(map(lambda x, y: x + y, self.matr[i], m.matr[i])))

        return res

    def __radd__(self, m: Matrix) -> Matrix | None:
        if self.rows != m.rows or self.cols != m.cols:
            return None

        res = Matrix(self.rows, self.cols)
        for i in range(self.rows):
            res.matr.append(list(map(lambda x, y: x + y, self.matr[i], m.matr[i])))

        return res

    def __mul__(self, m: int | Matrix) -> Matrix | None:
        if isinstance(m, int):
            res = Matrix(self.rows, self.cols)
            for i in range(self.rows):
                res.matr.append([m * x for x in self.matr[i]])
            return res

        if self.cols != m.rows:
            return None

        res = Matrix(self.rows, m.cols)
        for i in range(self.rows):
            tmp = []
            for j in range(m.cols):
                c = 0
                for k in range(self.cols):
                    c += self.matr[i][k] * m.matr[k][j]
                tmp.append(c)
            res.matr.append(tmp)
        return res

    def __rmul__(self, m: int | Matrix) -> Matrix | None:
        if isinstance(m, int):
            res = Matrix(self.rows, self.cols)
            for i in range(self.rows):
                res.matr.append([m * x for x in self.matr[i]])
            return res

        if self.cols != m.rows:
            return None

        res = Matrix(self.rows, m.cols)
        for i in range(self.rows):
            tmp = []
            for j in range(m.cols):
                c = 0
                for k in range(self.cols):
                    c += self.matr[i][k] * m.matr[k][j]
                tmp.append(c)
            res.matr.append(tmp)
        return res

    @staticmethod
    def findpivot(matr: list, r: int, c: int) -> tuple:  # нахождение "опорного элемента"
        for i in range(c, len(matr[0])):
            for j in range(r, len(matr)):
                if matr[j][i] != 0:
                    return j, i
        return None

    def kill(
        self,
        r: int,
        c: int,
        rr: int,
    ) -> None:  # r,c - опорный элемент,rr - строка, с которой работаем
        if rr != r:
            lol = -1 * self.matr[rr][c] / self.matr[r][c] + 1
            self.matr[rr] = list(map(lambda x, y: x + lol * y, self.matr[rr], self.matr[r]))

        for i in range(rr + 1, self.rows):
            lol = -1 * self.matr[i][c] / self.matr[rr][c]
            self.matr[i] = list(map(lambda x, y: x + lol * y, self.matr[i], self.matr[rr]))

    def triangular(self) -> None:
        count = 0

        for i in range(self.rows - 1):
            m = Matrix.findpivot(self.matr, i, count)

            # нашли опорный, теперь приводим i строку в порядок

            if m is None:
                return

            count = m[1]  # это столбец, с которого начнем следующий поиск опорного
            self.kill(m[0], m[1], i)

    def linsolve(self) -> list | None:
        flag_neopr = 0
        flag_nes = 0

        if self.rows < self.cols - 1:
            flag_neopr = 1

        self.triangular()
        print("neopr", flag_neopr)
        for i in range(self.rows - 1, -1, -1):
            if self.matr[i][i] == 0:
                flag_neopr = 1
                break

            if (functools.reduce(lambda y, x: y + x * x, self.matr[i])) == (
                self.matr[i][-1]
            ) ** 2 and (self.matr[i][-1]) != 0:
                flag_nes = 1
                break

        print(flag_nes, flag_neopr)
        if flag_nes == 0 and flag_neopr == 0:
            s = [0 for i in range(self.cols - 1)]  # изначально список нулей [x1,x2,...,xn]

            for i in range(self.cols - 2, -1, -1):
                s[i] = (
                    sum(map(lambda x, y: x * (-y), s, self.matr[i]))
                    + self.matr[i][-1] / self.matr[i][i]
                )

            return s

        return None

    def determinant(self) -> int | None:
        if self.rows != self.cols:
            return None
        self.triangular()
        det = 1

        for i in range(self.cols):
            det *= self.matr[i][i]
        return det

    def transposition(self) -> None:
        self.matr = [self.matr[j][i] for i, j in (range(self.cols), range(self.rows))]
        self.rows, self.cols = self.cols, self.rows

    def transposition_new(self) -> Matrix:
        self.matr = [self.matr[i][j] for i, j in (range(self.cols), range(self.rows))]
        self.rows, self.cols = self.cols, self.rows
        return self

    def __str__(self) -> str:
        res = ""
        for i in range(self.rows):
            res += str(self.matr[i]) + "\n"
        return res


# пример с построением точек, симметричных данным относительно заданной оси
matrix_array = []  # массив радиус-векторов точек

# генерация наших точек, для удобства ограничиваем область, в которой можно получить эти точки
for i in range(10):
    x = secrets.randbelow(0, 30) / 10
    y = secrets.randbelow(0, 30) / 10

    while True:
        if x / sqrt(x**2 + y**2) < cos(pi / 4):
            x = secrets.randbelow(0, 30) / 10
            y = secrets.randbelow(0, 30) / 10
        else:
            break
    matrix_array.append(Matrix(2, 1, [[x], [y]]))
    plt.scatter(matrix_array[i].matr[0], matrix_array[i].matr[1], c="red")


matrix_array2 = []  # массив радиус-векторов уже тех точек, которые мы найдем

reflection = Matrix(
    2,
    2,
    [[0, 1], [1, 0]],
)  # задаем матрицу отражения относительно нашей оси (ось под наклоном 45 градусов к оси Ox)

# получение нужных точек и их отображение на графике
for i in range(10):
    res = reflection * matrix_array[i]
    matrix_array2.append(res)
    plt.scatter(matrix_array2[i].matr[0], matrix_array2[i].matr[1], c="green")


plt.xlim([-0.5, 3])
plt.ylim([-0.5, 3])
plt.plot((0, 3), (0, 3))
plt.grid()


plt.show()
