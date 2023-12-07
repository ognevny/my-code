// a programm to test speed of each language (C implementation)

#include <stdio.h>

static inline unsigned int sum() {
  unsigned int n = 1;
  while (n < 1000000000)
    n++;
  return n;
}

int main() {
  printf("%d", sum());
  return 0;
}
