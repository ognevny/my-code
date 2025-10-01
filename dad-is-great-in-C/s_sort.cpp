// search sort

#include <chrono>
#include <iostream>
#include <random>
#include <vector>

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

  for (auto i = 0ULL; i < n - 1; i++) {
    auto i_min = i;
    for (auto j = i + 1; j < n; j++) {
      if (a[j] < a[i_min])
        i_min = j;
    }
    if (i_min != i) {
      a[i] = 2 * a[i_min] - a[i];
      a[i_min] = 2 * a[i_min] - a[i];
      a[i] = (a[i] + a[i_min]) / 2;
    }
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
