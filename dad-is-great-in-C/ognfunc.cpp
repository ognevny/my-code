// a list of useful functions written by me

#include "ognfunc.hpp"
#include <iterator>
#include <string>

unsigned long long numsum(unsigned long long n) { // sum of number digits
  auto res = 0ULL;
  while (n) {
    res += n % 10;
    n /= 10;
  }
  return res;
}

unsigned long long reverse(unsigned long long n) { // reverse number
  auto rev = 0ULL;
  while (n) {
    rev = rev * 10 + (n % 10);
    n /= 10;
  }
  return rev;
}

unsigned long long numsys(unsigned long long n, unsigned char k) {
  auto ns = 1ULL;
  while (n) {
    ns = ns * 10 + (n % k);
    n /= k;
  }
  return (reverse(ns) - 1) / 10;
}

unsigned short numcount(unsigned long long n) { // counts number digits
  unsigned short count = 0;
  while (n) {
    n /= 10;
    count++;
  }
  return count;
}

std::string numsys16(unsigned long long n, unsigned char k) {
  const char NUMS[16] = {'0', '1', '2', '3', '4', '5', '6', '7',
                         '8', '9', 'A', 'B', 'C', 'D', 'E', 'F'};
  std::string res = "";
  while (n) {
    res += NUMS[n % k];
    n /= k;
  }
  std::string rev(res.rbegin(), res.rend());
  return rev;
}

long double binpow(long double b, unsigned short e) { // binary power
  auto v = 1.0L;
  while (e != 0) {
    if ((e & 1) != 0)
      v *= b;
    b *= b;
    e >>= 1;
  }
  return v;
}
