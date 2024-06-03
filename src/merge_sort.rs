pub fn merge_sort<T: PartialOrd + Copy>(arr: &mut [T]) {
    _merge_sort(arr, 0, arr.len() - 1);
}

fn _merge_sort<T: PartialOrd + Copy>(arr: &mut [T], left: usize, right: usize) {
    if left < right {
        let mid = (left + right) / 2;
        _merge_sort(arr, left, mid);
        _merge_sort(arr, mid + 1, right);
        merge(arr, left, mid, right);
    }
}

fn merge<T: PartialOrd + Copy>(arr: &mut [T], left: usize, mid: usize, right: usize) {
    let left_arr = &arr[left..=mid].to_owned();
    let right_arr = &arr[mid + 1..=right].to_owned();

    let mut i = 0;
    let mut j = 0;

    for k in left..=right {
        if i < left_arr.len() && j < right_arr.len() {
            if left_arr[i] < right_arr[j] {
                arr[k] = left_arr[i];
                i += 1;
            } else {
                arr[k] = right_arr[j];
                j += 1;
            }
        } else if i >= left_arr.len() {
            arr[k] = right_arr[j];
            j += 1;
        } else {
            arr[k] = left_arr[i];
            i += 1;
        }
    }
}