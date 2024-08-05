// main function to do the heap sort
pub fn heap_sort<T: PartialOrd>(arr: &mut [T]) {
    let n = arr.len();

    // Build heap (rearrange array)
    for i in (0..=n / 2 - 1).rev() {
        heapify(arr, n, i);
    }

    // One by one extract an element from heap
    for i in (0..n).rev() {
        // Move the current root to the end
        arr.swap(0, i);

        // Call max meapify on the reduced heap
        heapify(arr, i, 0);
    }
}

// To heapify a subtree rooted with node i which is
// an index in arr[], n is size of heap
fn heapify<T: PartialOrd>(arr: &mut [T], n:usize, i: usize) {
    let mut largest = i;  // Initialize largest as root since we are using 0 based indexing
    let left = 2*i + 1;
    let right = 2*i + 2;

    // If the left child is larger than the root
    if left < n && arr[left] > arr[largest] {
        largest = left;
    }

    // If the right child is larger than the largest so far
    if right < n && arr[right] > arr[largest] {
        largest = right;
    }

    // If the largest is not the root
    if largest != i {
        // Swap arr[i] and arr[largest]
        arr.swap(i, largest);
    }
}