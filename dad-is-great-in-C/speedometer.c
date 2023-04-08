// a programm to test speed of each language (C implementation)

#include <stdio.h>

int s() {
    int n = 1;
    while (n < 1000000000) n++;
    return n; }

int main() {
    printf("%d", s());
    return 0; }
