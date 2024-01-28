/* task: generate all string combinations using inputed (?) alphabet with length
k */

#include <iostream>
#include <string>
#include <vector>

int main() {
  std::string alpha;
  std::cin >> alpha;
  int k;
  std::cin >> k;
  int count = 0;
  std::vector<std::string> a;
  std::vector<std::string> b;
  for (char c : alpha) {
    std::string buf(1, c);
    a.push_back(buf);
    b.push_back(buf);
  }
  for (int i = 1; i < k; i++) {
    std::vector<std::string> words;
    for (std::string first : a) {
      for (std::string second : b) {
        words.push_back(first + second);
      }
    }
    a = words;
  }
  for (std::string word : a) {
    // if (word[1] == alpha[0]) { // for another task
    std::cout << word << std::endl;
    count++;
  } //}
  std::cout << count;
  return 0;
}
