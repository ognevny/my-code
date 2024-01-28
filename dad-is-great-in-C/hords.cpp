/* solving equations with "hord" method
c = a - f(a)(a - b)/(f(a) - f(b)) */

#include <cmath>
#include <cstdlib>
#include <iomanip>
#include <iostream>

int main() {
  long double C;
  std::cin >> C;
  long double a = 0.0L, b = C;
  long double c = a - (a * a + sqrtl(a) - C) * (a - b) /
                          ((a * a + sqrtl(a) - C) - (b * b + sqrtl(b) - C));
  while (abs(c * c + sqrtl(c) - C) > 0.000001L) {
    if ((a * a + sqrtl(a) - C) * (c * c + sqrtl(c) - C) < 0)
      b = c;
    else
      a = c;
    c = a - (a * a + sqrtl(a) - C) * (a - b) /
                ((a * a + sqrtl(a) - C) - (b * b + sqrtl(b) - C));
  }
  std::cout << std::setprecision(9) << c;
  return 0;
}
