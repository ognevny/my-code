// a test to check that ognfunc works fine

#include "ognfunc.hpp"
#include <iostream>

int main() {
  uint64_t a = 12345;
  std::cout << rev(a) << std::endl;
  std::cout << numcount(a) << std::endl;
  std::cout << numsum(a) << std::endl;
  std::cout << numsys(a, 8) << std::endl;
  std::cout << numsys16(a, 16) << std::endl;
  std::cout << binpow(static_cast<double>(a), 2) << std::endl;
}
