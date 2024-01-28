// insertion sort

#include <chrono>
#include <cstdint>
#include <cstdlib>
#include <ctime>
#include <iostream>
#include <vector>

int main() {
  size_t n;
  std::cin >> n;
  clock_t st = clock();
  std::vector<int> a(n);
  srand(static_cast<unsigned int>(
      std::chrono::system_clock::now().time_since_epoch().count()));
  for (size_t i = 0; i < n; i++)
    a[i] = rand();

  for (size_t i = 0; i < n; i++) {
    int64_t j = i - 1;
    while (a[j] > a[i] and j >= 0) {
      a[j + 1] = a[j];
      j--;
    }
    a[j + 1] = a[i];
  }

  // for (int i : a) cout << i << " ";
  clock_t end = clock();
  std::cout << static_cast<double>(end - st) / CLOCKS_PER_SEC;
  return 0;
}
