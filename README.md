# All Sorts of Sort Algorithms (in Rust and Java)

A small collection of sort algorithms originally written in Java and adapted to Rust. Java code taken from Geeks for Geeks and modified to accept generics.

## Sorting Algorithms

- Binary Heap Sort (Java: Min Heap, Rust: Min and Max Heap). Worst and average time complexity is O(n log<sub>2</sub>n), or O(n) in the best case. Space complexity is O(1).
- Bubble Sort. Time complexity is O(n<sup>2</sup>) for average and worst case, O(n) for best case when the array is already sorted. Space complexity is O(1). Rust version can sort ascending or descending.
- Heap Sort (another implementation).
- Insertion Sort. Time complexity is O(n<sup>2</sup>) for average and worst case, O(n) for best case, and space complexity of O(1).
- Merge Sort. Time complexity is O(n log<sub>2</sub>n) in all cases, and space complexity is O(n).
- Quicksort. This is O(n log<sub>2</sub>n) at best and average, and O(n<sup>2</sup>) in the worst case. Space complexity is O(log<sub>2</sub>n).