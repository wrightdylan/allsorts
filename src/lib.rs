//! `allsorts` is a collection of basic sort algorithms
//! 
//! ### Available algorithms
//! * Bubble sort
//! * Heap sort
//! * Heap sort (alternative)
//! * Insertion sort
//! * Merge sort
//! * Quicksort
//! * Selection sort
//! 
//! All sort algorithms allow generic types

mod bheap;
mod bubble_sort;
mod heap_sort;
mod insertion_sort;
mod merge_sort;
mod quicksort;
mod selection_sort;

pub use bheap::*;
pub use bubble_sort::*;
pub use heap_sort::heap_sort;
pub use insertion_sort::ins_sort;
pub use merge_sort::merge_sort;
pub use quicksort::quicksort;
pub use selection_sort::sel_sort;