import java.util.Random;

public class BogoSort {
    public <T extends Comparable<T>> void sort(T[] arr) {
        while (isSorted(arr) == false) {
            shuffle(arr);
        }
    }

    <T extends Comparable<T>> boolean isSorted(T[] arr) {
        for (int i = 1; i < arr.length; i++) {
            if (arr[i].compareTo(arr[i - 1]) < 0) {
                return false;
            }
        }
        return true;
    }

    static <T> void swap(T[] arr, int i, int j) {
        T temp = arr[i];
        arr[i] = arr[j];
        arr[j] = temp;
    }

    static <T> void shuffle(T[] arr) {
        Random rand = new Random();
        for (int i = 1; i < arr.length; i++) {
            int randomIndex = rand.nextInt(i + 1);
            swap(arr, i, randomIndex);
        }
    }
}