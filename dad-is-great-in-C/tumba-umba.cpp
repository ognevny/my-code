/* task: generate all string combinations using inputed (?) alphabet with length
k */

#include <iostream>
#include <string>
#include <vector>
using namespace std;

int main() {
  string alpha;
  cin >> alpha;
  int k;
  cin >> k;
  int count = 0;
  vector<string> a;
  vector<string> b;
  for (char c : alpha) {
    string buf(1, c);
    a.push_back(buf);
    b.push_back(buf);
  }
  for (int i = 1; i < k; i++) {
    vector<string> words;
    for (string first : a) {
      for (string second : b) {
        words.push_back(first + second);
      }
    }
    a = words;
  }
  for (string word : a) {
    // if (word[1] == alpha[0]) { // just for another task
    cout << word << endl;
    count++;
  } //}
  cout << count;
  return 0;
}