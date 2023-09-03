// a programm to test speed of each language (C++ implementation)

#include <iostream>
using namespace std;

const int s() {
  int n = 1;
  while (n < 1'000'000'000)
    n++;
  return n;
}

int main() {
  cout << s();
  return 0;
}
