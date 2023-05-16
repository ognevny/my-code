// a list of useful functions written by me

#include "rustificate.h"
#include <string>
use namespace std;

u16 numsum(u64 n) { // sum of number digits
  u16 res = 0;
  while (n) {
    res += n % 10;
    n /= 10;
  }
  return res;
}

u64 rev(u64 n) { // reverse number
  u64 rev = 0;
  while (n) {
    rev = rev * 10 + (n % 10);
    n /= 10;
  }
  return rev;
}

u64 numsys(u64 n, u8 k) {
  u64 ns = 1;
  while (n) {
    ns = ns * 10 + (n % k);
    n /= k;
  }
  return (rev(ns) - 1) / 10;
}

u16 numcount(u64 n) { // counts number digits
  u16 count = 0;
  while (n) {
    n /= 10;
    count++;
  }
  return count;
}

String numsys16(u64 n, u8 k) {
  const String NUMS[16] = {"0", "1", "2", "3", "4", "5", "6", "7",
                           "8", "9", "A", "B", "C", "D", "E", "F"};
  String res = "";
  while (n) {
    res += NUMS[n % k];
    n /= k;
  }
  String rev(res.rbegin(), res.rend());
  return rev;
}

f64 binpow(f64 b, u16 e) { // binary power
  f64 v = 1.0;
  while (e != 0) {
    if ((e & 1) != 0)
      v *= b;
    b *= b;
    e >>= 1;
  }
  return v;
}
