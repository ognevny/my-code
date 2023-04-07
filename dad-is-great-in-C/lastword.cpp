// task: print last word of input

#include <iostream>
#include <string>
using namespace std;

int main() {
    string line;
    getline(cin, line);
    size_t end = line.find_last_not_of(" ");
    string trimmed = line.substr(0, end + 1);
    string last(trimmed.begin() + trimmed.rfind(" ") + 1, trimmed.end());
    cout << last;
    return 0; }
