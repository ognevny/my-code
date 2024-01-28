// task: print last word of input

#include <cstdint>
#include <iostream>
#include <stddef.h>
#include <string>

int main() {
  std::string line;
  std::getline(std::cin, line);
  size_t end = line.find_last_not_of(" ");
  std::string trimmed = line.substr(0, end + 1);
  std::string last(trimmed.begin() + static_cast<int64_t>(trimmed.rfind(" ")) +
                       1,
                   trimmed.end());
  std::cout << last;
  return 0;
}
