// a programm to test speed of each language (C implementation)

#include <stdio.h>

unsigned int s() {
  unsigned n = 1;
  while (n < 1000000000)
    n++;
  return n;
}

int main() {
  printf("%d", s());
  return 0;
}
