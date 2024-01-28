// bubble sort

#include <chrono>
#include <cstdlib>
#include <ctime>
#include <iostream>
#include <utility>
#include <vector>

static void bubble(std::vector<int> arr, size_t n) {
  for (size_t i = n - 1; i > 0; i--) {
    int count = 0;
    for (size_t j = 0; j < i; j++) {
      if (arr[j] > arr[j + 1]) {
        std::swap(arr[j], arr[j + 1]);
        count++;
      }
    }
    if (count == 0)
      break;
  }
}

int main() {
  size_t n;
  std::cin >> n;
  clock_t st = clock();
  std::vector<int> a(n);
  srand(static_cast<unsigned int>(
      std::chrono::system_clock::now().time_since_epoch().count()));
  for (size_t i = 0; i < n; i++) {
    a[i] = rand();
    std::cout << a[i] << " ";
  }
  std::cout << std::endl;
  bubble(a, n);
  // for (int i : a)
  // cout << i << " ";
  clock_t end = clock();
  std::cout << std::endl << static_cast<double>(end - st) / CLOCKS_PER_SEC;
  return 0;
}
