# merge sort (using mlp)

import copy
import multiprocessing as mlp
from random import randint
from time import perf_counter


def merge(left, right):
    res = []
    i = 0
    j = 0

    while i < len(left) and j < len(right):
        if left[i] < right[j]:
            res.append(left[i])
            i += 1
        else:
            res.append(right[j])
            j += 1

    res += left[i:]
    res += right[j:]

    return res


def merge_sort(arr):
    if len(arr) == 1:
        return arr
    elif len(arr) > 1:
        mid = len(arr) // 2
        Left = arr[:mid]
        Right = arr[mid:]
        Left = merge_sort(Left)
        Right = merge_sort(Right)

        return merge(Left, Right)


def generation(length):
    return [randint(10, 10000) for _ in range(length)]


if __name__ == "__main__":
    data1 = map(generation, [randint(100, 100000) for _ in range(100)])
    data2 = copy.deepcopy(data1)
    start = perf_counter()
    with mlp.Pool(processes=mlp.cpu_count()) as pool:
        sorted_data = pool.map(merge_sort, data1)
    end = perf_counter()

    print("parallel", end - start)

    start = perf_counter()

    sorted_data2 = list(map(merge_sort, data2))

    end = perf_counter()

    print("not parallel", end - start)
