use allsorts::*;

fn main() {
    // Heap sort
    println!("Heap Sort");
    let mut hsa1 = [12, 11, 13, 5, 6, 7];
    let mut hsa2 = [11.4, 4.2, 24.1, 4.3, 56.9, 17.4, 4.3, -1.2];
    heap_sort(&mut hsa1);
    heap_sort(&mut hsa2);
    println!("i32: {:?}", hsa1);
    println!("f64: {:?}", hsa2);

    // Binary heap sort
    println!("Binary Heap Sort");
    let mut bh = Heap::from((vec![3, 2], HeapType::Min));
    bh.delete_key(0);
    bh.insert(15);
    bh.insert(5);
    bh.insert(4);
    bh.insert(45);
    print!("{} {} ", bh.pop().unwrap(), bh.get_root());
    bh.decrease(1, 1);
    print!("{}\n", bh.get_root());
    bh.insert(16);
    bh.insert(22);
    bh.insert(3);
    println!("{}", bh);
    let mut bh2 = Heap::from((vec![3.1, 2.2], HeapType::Max));
    bh2.insert(22.3);
    bh2.insert(14.8);
    bh2.insert(3.2);
    bh2.insert(-0.3);
    println!("{}", bh2);

    // Quicksort with Lomute partitioning scheme
    println!("Quicksort (Lomuto)");
    let mut qsa1 = [10, 7, 8, 9, 1, 5];
    let mut qsa2 = [11.4, 4.2, 24.1, 4.3, 56.9, 17.4, 4.3, -1.2];
    quicksort(&mut qsa1, Scheme::Lomuto);
    quicksort(&mut qsa2, Scheme::Lomuto);
    println!("i32: {:?}", qsa1);
    println!("f64: {:?}", qsa2);

    // Quicksort with Hoare partitioning scheme
    println!("Quicksort (Hoare)");
    let mut qsa3 = [10, 7, 8, 9, 1, 5];
    let mut qsa4 = [11.4, 4.2, 24.1, 4.3, 56.9, 17.4, 4.3, -1.2];
    quicksort(&mut qsa3, Scheme::Hoare);
    quicksort(&mut qsa4, Scheme::Hoare);
    println!("i32: {:?}", qsa3);
    println!("f64: {:?}", qsa4);

    // Merge sort
    println!("Merge Sort");
    let mut msa1 = [12, 11, 13, 5, 6, 7];
    let mut msa2 = [11.4, 4.2, 24.1, 4.3, 56.9, 17.4, 4.3, -1.2];
    merge_sort(&mut msa1);
    merge_sort(&mut msa2);
    println!("i32: {:?}", msa1);
    println!("f64: {:?}", msa2);

    // Insertion sort
    println!("Insertion Sort");
    let mut isa1 = [12, 11, 13, 5, 6];
    let mut isa2 = [6.2, 3.3, 1.8, 0.5, -12.0, 15.2];
    ins_sort(&mut isa1);
    ins_sort(&mut isa2);
    println!("i32: {:?}", isa1);
    println!("f64: {:?}", isa2);

    // Bubble sort
    println!("Bubble Sort");
    let mut bsa1 = [12, 11, 13, 5, 6, 7];
    let mut bsa2 = [11.4, 4.2, 24.1, 4.3, 56.9, 17.4, 4.3, -1.2];
    bubble_sort(&mut bsa1, SortOrder::Asc);
    bubble_sort(&mut bsa2, SortOrder::Asc);
    println!("i32 (asc): {:?}", bsa1);
    println!("f64 (asc): {:?}", bsa2);
    bubble_sort(&mut bsa1, SortOrder::Desc);
    bubble_sort(&mut bsa2, SortOrder::Desc);
    println!("i32 (desc): {:?}", bsa1);
    println!("f64 (desc): {:?}", bsa2);

    // Selection sort
    println!("Selection sort");
    let mut ssa1 = [12, 11, 13, 5, 6, 7];
    let mut ssa2 = [11.4, 4.2, 24.1, 4.3, 56.9, 17.4, 4.3, -1.2];
    sel_sort(&mut ssa1);
    sel_sort(&mut ssa2);
    println!("i32: {:?}", ssa1);
    println!("f64: {:?}", ssa2);

    // Shell sort
    println!("Shell sort");
    let mut ssa3 = [12, 11, 13, 5, 6, 7];
    let mut ssa4 = [11.4, 4.2, 24.1, 4.3, 56.9, 17.4, 4.3, -1.2];
    shell_sort(&mut ssa3);
    shell_sort(&mut ssa4);
    println!("i32: {:?}", ssa3);
    println!("f64: {:?}", ssa4);

    // Bogosort
    println!("Bogosort");
    let mut bsa3 = [12, 11, 13, 5, 6, 7];
    let mut bsa4 = [11.4, 4.2, 24.1, 4.3, 56.9, 17.4, 4.3, -1.2];
    let bsatt1 = bogosort(&mut bsa3);
    println!("i32: {:?} in {} attempts", bsa3, bsatt1);
    let bsatt2 = bogosort(&mut bsa4);
    println!("f64: {:?} in {} attempts", bsa4, bsatt2);

    // Timsort
    println!("Timsort");
    let mut tsa1 = [12, 11, 13, 5, 6, 7];
    let mut tsa2 = [11.4, 4.2, 24.1, 4.3, 56.9, 17.4, 4.3, -1.2];
    timsort(&mut tsa1);
    timsort(&mut tsa2);
    println!("i32: {:?}", tsa1);
    println!("f64: {:?}", tsa2);
}
