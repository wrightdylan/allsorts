public class TestAll {
    public static void main(String[] args) {
        // Heap sort
        System.out.println("Heap Sort");
        Integer[] hsa1 = { 12, 11, 13, 5, 6, 7 };
        Double[] hsa2 = { 11.4, 4.2, 24.1, 56.9, 17.4, 4.3, -1.2 };

        HeapSort hsob1 = new HeapSort();
        hsob1.sort(hsa1);
        HeapSort hsob2 = new HeapSort();
        hsob2.sort(hsa2);

        System.out.println("Sorted int array is");
        printArray(hsa1);
        System.out.println("Sorted double array is");
        printArray(hsa2);

        // Binary heap sort
        System.out.println("Binary Heap Sort");
        MinHeap<Integer> bh = new MinHeap<>(11, Integer.MIN_VALUE);
        bh.insert(3);
        bh.insert(2);
        bh.deleteKey(1);
        bh.insert(15);
        bh.insert(5);
        bh.insert(4);
        bh.insert(45);
        System.out.print(bh.extractMin() + " ");
        System.out.print(bh.getMin() + " ");

        bh.decreaseKey(2, 1);
        System.out.print(bh.getMin());
        System.out.println(); // 2 4 1

        // Quicksort
        System.out.println("Quicksort");
        Integer[] qsa = { 10, 7, 8, 9, 1, 5 };
        QuickSort.quickSort(qsa, 0, qsa.length - 1);
        System.out.println("Sorted array:");
        printArray(qsa);

        // Merge sort
        System.out.println("Merge Sort");
        Integer[] msa1 = { 12, 11, 13, 5, 6, 7 };
        Double[] msa2 = { 11.4, 4.2, 24.1, 56.9, 17.4, 4.3, -1.2 };

        MergeSort msob1 = new MergeSort();
        msob1.sort(msa1, 0, msa1.length - 1);
        MergeSort msob2 = new MergeSort();
        msob2.sort(msa2, 0, msa2.length - 1);

        System.out.println("Sorted int array is");
        printArray(msa1);
        System.out.println("Sorted double array is");
        printArray(msa2);

        // Insertion sort
        System.out.println("Insertion Sort");
        Integer[] isa = { 12, 11, 13, 5, 6 };

        InsertionSort isob = new InsertionSort();
        isob.sort(isa);

        printArray(isa);

        // Bubble sort
        System.out.println("Bubble Sort");
        Integer[] bsa1 = { 12, 11, 13, 5, 6, 7 };
        Double[] bsa2 = { 11.4, 4.2, 24.1, 56.9, 17.4, 4.3, -1.2 };

        BubbleSort bsob1 = new BubbleSort();
        bsob1.sort(bsa1);
        BubbleSort bsob2 = new BubbleSort();
        bsob2.sort(bsa2);

        System.out.println("Sorted int array is");
        printArray(bsa1);
        System.out.println("Sorted double array is");
        printArray(bsa2);

        // Selection sort
        System.out.println("Selection Sort");
        Integer[] ssa1 = { 12, 11, 13, 5, 6, 7 };
        Double[] ssa2 = { 11.4, 4.2, 24.1, 56.9, 17.4, 4.3, -1.2 };

        SelectionSort ssob1 = new SelectionSort();
        ssob1.sort(ssa1);
        SelectionSort ssob2 = new SelectionSort();
        ssob2.sort(ssa2);

        System.out.println("Sorted int array is");
        printArray(ssa1);
        System.out.println("Sorted double array is");
        printArray(ssa2);

        // Shell sort
        System.out.println("Shell Sort");
        Integer[] ssa3 = { 12, 11, 13, 5, 6, 7 };
        Double[] ssa4 = { 11.4, 4.2, 24.1, 56.9, 17.4, 4.3, -1.2 };

        SelectionSort ssob3 = new SelectionSort();
        ssob3.sort(ssa3);
        SelectionSort ssob4 = new SelectionSort();
        ssob4.sort(ssa4);

        System.out.println("Sorted int array is");
        printArray(ssa3);
        System.out.println("Sorted double array is");
        printArray(ssa4);
    }

    static <T> void printArray(T[] arr) {
        for (T element : arr)
            System.out.print(element + " ");
        System.out.println();
    }
}
