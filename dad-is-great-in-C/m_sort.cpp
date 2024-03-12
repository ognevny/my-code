// merge sort

#include <chrono>
#include <iostream>
#include <random>
#include <ratio>
#include <stddef.h>
#include <vector>

static void merge(std::vector<unsigned int> arr, size_t left, size_t mid,
                  size_t right) {
  size_t rt = mid - left + 1, lt = right - mid;
  int *left_arr = new int[rt],
      *right_arr = new int[lt]; // создаём два новых массива

  for (size_t i = 0; i < rt; i++)
    left_arr[i] = arr[left + i]; // заполняем их
  for (size_t i = 0; i < lt; i++)
    right_arr[i] = arr[mid + 1 + i];

  size_t i_right = 0, i_left = 0,
         i_merged = left; // индексы, по которым мы идём вдоль массивов

  while (i_right < rt and i_left < lt) { // слияние
    if (left_arr[i_right] <= right_arr[i_left]) {
      arr[i_merged] = left_arr[i_right];
      i_right++;
    } else {
      arr[i_merged] = right_arr[i_left];
      i_left++;
    }
    i_merged++;
  }

  while (i_right < rt) { // дополняем из левого массива
    arr[i_merged] = left_arr[i_right];
    i_right++;
    i_merged++;
  }

  while (i_left < lt) { // дополняем из правого
    arr[i_merged] = right_arr[i_left];
    i_left++;
    i_merged++;
  }

  delete[] left_arr;
  delete[] right_arr;
}

static void merge_sort(std::vector<unsigned int> arr, size_t begin,
                       size_t end) {
  if (begin >= end)
    return; // раздел до единичных

  size_t mid = (end + begin) / 2;
  merge_sort(arr, begin, mid);
  merge_sort(arr, mid + 1, end);
  merge(arr, begin, mid, end);
}

int main() {
  size_t n;
  std::cin >> n;
  auto st = std::chrono::system_clock::now();

  std::vector<unsigned int> a(n);
  std::random_device rd;
  std::mt19937 gen(rd());
  std::uniform_int_distribution<std::mt19937::result_type> dist(1, 1000000);

  for (size_t i = 0; i < n; i++)
    a[i] = dist(gen);
  merge_sort(a, 0, n - 1);

  // for (int i : a)
  //   std::cout << i << " ";

  auto end = std::chrono::system_clock::now();
  auto elapsed =
      std::chrono::duration_cast<std::chrono::microseconds>(end - st) /
      1000000.0;
  std::cout << std::fixed << elapsed.count();
  return 0;
}
