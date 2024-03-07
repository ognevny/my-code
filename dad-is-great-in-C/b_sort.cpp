// bubble sort

#include <ctime>
#include <iostream>
#include <random>
#include <utility>
#include <vector>

static void bubble(std::vector<unsigned int> arr, size_t n) {
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
  std::vector<unsigned int> a(n);

  std::random_device rd;
  std::mt19937 gen(rd());
  std::uniform_int_distribution<std::mt19937::result_type> dist(1, 1000000);

  for (size_t i = 0; i < n; i++)
    a[i] = dist(gen);

  bubble(a, n);
  // for (int i : a)
  //   std::cout << i << " ";
  clock_t end = clock();
  std::cout << std::endl << static_cast<double>(end - st) / CLOCKS_PER_SEC;
  return 0;
}
