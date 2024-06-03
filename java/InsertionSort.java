public class InsertionSort {
    public <T extends Comparable<T>> void sort(T[] arr) {
        int n = arr.length;
        for (int i = 1; i < n; ++i) {
            T value = arr[i];
            int j = i - 1;

            // Move elements of arr[0..i-1], that are
            // greater than value, to one position
            // ahead of their current position
            while (j >= 0 && arr[j].compareTo(value) > 0) {
                arr[j + 1] = arr[j];
                j--;
            }
            arr[j + 1] = value;
        }
    }

    // A utility function to print array of size n
    static <T> void printArray(T[] arr) {
        for (T element : arr)
            System.out.print(element + " ");
        System.out.println();
    }
}
