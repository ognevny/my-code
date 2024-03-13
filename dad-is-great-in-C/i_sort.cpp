// insertion sort

#include <algorithm>
#include <chrono>
#include <iostream>
#include <iterator>
#include <random>
#include <ratio>
#include <utility>
#include <vector>

int main() {
  unsigned long long n;
  std::cin >> n;
  auto st = std::chrono::system_clock::now();
  std::vector<unsigned int> a(n);

  std::random_device rd;
  std::mt19937 gen(rd());
  std::uniform_int_distribution<std::mt19937::result_type> dist(1, 1000000);
  for (unsigned long long i = 0; i < n; i++)
    a[i] = dist(gen);

  for (unsigned long long i = 1; i < n; i++) {
    unsigned int key = a[i];
    unsigned long long j = i;
    unsigned long long pos = static_cast<unsigned long long>(std::distance(
        a.begin(), std::lower_bound(
                       a.begin(), a.begin() + static_cast<long long>(i), key)));
    while (j > pos) {
      std::swap(a[j - 1], a[j]);
      j--;
    }
  }

  // for (unsigned int i : a)
  //   std::cout << i << " ";

  auto end = std::chrono::system_clock::now();
  auto elapsed =
      std::chrono::duration_cast<std::chrono::microseconds>(end - st) /
      1000000.0;
  std::cout << std::fixed << elapsed.count();
  return 0;
}
