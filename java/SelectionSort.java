public class SelectionSort {
    public <T extends Comparable<T>> void sort(T[] arr) {
        int n = arr.length;

        // Move the index of the sorted boundary in the array by one
        for (int i = 0; i < n - 1; i++) {
            int min_idx = i;
            for (int j = i + 1; j < n; j++) {
                if (arr[j].compareTo(arr[min_idx]) < 0) {
                    min_idx = j;
                }
            }

            // Swap elements
            T temp = arr[min_idx];
            arr[min_idx] = arr[i];
            arr[i] = temp;
        }
    }
}
