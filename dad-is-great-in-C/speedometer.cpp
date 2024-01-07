// a programm to test speed of each language (C++ implementation)

#include <iostream>

static inline unsigned int s() {
  unsigned int n = 1;
  while (n < 1000000000)
    n++;
  return n;
}

int main() {
  std::cout << s();
  return 0;
}
