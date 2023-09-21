// bubble sort

#include <ctime>
#include <iostream>
#include <random>
using namespace std;

void bubble(int *arr, int n) {
  for (int i = n - 1; i > 0; i--) {
    int count = 0;
    for (int j = 0; j < i; j++) {
      if (arr[j] > arr[j + 1]) {
        swap(arr[j], arr[j + 1]);
        count++;
      }
    }
    if (count == 0)
      break;
  }
}

int main() {
  int n;
  cin >> n;
  // clock_t st = clock();
  int a[n];
  srand(time(NULL));
  for (int i = 0; i < n; i++) {
    a[i] = rand();
    cout << a[i] << " ";
  }
  cout << endl;
  bubble(a, n);
  for (int i : a)
    cout << i << " ";
  // clock_t end = clock();
  // cout << endl << (double)(end - st) / CLOCKS_PER_SEC;
  return 0;
}
