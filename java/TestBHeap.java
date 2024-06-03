public class TestBHeap {
    public static void main(String[] args) {
        MinHeap<Integer> h = new MinHeap<>(11, Integer.MIN_VALUE);
        h.insert(3);
        h.insert(2);
        h.deleteKey(1);
        h.insert(15);
        h.insert(5);
        h.insert(4);
        h.insert(45);
        System.out.print(h.extractMin() + " ");
        System.out.print(h.getMin() + " ");

        h.decreaseKey(2, 1);
        System.out.print(h.getMin());
        System.out.println(); // 2 4 1
    }
}
