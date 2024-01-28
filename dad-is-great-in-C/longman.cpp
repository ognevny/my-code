/* task: print the longest word in input and its length
reference to "Long, long, maaaaaaaan" (Japanese ad) */

#ifdef __clang__
#include <__fwd/sstream.h>
#endif

#include <cstdint>
#include <iostream>
#include <string>
#include <sstream>

int main() {
  std::string line;
  std::getline(std::cin, line);
  std::stringstream ss(line);
  std::string word;
  uint64_t max = 0;
  std::string maxstr;
  while (ss >> word) {
    if (word.size() > max) {
      maxstr = word;
      max = word.size();
    }
  }
  std::cout << maxstr << std::endl << max;
  return 0;
}
