// TODO: write a code for more sorts

/// Bubble sort algorithm
/// # Examples
///
/// ```
/// use ognlib::algorithm::sort::bubble;
///
/// let mut arr = vec![5, 3, 4, 1, 2];
/// bubble(&mut arr);
/// assert_eq!(arr, [1, 2, 3, 4, 5]);
/// ```

pub fn bubble<T: Ord>(arr: &mut [T]) {
    let n = arr.len();
    for i in 0..n {
        for j in 0..n - i - 1 {
            if arr[j] > arr[j + 1] { arr.swap(j + 1, j) } } } }


/// Search sort algorithm
/// # Examples
///
/// ```
/// use ognlib::algorithm::sort::search;
///
/// let mut arr = vec![5, 3, 4, 1, 2];
/// search(&mut arr);
/// assert_eq!(arr, [1, 2, 3, 4, 5]);
/// ```

pub fn search<T: Ord>(arr: &mut [T]) {
    let n = arr.len();
    for i in 0..n - 1 {
        let mut i_min = i;
        for j in i + 1..n {
            if arr[j] < arr[i_min] { i_min = j } }
        if i_min != i { arr.swap(i, i_min) } } }


/// Insertion sort algorithm
/// # Examples
///
/// ```
/// use ognlib::algorithm::sort::insertion;
///
/// let mut arr = vec![5, 3, 4, 1, 2];
/// insertion(&mut arr);
/// assert_eq!(arr, [1, 2, 3, 4, 5]);
/// ```

pub fn insertion<T: Ord>(arr: &mut [T]) {
    for i in 1..arr.len() {
        let key = &arr[i];
        let mut j = i;
        let pos = arr[..i].binary_search(key).unwrap_or_else(|pos| pos);
        while j > pos {
            arr.swap(j - 1, j);
            j -= 1; } } }


/// Merge sort algorithm
/// # Examples
///
/// ```
/// use ognlib::algorithm::sort::merge;
/// 
/// let mut arr = vec![5, 3, 4, 1, 2];
/// arr = merge(&arr);
/// assert_eq!(arr, [1, 2, 3, 4, 5]);
/// ```

pub fn merge<T: Ord + Clone + Copy>(arr: &Vec<T>) -> Vec<T> {
    if arr.len() >= 2 {
        let mid = arr.len() / 2;
        let left = merge(&arr[..mid].to_vec());
        let right = merge(&arr[mid..].to_vec());
        merging(&left, &right) }
    else { arr.to_vec() } }
            
fn merging<T: Ord + Clone + Copy>(left: &Vec<T>, right: &Vec<T>) -> Vec<T> {
    let (mut i, mut j) = (0, 0);
    let mut merged: Vec<T> = Vec::new();
    while i < left.len() && j < right.len() {
        if left[i] < right[j] {
            merged.push(left[i]);
            i += 1; } 
        else {
            merged.push(right[j]);
            j += 1; } }
    if i < left.len() {
        merged.extend_from_slice(&left[i..]); }
    if j < right.len() {
        merged.extend_from_slice(&right[j..]); }
    merged } 
