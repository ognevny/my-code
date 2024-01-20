// search sort

#include <ctime>
#include <iostream>
#include <random>
#include <vector>
using namespace std;

int main() {
  int n;
  cin >> n;
  clock_t st = clock();
  vector<int> a(n);
  srand(time(0));
  for (int i = 0; i < n; i++)
    a[i] = rand();

  for (int i = 0; i < n - 1; i++) {
    int i_min = i;
    for (int j = i + 1; j < n; j++) {
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
  cout << static_cast<double>(end - st) / CLOCKS_PER_SEC;
  return 0;
}
