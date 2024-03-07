// insertion sort

#include <cstdint>
#include <ctime>
#include <iostream>
#include <random>
#include <vector>

int main() {
  size_t n;
  std::cin >> n;
  clock_t st = clock();
  std::vector<unsigned int> a(n);

  std::random_device rd;
  std::mt19937 gen(rd());
  std::uniform_int_distribution<std::mt19937::result_type> dist(1, 1000000);
  for (size_t i = 0; i < n; i++)
    a[i] = dist(gen);

  for (size_t i = 0; i < n; i++) {
    int64_t j = i - 1;
    while (a[j] > a[i] and j >= 0) {
      a[j + 1] = a[j];
      j--;
    }
    a[j + 1] = a[i];
  }

  for (int i : a)
    std::cout << i << " ";

  clock_t end = clock();
  std::cout << static_cast<double>(end - st) / CLOCKS_PER_SEC;
  return 0;
}
