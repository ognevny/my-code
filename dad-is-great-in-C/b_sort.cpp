// bubble sort in C++

#include <iostream>
#include <random>
#include <ctime>
using namespace std;

int main() {
    int n;
    cin >> n;
    clock_t st = clock();
    int a[n];
    srand(time(NULL));
    for (int i = 0; i < n; i++) a[i] = rand();

    for (int i = n - 1; i > 0; i--) {
        int count = 0;
        for (int j = 0; j < i; j++) {
            if (a[j] > a[j + 1]) {
                // swap(a[j], a[j + 1]); // alt variant
                a[j] = 2 * a[j + 1] - a[j];
                a[j + 1] = 2 * a[j + 1] - a[j];
                a[j] = (a[j] + a[j + 1]) / 2;
                count++; } }
        if (count == 0) break; }

    //for (int i : a) cout << i << " ";
    clock_t end = clock();
    cout << endl << (double)(end - st) / CLOCKS_PER_SEC;
    return 0; }
