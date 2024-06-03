pub fn ins_sort<T: PartialOrd + Copy>(arr: &mut [T]) {
    for i in 1..arr.len() {
        let value = arr[i];
        let mut j = i;

        while j > 0 && arr[j - 1] > value {
            arr[j] = arr[j - 1];
            j -= 1;
        }
        arr[j] = value;
    }
}