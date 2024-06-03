public class TestInsertionSort {
    public static void main(String[] args) {
        Integer[] arr = { 12, 11, 13, 5, 6 };

        InsertionSort ob = new InsertionSort();
        ob.sort(arr);

        InsertionSort.printArray(arr);
    }
}
