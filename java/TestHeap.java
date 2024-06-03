public class TestHeap {
    public static void main(String[] args) {
        Integer[] arr1 = { 12, 11, 13, 5, 6, 7 };
        Double[] arr2 = { 11.4, 4.2, 24.1, 56.9, 17.4, 4.3, -1.2 };

        HeapSort ob1 = new HeapSort();
        ob1.sort(arr1);
        HeapSort ob2 = new HeapSort();
        ob2.sort(arr2);

        System.out.println("Sorted int array is");
        HeapSort.printArray(arr1);
        System.out.println("Sorted double array is");
        HeapSort.printArray(arr2);
    }
}
