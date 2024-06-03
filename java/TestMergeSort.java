public class TestMergeSort {
    public static void main(String[] args) {
        Integer[] arr1 = { 12, 11, 13, 5, 6, 7 };
        Double[] arr2 = { 11.4, 4.2, 24.1, 56.9, 17.4, 4.3, -1.2 };

        MergeSort ob1 = new MergeSort();
        ob1.sort(arr1, 0, arr1.length - 1);
        MergeSort ob2 = new MergeSort();
        ob2.sort(arr2, 0, arr2.length - 1);

        System.out.println("Sorted int array is");
        MergeSort.printArray(arr1);
        System.out.println("Sorted double array is");
        MergeSort.printArray(arr2);
    }
}
