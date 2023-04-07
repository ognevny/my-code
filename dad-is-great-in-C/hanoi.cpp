#include <iostream>
using namespace std;

void hanoi(int n, int a, int b, int c) {
    if (n == 0) return;
    hanoi(n - 1, a, c, b);
    cout << n << " " << a << " " << b << endl;
    hanoi(n - 1, c, b, a); }

int main() {
    int n, b, c = 5;
    cin >> n;
    if (n % 2) b = 2;
    else b = 3;
    c -= b;
    hanoi(n, 1, b, c);
    for (int i = n; i > 0; i--) {
        hanoi(i - 1, b, c, 1);
        swap(b, c); }
    return 0; }
