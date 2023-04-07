// merge sort

#include <iostream>
#include <ctime>
#include <random>
using namespace std;

void merge(int arr[], int left, int mid, int right) {
    int rt = mid - left + 1, lt = right - mid; 
    int *left_arr = new int[rt], *right_arr = new int[lt]; // создаём два новых массива

    for (int i = 0; i < rt; i++) left_arr[i] = arr[left + i]; // заполняем их
    for (int i = 0; i < lt; i++) right_arr[i] = arr[mid + 1 + i];

    int i_right = 0, i_left = 0, i_merged = left; // индексы, по которым мы идём вдоль массивов

    while (i_right < rt and i_left < lt) { // слияние
        if (left_arr[i_right] <= right_arr[i_left]) {
            arr[i_merged] = left_arr[i_right];
            i_right++; }
        else {
            arr[i_merged] = right_arr[i_left];
            i_left++; }
        i_merged++; }
    
    while (i_right < rt) { // дополняем из левого массива
        arr[i_merged] = left_arr[i_right];
        i_right++;
        i_merged++; }

    while (i_left < lt) { // дополняем из правого
        arr[i_merged] = right_arr[i_left];
        i_left++;
        i_merged++; }

    delete[] left_arr;
    delete[] right_arr; }

void merge_sort(int arr[], int begin, int end) {
    if (begin >= end) return; // раздел до единичных
 
    int mid = (end + begin) / 2;
    merge_sort(arr, begin, mid);
    merge_sort(arr, mid + 1, end);
    merge(arr, begin, mid, end); }

int main() {
    int n;
    cin >> n;
    clock_t st = clock();
    int a[n];
    srand(time(NULL));
    for (int i = 0; i < n; i++) a[i] = rand(); 
    merge_sort(a, 0, n - 1);
    // for (int i : a) cout << i << " ";
    clock_t end = clock();
    cout << endl << (double)(end - st) / CLOCKS_PER_SEC; 
    return 0; }
