public class TestBubbleSort {
    public static void main(String[] args) {
        Integer[] arr1 = { 12, 11, 13, 5, 6, 7 };
        Double[] arr2 = { 11.4, 4.2, 24.1, 56.9, 17.4, 4.3, -1.2 };

        BubbleSort ob1 = new BubbleSort();
        ob1.sort(arr1);
        BubbleSort ob2 = new BubbleSort();
        ob2.sort(arr2);

        System.out.println("Sorted int array is");
        MergeSort.printArray(arr1);
        System.out.println("Sorted double array is");
        MergeSort.printArray(arr2);
    }
}
