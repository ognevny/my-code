#include <iostream>
using namespace std;

void rev(int n) {
    if (n == 0) return;
    int a;
    cin >> a;
    rev(n - 1);
    cout << a << " "; }

int main() {
    int n;
    cin >> n;
    rev(n);
    return 0; }