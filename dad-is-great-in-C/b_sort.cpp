// bubble sort

#include <chrono>
#include <iostream>
#include <random>
#include <utility>
#include <vector>

static void bubble(std::vector<unsigned long int> arr, unsigned long long n) {
  for (auto i = n - 1; i > 0; i--) {
    int count = 0;
    for (auto j = 0ULL; j < i; j++) {
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
  unsigned long long n;
  std::cin >> n;
  auto st = std::chrono::system_clock::now();
  std::vector<unsigned long int> a(n);

  std::random_device rd;
  std::mt19937 gen(rd());
  std::uniform_int_distribution<std::mt19937::result_type> dist(1, 1000000);

  for (auto i = 0ULL; i < n; i++)
    a[i] = dist(gen);

  bubble(a, n);

  // for (int i : a)
  //   std::cout << i << " ";

  auto end = std::chrono::system_clock::now();
  auto elapsed =
      std::chrono::duration_cast<std::chrono::microseconds>(end - st) /
      1000000.0;
  std::cout << std::fixed << elapsed.count();
  return 0;
}
