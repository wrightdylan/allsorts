public class HeapSort {
    public <T extends Comparable<T>> void sort(T[] arr) {
        int n = arr.length;

        // Build heap (rearrange array)
        for (int i = n / 2 - 1; i >= 0; i--)
            heapify(arr, n, i);

        // One by one extract an element from heap
        for (int i = n - 1; i >= 0; i--) {
            // Move current root to end
            T temp = arr[0];
            arr[0] = arr[i];
            arr[i] = temp;

            // call max heapify on the reduced heap
            heapify(arr, i, 0);
        }
    }

    // To heapify a subtree rooted with the node i which is
    // an index in arr[]. n is size of heap
    private <T extends Comparable<T>> void heapify(T[] arr, int n, int i) {
        int largest = i; // Initialize largest as root
        int left = 2 * i + 1;
        int right = 2 * i + 2;

        // If left child is larger than root
        if (left < n && arr[left].compareTo(arr[largest]) > 0)
            largest = left;

        // If right child is larger than the largest so far
        if (right < n && arr[right].compareTo(arr[largest]) > 0)
            largest = right;
        
        // If largest is not root
        if (largest != i) {
            T swap = arr[i];
            arr[i] = arr[largest];
            arr[largest] = swap;

            // Recursively heapify the affected sub-tree
            heapify(arr, n, largest);
        }
    }

    // A utility function to print array of size n
    static <T> void printArray(T[] arr) {
        for (T element : arr)
            System.out.print(element + " ");
        System.out.println();
    }
}