// search sort

#include <chrono>
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

  for (size_t i = 0; i < n - 1; i++) {
    size_t i_min = i;
    for (size_t j = i + 1; j < n; j++) {
      if (a[j] < a[i_min])
        i_min = j;
    }
    if (i_min != i) {
      a[i] = 2 * a[i_min] - a[i];
      a[i_min] = 2 * a[i_min] - a[i];
      a[i] = (a[i] + a[i_min]) / 2;
    }
  }

  // for (int i : a) cout << i << " ";
  clock_t end = clock();
  std::cout << static_cast<double>(end - st) / CLOCKS_PER_SEC;
  return 0;
}
