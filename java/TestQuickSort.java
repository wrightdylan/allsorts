public class TestQuickSort {
    public static void main(String[] args) {
        Integer[] arr = { 10, 7, 8, 9, 1, 5 };
        QuickSort.quickSort(arr, 0, arr.length - 1);
        System.out.println("Sorted array:");
        QuickSort.printArr(arr);
    }
}
