// task: represent N as p1^a1*p2^a2*...*pn^an

#include <iostream>
using namespace std;

int main() {
  int n;
  cin >> n;
  bool printed = false;
  for (int i = 2; i <= n; i++) {
    int count = 0;
    if (n % i == 0) {
      while (n % i == 0) {
        n /= i;
        count++;
      }
      if (printed) {
        if (count == 1)
          cout << "*" << i;
        else
          cout << "*" << i << "^" << count;
      } else {
        if (count == 1)
          cout << i;
        else
          cout << i << "^" << count;
        printed = true;
      }
    }
  }
  return 0;
}
