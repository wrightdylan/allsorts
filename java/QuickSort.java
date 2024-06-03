public class QuickSort {
    // A utility function to swap two elements
    static <T> void swap(T[] arr, int i, int j) {
        T temp = arr[i];
        arr[i] = arr[j];
        arr[j] = temp;
    }

    // This function takes last element as pivot,
    // places the pivot element at its correct position
    // in sorted array, and places all smaller to left
    // of pivot and all greater elements to right of pivot
    static <T extends Comparable<T>> int partition(T[] arr, int low, int high) {
        // Choosing the pivot
        T pivot = arr[high];

        // Index of smaller element and indicates
        // the rightposition of pivot found so far
        int i = (low - 1);

        for (int j = low; j <= high - 1; j++) {
            // If current element is smaller than the pivot
            if(arr[j].compareTo(pivot) < 0) {
                // Increment index of smaller element
                i++;
                swap(arr, i, j);
            }
        }
        swap(arr, i + 1, high);
        return (i + 1);
    }

    // The main function that implements QuickSort
    // arr[] --> Array to be sorted,
    // low --> Starting index,
    // high --> Ending index
    static <T extends Comparable<T>> void quickSort(T[] arr, int low, int high) {
        if (low < high) {
            // pi is partitioning index, arr[p]
            // is now at right place
            int pi = partition(arr, low, high);

            // Separately sort elements before
            // partition and after partition
            quickSort(arr, low, pi - 1);
            quickSort(arr, pi + 1, high);
        }
    }

    // To print sorted array
    public static <T> void printArr(T[] arr) {
        for (T element : arr)
            System.out.print(element + " ");
        System.out.println();
    }
}
