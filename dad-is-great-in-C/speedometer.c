// a programm to test speed of each language (C implementation)

#include <stdio.h>

int s() {
    int n = 1;
    while (n < 1000000000) n++;
    return n; }

int main() {
    int n = s();
    printf("%d", n);
    return 0; }
