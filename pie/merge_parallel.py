# merge sort (using mlp)

from __future__ import annotations

import copy
import multiprocessing as mlp
from secrets import randbelow
from time import perf_counter


def merge(left: list[int], right: list[int]) -> list[int]:
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


def merge_sort(arr: list[int]) -> list[int]:
    if len(arr) <= 1:
        return arr

    mid = len(arr) // 2
    left = arr[:mid]
    right = arr[mid:]
    left = merge_sort(left)
    right = merge_sort(right)

    return merge(left, right)


def generation(length: int) -> list[int]:
    return [randbelow(10000) for _ in range(length)]


if __name__ == "__main__":
    data1 = map(generation, [randbelow(100000) for _ in range(100)])
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
