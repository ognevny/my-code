#include <iostream>
#include <string>
#include <vector>
using namespace std;

int main() {
    string line;
    getline(cin, line);
    size_t end = line.find_last_not_of(" ");
    string trimmed = line.substr(0, end + 1);
    vector<string> slices;
    while (!trimmed.empty()) {
        size_t slice = trimmed.find_first_of(" ");
        slices.push_back(trimmed.substr(0, slice));
        if (slice == string::npos) break;
        trimmed = trimmed.substr(slice + 1); }
    int max = 0;
    string maxstr;
    for (int i = 0; i < slices.size(); i++) {
        if (slices[i].size() > max) { maxstr = slices[i]; max = slices[i].size(); } }
    cout << maxstr << endl << max;
    return 0; }
