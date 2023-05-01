// a list of useful functions written by me

#include <string>
using namespace std;

uint16_t numsum(uint64_t n) { // sum of number digits
  uint16_t res = 0;
  while (n) {
    res += n % 10;
    n /= 10;
  }
  return res;
}

uint64_t rev(uint64_t n) { // reverse number
  uint64_t rev = 0;
  while (n) {
    rev = rev * 10 + (n % 10);
    n /= 10;
  }
  return rev;
}

uint64_t numsys(uint64_t n, uint8_t k) {
  uint64_t ns = 1;
  while (n) {
    ns = ns * 10 + (n % k);
    n /= k;
  }
  return (rev(ns) - 1) / 10;
}

uint16_t numcount(uint64_t n) { // counts number digits
  uint16_t count = 0;
  while (n) {
    n /= 10;
    count++;
  }
  return count;
}

string numsys16(uint64_t n, uint8_t k) {
  const string nums[16] = {"0", "1", "2", "3", "4", "5", "6", "7",
                           "8", "9", "A", "B", "C", "D", "E", "F"};
  string res = "";
  while (n) {
    res += nums[n % k];
    n /= k;
  }
  string rev(res.rbegin(), res.rend());
  return rev;
}

long double binpow(double b, uint16_t e) { // binary power
  long double v = 1.0;
  while (e != 0) {
    if ((e & 1) != 0)
      v *= b;
    b *= b;
    e >>= 1;
  }
  return v;
}
