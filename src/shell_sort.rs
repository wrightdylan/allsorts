pub fn shell_sort<T: PartialOrd + Copy>(arr: &mut [T]) {
    let n = arr.len();

    // Use Marcin Ciura's gap sequence for optimal performance
    let gaps = [701, 301, 132, 57, 23, 10, 4, 1];

    for &gap in gaps.iter() {
        let mut i = gap;
        while i < n {
            let mut j = i;
            while j >= gap && arr[j - gap] > arr[j] {
                arr.swap(j, j - gap);
                j -= gap;
            }
            i += 1;
        }
    }
}