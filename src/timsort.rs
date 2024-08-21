use crate::ins_sort;
use crate::merge_sort;

pub fn timsort<T: PartialOrd + Copy>(arr: &mut [T]) {
    let min_run = 32;

    let n = arr.len();

    // Sort subarrays of size min_run
    for i in (0..n).step_by(min_run) {
        let end = usize::min(i + min_run, n);
        ins_sort(&mut arr[i..end]);
    }

    // Merge sorted runs
    let mut run_size = min_run;
    while run_size < n {
        for i in (0..n).step_by(2 * run_size) {
            let mid = usize::min(i + run_size, n);
            let end = usize::min(i + 2 * run_size, n);

            if mid < end {
                merge_sort(&mut arr[i..end]);
            }
        }

        run_size *= 2;
    }
}