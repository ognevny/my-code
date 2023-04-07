/* task: print the longest word in input and its length
reference to "Long, long, maaaaaaaan" (Japanese ad) */

#include <iostream>
#include <string>
#include <vector>
#include <sstream>
using namespace std;

int main() {
    string line;
    getline(cin, line);
    stringstream ss(line);
    string word;
    int max = 0;
    string maxstr;
    while (ss >> word) {
        if (word.size() > max) {
            maxstr = word;
            max = word.size(); } }
    cout << maxstr << endl << max;
    return 0; }
