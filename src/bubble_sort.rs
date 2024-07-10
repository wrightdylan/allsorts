pub enum SortOrder {
    Asc,
    Desc,
}
pub fn bubble_sort<T: PartialOrd>(arr: &mut [T], ord: SortOrder) {
    let n = arr.len() - 1;
    for i in 0..n {
        let mut swapped = false;
        for j in 0..n - i {
            match ord {
                SortOrder::Asc => {
                    if arr[j] > arr[j + 1] {
                        arr.swap(j, j + 1);
                        swapped = true;
                    }
                },
                SortOrder::Desc => {
                    if arr[j] < arr[j + 1] {
                        arr.swap(j, j + 1);
                        swapped = true;
                    }
                },
            }
            
        }
        if !swapped {
            break;
        }
    }
}