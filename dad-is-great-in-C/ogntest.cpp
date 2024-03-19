// a test to check that ognfunc works fine

#include "ognfunc.hpp"
#include <iostream>
#include <cassert>
#include <string>

int main() {
  auto a = 12345ULL;
  assert(reverse(a) == 54321);
  assert(numcount(a) == 5);
  assert(numsum(a) == 15);
  assert(numsys(a, 8) == 30071);
  assert(numsys16(a, 16) == "3039");
  assert(binpow(static_cast<double>(a), 2) == 152399025.0);
  return 0;
}
