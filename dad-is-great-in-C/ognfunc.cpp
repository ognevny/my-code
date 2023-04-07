// a list of useful functions written by me

#include <iostream>
#include <string>
#include <map>
using namespace std;

uint16_t numsum(uint64_t n) { // sum of number digits
    uint16_t res = 0;
    while (n) {
        res += n % 10;
        n /= 10; }
    return res; }

uint64_t rev(uint64_t n) { // reverse number
   uint64_t rev = 0; 
   while (n) {
       rev = rev * 10 + (n % 10);
       n /= 10; }
    return rev; }
    
uint64_t numsys(uint64_t n, uint8_t k) {
    uint64_t ns = 1;
    while (n) {
        ns = ns * 10 + (n % k);
        n /= k; }
    return (rev(ns) - 1) / 10; }

uint16_t numcount(uint64_t n) { // counts number digits
    uint16_t count = 0;
    while (n) {
        n /= 10;
        count++; }
    return count; }

string numsys16(uint64_t n, uint8_t k) {
    map<uint8_t, string> nums = {{0, "0"}, {1, "1"}, {2, "2"}, {3, "3"}, {4, "4"}, {5, "5"}, {6, "6"}, {7, "7"}, 
                                 {8, "8"}, {9, "9"}, {10, "A"}, {11, "B"}, {12, "C"}, {13, "D"}, {14, "E"}, {15, "F"}};
    string res = "";
    while (n) {
        res += nums[n % k];
        n /= k; }
    string rev(res.rbegin(), res.rend());
    return rev; }

long double binpow(double b, uint16_t e) { // binary power
    long double v = 1.0;
    while (e != 0) {
        if ((e & 1) != 0) v *= b;
        b *= b;
        e >>= 1; }
    return v; }
