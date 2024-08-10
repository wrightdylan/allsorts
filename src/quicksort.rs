pub enum Scheme {
    Lomuto,
    Hoare,
}

pub fn quicksort<T: PartialOrd + Copy>(arr: &mut [T], scheme: Scheme) {
    _quicksort(arr, 0, (arr.len() - 1) as isize, &scheme);
}

fn _quicksort<T: PartialOrd + Copy>(arr: &mut [T], left: isize, right: isize, scheme: &Scheme) {
    if left <= right {
        let partition_idx = match scheme {
            Scheme::Lomuto => lomuto(arr, left, right),
            Scheme::Hoare  => hoare(arr, left as usize, right as usize),
        };

        _quicksort(arr, left, partition_idx - 1, &scheme);
        _quicksort(arr, partition_idx + 1, right, &scheme);
    }
}

fn hoare<T: PartialOrd + Copy>(arr: &mut [T], left: usize, right: usize) -> isize {
    let pivot = arr[left];
    let mut i = left;
    let mut j = right;

    loop {
        while arr[i] < pivot {
            i += 1;
        }

        while arr[j] > pivot {
            j -= 1;
        }

        if i >= j {
            return j as isize;
        }

        arr.swap(i, j);
        i += 1;
        j -= 1;
    }
}

fn lomuto<T: PartialOrd + Copy>(arr: &mut [T], left: isize, right: isize) -> isize {
    let pivot = arr[right as usize];
    let mut i = left;

    for j in left..right {
        if arr[j as usize] <= pivot {
            arr.swap(i as usize, j as usize);
            i += 1;
        }
    }

    arr.swap(i as usize, right as usize);

    i
}