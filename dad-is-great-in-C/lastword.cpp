// task: print last word of input

#include <iostream>
#include <string>

int main() {
  std::string line;
  std::getline(std::cin, line);
  auto end = line.find_last_not_of(" ");
  auto trimmed = line.substr(0, end + 1);
  std::string last(trimmed.begin() +
                       static_cast<long long>(trimmed.rfind(" ")) + 1,
                   trimmed.end());
  std::cout << last;
  return 0;
}
