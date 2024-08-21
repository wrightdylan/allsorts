use rand::{Rng, thread_rng};

/// Bogosort returns the number of attempts required to sort the array.
/// Since this is random, the number can vary quite considerably.
/// The f64 array in the examples has taken between 10 and 77,000 attempts in trial runs.
pub fn bogosort<T: PartialOrd>(arr: &mut [T]) -> usize {
    let mut attempts = 0;
    while !is_sorted(arr) {
        shuffle(arr);
        attempts += 1;
    }

    attempts
}

fn is_sorted<T: PartialOrd>(arr: &[T]) -> bool {
    for i in 1..arr.len() {
        if arr[i] < arr[i - 1] {
            return false;
        }
    }
    true
}

fn shuffle<T>(arr: &mut [T]) {
    let mut rng = thread_rng();
    for i in 1..arr.len() {
        let rdm_idx = rng.gen_range(0..=i);
        arr.swap(i, rdm_idx);
    }
}