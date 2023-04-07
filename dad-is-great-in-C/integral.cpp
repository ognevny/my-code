#include <iostream>
#include <cmath>
using namespace std;


long double f(long double x) {
    return 10 * cos(x / 2.) * sin(x);  
}


long double rect(long double a, long double b, int n) {
    long double s = 0.0;
    for (int i = 0; i <= n - 1; i++) s += f(a + i * (b - a) / (n - 1)); 
    return s * ((b - a) / n); }


int main() {
    long double a, b; cin >> a >> b;
    long double eps; cin >> eps;
    int n = 1;
    long double s1 = 0, s2 = f(a) * (b - a);
    while (abs(s2 - s1) > eps) {
        n *= 2;
        s1 = s2;
        s2 = rect(a, b, n);
        cout << s1 << " " << s2 << endl; }
    cout << s2;
    return 0; }