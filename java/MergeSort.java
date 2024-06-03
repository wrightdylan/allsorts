public class MergeSort {
    // Merges two subarrays of arr[].
    // First subarray is arr[l..m]
    // Second subarray is arr[m+1..r]
    public <T extends Comparable<T>> void merge(T[] arr, int left, int mid, int right) {
        // Find sizes of two subarrays to be merged
        int n1 = mid - left + 1;
        int n2 = right - mid;

        // Create temp arrays
        T Left[] = (T[]) new Comparable[n1];
        T Right[] = (T[]) new Comparable[n2];

        // Copy data to temp arrays
        for (int i = 0; i < n1; ++i)
            Left[i] = arr[left + i];
        for (int j = 0; j < n2; ++j)
            Right[j] = arr[mid + 1 + j];
        
        // Merge the temp arrays

        // Initial indices of first and second subarrays
        int i = 0, j = 0;

        // Initial index of merged subarray array
        int k = left;
        while (i < n1 && j < n2) {
            if (Left[i].compareTo(Right[j]) <= 0) {
                arr[k] = Left[i];
                i++;
            } else {
                arr[k] = Right[j];
                j++;
            }
            k++;
        }

        // Copy remaining elements of L[] if any
        while (i < n1) {
            arr[k] = Left[i];
            i++;
            k++;
        }

        // Copy remaining elements of R[] if any
        while (j < n2) {
            arr[k] = Right[j];
            j++;
            k++;
        }
    }

    <T extends Comparable<T>> void sort(T[] arr, int left, int right) {
        if (left < right) {
            // Find the middle point
            int m = left + (right - left) / 2;

            // Sort first and second halves
            sort(arr, left, m);
            sort(arr, m + 1, right);

            // Merge the sorted halves
            merge(arr, left, m, right);
        }
    }

    // A utility function to print array of size n
    static <T> void printArray(T[] arr) {
        for (T element : arr)
            System.out.print(element + " ");
        System.out.println();
    }
}
