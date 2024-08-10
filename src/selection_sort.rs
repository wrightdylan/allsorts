pub fn sel_sort<T: PartialOrd>(arr: &mut [T]) {
    let n = arr.len();

    for i in 0..n - 1 {
        let mut min_idx = i;
        for j in (i + 1)..n {
            if arr[j] < arr[min_idx] {
                min_idx = j;
            }
        }
        arr.swap(i, min_idx);
    }
}