public class BubbleSort {
    public <T extends Comparable<T>> void sort(T[] arr) {
        int n = arr.length - 1;
        for (int i = 0; i < n; i++) {
            boolean swapped = false;
            for (int j = 0; j < n - i; j++) {
                if (arr[j].compareTo(arr[j + 1]) > 0) {
                    T temp = arr[j];
                    arr[j] = arr[j + 1];
                    arr[j + 1] =  temp;
                    swapped = true;
                }
            }

            if (swapped == false)
                break;
        }
    }
}