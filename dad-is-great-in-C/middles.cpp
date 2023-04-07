#include <iostream>
#include <cmath>
#include <iomanip>
using namespace std;


long double f(long double n) {
    return n * n + sqrtl(n); }


int main() {
    long double epsilon = 0.0000000001;
    long double C; cin >> C;
    long double a = 0.0, b = C;
    long double c = (a + b) / 2;
    while (abs(f(c) - C) > epsilon) {
        if ((f(a) - C) * (f(c) - C) < 0) b = c;
        else a = c;
        c = (a + b) / 2; }
    cout << setprecision(9) << c;
    return 0; }