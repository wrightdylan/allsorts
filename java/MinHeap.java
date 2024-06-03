// A class for Min Heap
public class MinHeap<T extends Comparable<T>> {
    // max size of the heap
    private int capacity;

    // To store array of elements in heap
    private T[] heapArray;

    // Current number of elements in the heap
    private int current_heap_size;

    // Deleted key generic
    private final T DELETED_KEY;
    
    // Constructor
    @SuppressWarnings("unchecked")
    public MinHeap(int n, T deletedKey) {
        capacity = n;
        heapArray = (T[]) new Comparable[capacity];
        current_heap_size = 0;
        DELETED_KEY = deletedKey;
    }

    // Swapping using reference
    private void swap(T[] arr, int a, int b) {
        T temp = arr[a];
        arr[a] = arr[b];
        arr[b] = temp;
    }

    // Get the Parent index for the given index
    private int parent(int key) {
        return (key - 1) / 2;
    }

    // Get the Left child index for the given index
    private int left(int key) {
        return 2 * key + 1;
    }

    // Get the Right child index for the given index
    private int right(int key) {
        return 2 * key + 2;
    }

    // Inserts a new element
    public boolean insert(T element) {
        if (current_heap_size == capacity)
            return false;

        int i = current_heap_size;
        heapArray[i] = element;
        current_heap_size++;

        // Fix the min heap property if it is violated
        while (i != 0 && heapArray[i].compareTo(heapArray[parent(i)]) < 0) {
            swap(heapArray, i, parent(i));
            i = parent(i);
        }
        return true;
    }

    // Decreases value of given key to new val.
    // It is assumed that new_val is smaller
    // than heapArray[key].
    public void decreaseKey(int key, T new_val) {
        heapArray[key] = new_val;

        while (key != 0 && heapArray[key].compareTo(heapArray[parent(key)]) < 0) {
            swap(heapArray, key,parent(key));
            key = parent(key);
        }
    }

    // Return the minimum key (key at
    // root) from min heap
    public T getMin() {
        return heapArray[0];
    }

    // Method to remove minimum element
    // (or root) from min heap
    public T extractMin() {
        if (current_heap_size <= 0) {
            return null;
        }

        if (current_heap_size == 1) {
            current_heap_size--;
            return heapArray[0];
        }

        // Store the minimum value and remove it from the heap
        T root = heapArray[0];

        heapArray[0] = heapArray[current_heap_size - 1];
        current_heap_size--;
        minHeapify(0);

        return root;
    }

    // This function deletes key at the
    // given index. It first reduced value
    // to minus infinite, then calls extractMin()
    public void deleteKey(int key) { 
        decreaseKey(key, DELETED_KEY); 
        extractMin(); 
    }

    // A recursive method to heapify a subtree
    // with the root at given index
    // This method assumes that the subtrees
    // are already heapified
    private void minHeapify(int key) {
        int left = left(key);
        int right = right(key);

        int smallest = key;
        if (left < current_heap_size && heapArray[left].compareTo(heapArray[smallest]) < 0)
            smallest = left;
        if (right < current_heap_size && heapArray[right].compareTo(heapArray[smallest]) < 0)
        smallest = right;

        if (smallest != key) {
            swap(heapArray, key, smallest);
            minHeapify(smallest);
        }
    }

    // Increases value of given key to new_val.
    // It is assumed that new_val is greater
    // than heapArray[key].
    // Heapify from the given key
    public void inreaseKey(int key, T new_val) {
        heapArray[key] = new_val;
        minHeapify(key);
    }

    // Changes value on a key
    public void changeValueOnAKey(int key, T new_val) {
        if (heapArray[key] == new_val)
            return;
        if (heapArray[key].compareTo(new_val) < 0) {
            inreaseKey(key, new_val);
        } else {
            decreaseKey(key, new_val);
        }
    }
}
