use bheap::*;
use bubble_sort::SortOrder;

mod bheap;
mod bubble_sort;
mod heap_sort;
mod insertion_sort;
mod merge_sort;
mod quicksort;

fn main() {
    println!("Heap Sort");
    let mut hsa1 = [12, 11, 13, 5, 6, 7];
    let mut hsa2 = [11.4, 4.2, 24.1, 56.9, 17.4, 4.3, -1.2];
    heap_sort::sort(&mut hsa1);
    heap_sort::sort(&mut hsa2);
    println!("i32: {:?}", hsa1);
    println!("f64: {:?}", hsa2);

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

    println!("Quicksort");
    let mut qsa1 = [10, 7, 8, 9, 1, 5];
    let mut qsa2 = [11.4, 4.2, 24.1, 56.9, 17.4, 4.3, -1.2];
    quicksort::quicksort(&mut qsa1);
    quicksort::quicksort(&mut qsa2);
    println!("i32: {:?}", qsa1);
    println!("f64: {:?}", qsa2);

    println!("Merge Sort");
    let mut msa1 = [12, 11, 13, 5, 6, 7];
    let mut msa2 = [11.4, 4.2, 24.1, 56.9, 17.4, 4.3, -1.2];
    merge_sort::merge_sort(&mut msa1);
    merge_sort::merge_sort(&mut msa2);
    println!("i32: {:?}", msa1);
    println!("f64: {:?}", msa2);

    println!("Insertion Sort");
    let mut isa1 = [12, 11, 13, 5, 6];
    let mut isa2 = [6.2, 3.3, 1.8, 0.5, -12.0, 15.2];
    insertion_sort::ins_sort(&mut isa1);
    insertion_sort::ins_sort(&mut isa2);
    println!("i32: {:?}", isa1);
    println!("f64: {:?}", isa2);

    println!("Bubble Sort");
    let mut bsa1 = [12, 11, 13, 5, 6, 7];
    let mut bsa2 = [11.4, 4.2, 24.1, 56.9, 17.4, 4.3, -1.2];
    bubble_sort::bubble_sort(&mut bsa1, SortOrder::Asc);
    bubble_sort::bubble_sort(&mut bsa2, SortOrder::Asc);
    println!("i32 (asc): {:?}", bsa1);
    println!("f64 (asc): {:?}", bsa2);
    bubble_sort::bubble_sort(&mut bsa1, SortOrder::Desc);
    bubble_sort::bubble_sort(&mut bsa2, SortOrder::Desc);
    println!("i32 (desc): {:?}", bsa1);
    println!("f64 (desc): {:?}", bsa2);
}
