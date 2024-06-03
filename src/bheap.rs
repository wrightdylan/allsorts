use std::fmt;

pub enum HeapType {
    Min,
    Max,
}
pub struct Heap<T: PartialOrd + Copy> {
    pub data: Vec<T>,
    heap: HeapType,
}

impl<T: PartialOrd + Copy> Heap<T> {
    pub fn new_max() -> Self {
        Heap { data: vec![], heap: HeapType::Max }
    }

    pub fn new_min() -> Self {
        Heap { data: vec![], heap: HeapType::Min }
    }

    // Get the Parent index for the given index
    fn parent(&self, idx: usize) -> usize {
        idx / 2
    }

    // Get the Left child index for the given index
    fn left(&self, idx: usize) -> usize {
        2 * idx + 1
    }

    // Get the Right child index for the given index
    fn right(&self, idx: usize) -> usize {
        2 * idx + 2
    }

    // Inserts a new element
    pub fn insert(&mut self, element: T) {
        self.data.push(element);
        let mut last_element_idx = self.data.len() - 1;

        while last_element_idx != 0 {
            let parent_idx = self.parent(last_element_idx);

            // Parent is smaller in MaxHeap, bigger in MinHeap
            match self.heap {
                HeapType::Max => {
                    if self.data[last_element_idx] > self.data[parent_idx] {
                        self.data.swap(last_element_idx, parent_idx)
                    }
                },
                HeapType::Min => {
                    if self.data[last_element_idx] < self.data[parent_idx] {
                        self.data.swap(last_element_idx, parent_idx)
                    }
                },
            }

            last_element_idx = parent_idx;
        }
    }

    // Deletes a new element
    pub fn delete_key(&mut self, idx: usize) {
        self.data.remove(idx);
        let mut last_element_idx = self.data.len() - 1;

        while last_element_idx != 0 {
            let parent_idx = self.parent(last_element_idx);

            match self.heap {
                HeapType::Max => {
                    if self.data[last_element_idx] > self.data[parent_idx] {
                        self.data.swap(last_element_idx, parent_idx)
                    }
                },
                HeapType::Min => {
                    if self.data[last_element_idx] < self.data[parent_idx] {
                        self.data.swap(last_element_idx, parent_idx)
                    }
                },
            }

            last_element_idx = parent_idx;
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        match self.data.len() {
            n if n == 0 => None,
            n if n == 1 => self.data.pop(),
            _ => {
                let last_element_idx = self.data.len() - 1;
                self.data.swap(0, last_element_idx);
                let value = self.data.pop().unwrap();

                self.heapify(0);

                Some(value)
            }
        }
    }

    pub fn into_sorted_vec(&mut self) -> Vec<T> {
        let mut vec = vec![];

        while self.data.len() > 0 {
            vec.push(self.pop().unwrap());
        }

        vec.reverse();

        vec
    }

    fn heapify(&mut self, idx: usize) {
        let mut current = idx;

        loop {
            let left = self.left(current);
            let right = self.right(current);
            let size = self.data.len() - 1;

            if left > size || right > size {
                break;
            }

            // Children should be less for a MaxHeap or greater for a MinHeap
            match self.heap {
                HeapType::Max => {
                    let max = self.get_max_idx(left, right, current);
        
                    if max != current {
                        self.data.swap(current, max);
        
                        current = max;
                    } else {
                        break;
                    }
                },
                HeapType::Min => {
                    let min = self.get_min_idx(left, right, current);
        
                    if min != current {
                        self.data.swap(current, min);
        
                        current = min;
                    } else {
                        break;
                    }
                },
            }
        }
    }

    fn get_max_idx(&self, left_idx: usize, right_idx: usize, idx: usize) -> usize {
        let left = self.data[left_idx];
        let right = self.data[right_idx];
        let current = self.data[idx];

        if left >= right && left >= current {
            left_idx
        } else if right >= current {
            right_idx
        } else {
            idx
        }
    }

    fn get_min_idx(&self, left_idx: usize, right_idx: usize, idx: usize) -> usize {
        let left = self.data[left_idx];
        let right = self.data[right_idx];
        let current = self.data[idx];

        if left <= right && left <= current {
            left_idx
        } else if right <= current {
            right_idx
        } else {
            idx
        }
    }

    pub fn decrease(&mut self, idx: usize, value: T) {
        self.data[idx] = value;
        let mut last_element_idx = self.data.len() - 1;

        while last_element_idx != 0 {
            let parent_idx = self.parent(last_element_idx);

            // Parent is smaller in MaxHeap, bigger in MinHeap
            match self.heap {
                HeapType::Max => {
                    if self.data[last_element_idx] > self.data[parent_idx] {
                        self.data.swap(last_element_idx, parent_idx)
                    }
                },
                HeapType::Min => {
                    if self.data[last_element_idx] < self.data[parent_idx] {
                        self.data.swap(last_element_idx, parent_idx)
                    }
                },
            }

            last_element_idx = parent_idx;
        }
    }

    // Return the minimum key (key at
    // root) from min heap
    pub fn get_root(&self) -> T {
        self.data[0]
    }
}

impl<T: PartialOrd + Copy + fmt::Debug> fmt::Display for Heap<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Data:")?;
        for item in &self.data {
            write!(f, " {:?}", item)?;
        }
        let heap_type_str = match self.heap {
            HeapType::Max => "Max",
            HeapType::Min => "Min",
        };
        write!(f, ", Heap type: {}", heap_type_str)?;

        Ok(())
    }
}

impl<T: PartialOrd + Copy> From<(Vec<T>, HeapType)> for Heap<T> {
    fn from((data, heap): (Vec<T>, HeapType)) -> Self {
        let mut heap = Heap { data, heap };

        for idx in (0..=heap.data.len() - 1).rev() {
            heap.heapify(idx)
        }

        heap
    }
}