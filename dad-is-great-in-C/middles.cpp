// something like binary search for solving equations

#include <cmath>
#include <cstdlib>
#include <iomanip>
#include <iostream>

static long double f(long double n) { return n * n + sqrtl(n); }

int main() {
  long double epsilon = 0.0000000001L; // could be in input
  long double C;
  std::cin >> C;
  long double a = 0.0L, b = C;
  long double c = (a + b) / 2;
  while (abs(f(c) - C) > epsilon) {
    if ((f(a) - C) * (f(c) - C) < 0)
      b = c;
    else
      a = c;
    c = (a + b) / 2;
  }
  std::cout << std::setprecision(9) << c;
  return 0;
}
