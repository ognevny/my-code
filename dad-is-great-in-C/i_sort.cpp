// insertion sort

#include <chrono>
#include <cstdint>
#include <iostream>
#include <random>
#include <vector>

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

  for (size_t i = 0; i < n; i++) {
    int64_t j = i - 1;
    while (a[j] > a[i] and j >= 0) {
      a[j + 1] = a[j];
      j--;
    }
    a[j + 1] = a[i];
  }

  // for (int i : a)
  //   std::cout << i << " ";

  auto end = std::chrono::system_clock::now();
  auto elapsed =
      std::chrono::duration_cast<std::chrono::microseconds>(end - st) /
      1000000.0;
  std::cout << std::fixed << elapsed.count();
  return 0;
}
