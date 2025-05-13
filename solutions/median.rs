struct FastMedian {
    equal:bool,
    left:MaxHeap,
    right:MinHeap,
}

struct MinHeap {
    elements:usize,
    data:Vec<u8>,
}

struct MaxHeap {
    elements:usize,
    data:Vec<u8>,
}

impl FastMedian {
    fn new() -> Self {
        Self {equal:true, left:MaxHeap::new(), right:MinHeap::new() }
    }

    fn insert(&mut self, element:u8) {
        if let Some(max_left) = self.left.max() {
            // insert into the correct tree
            if element <= max_left {
                self.left.insert(element);
            } else {
                self.right.insert(element);
            }
        } else {
            // handle base case
            self.left.insert(element);
        }
        // rebalance
        let diff = self.left.elements as isize - self.right.elements as isize;
        if diff > 1 {
            let max = self.left.pop().unwrap();
            self.right.insert(max);
        } else if diff < - 1 {
            let min = self.right.pop().unwrap();
            self.left.insert(min);
        }
        self.equal = self.left.elements == self.right.elements;
    }

    fn median(&self) -> Option<u8> {
        if self.equal {
            match (self.left.max(), self.right.min()) {
                (Some(l), Some(r)) => Some((l + r) / 2),
                (_, _) => None,
            }
        } else if self.left.elements > self.right.elements {
            self.left.max()
        } else {
            self.right.min()
        }
    }
}

impl MaxHeap {
    fn new() -> Self {
        Self{elements:0, data:Vec::with_capacity(16)}
    }
    fn heapify(mut data:Vec<u8>) -> Self {
        let elements = data.len();
        let mut heap = Self{elements, data};
        heap._heapify_();
        heap

    }
    fn _heapify_(&mut self) {
        for leaf in (0..self.elements / 2).rev() {
            self._shift_down_(leaf);
        }
    }

    fn max(&self) -> Option<u8> {
        if self.elements == 0 { return None };
        Some(self.data[0])
    }
    fn pop(&mut self) -> Option<u8> {
        if self.elements == 0 {return None };
        let result = self.data.swap_remove(0);
        self.elements -= 1;
        if self.elements > 0 {
            self._shift_down_(0);
        }
        Some(result)
    }
    fn _shift_down_(&mut self, mut index:usize) {
        loop {
            let left = 2 * index + 1;
            let right = 2 * index + 2;
            let mut largest = index;
            if left < self.elements && self.data[largest] < self.data[left] {
                largest = left;
            }
            if right < self.elements && self.data[largest] < self.data[right] {
                largest = right;
            }
            if largest == index {
                break;
            }
            self.data.swap(index, largest);
            index = largest;
        }
    }
    fn insert(&mut self, element:u8) {
        if self.elements == self.data.len() {
            self.data.push(element);
        } else {
            self.data[self.elements] = element;
        }
        self._insert_(self.elements);
        self.elements += 1;
    }
    fn _insert_(&mut self, index:usize) {
        if index == 0 {return; };
        let parent = (index - 1)/2;
        if self.data[index] < self.data[parent] {
            return;
        }
        self.data.swap(index, parent);
        self._insert_(parent);

    }
}

impl MinHeap {
    fn new() -> Self {
        Self{elements:0, data:Vec::with_capacity(16)}
    }
    fn min(&self) -> Option<u8> {
        if self.elements == 0 { return None };
        Some(self.data[0])
    }
    fn heapify(mut data:Vec<u8>) -> Self {
        let elements = data.len();
        let mut heap = Self{elements, data};
        heap._heapify_();
        heap

    }
    fn _heapify_(&mut self) {
        for leaf in (0..self.elements / 2).rev() {
            self._shift_down_(leaf);
        }
    }
    fn pop(&mut self) -> Option<u8> {
        if self.elements == 0 {return None}
        let result = self.data.swap_remove(0);
        self.elements -= 1;
        if self.elements > 0 {
            self._shift_down_(0);
        }
        Some(result)
    }
    fn _shift_down_(&mut self, mut index:usize) {
        loop {
            let left = 2 * index + 1;
            let right = 2 * index + 2;
            let mut smallest = index;
            if left < self.elements && self.data[left] < self.data[smallest] {
                smallest = left;
            }
            if right < self.elements && self.data[right] < self.data[smallest] {
                smallest = right;
            }
            if smallest == index {
                break;
            }
            self.data.swap(index, smallest);
            index = smallest;
        }
    }
    fn insert(&mut self, element:u8) {
        if self.elements == self.data.len() {
            self.data.push(element);
        } else {
            self.data[self.elements] = element;
        }
        self._insert_(self.elements);
        self.elements += 1;
    }
    fn _insert_(&mut self, index:usize) {
        if index == 0 {return; };
        let parent = (index - 1)/2;
        if self.data[index] > self.data[parent] {
            return;
        }
        self.data.swap(index, parent);
        self._insert_(parent);

    }
}

fn heapsort(nums:Vec<u8>) -> Vec<u8> {
    let mut sort = Vec::with_capacity(nums.len());
    let mut min_heap = MinHeap::heapify(nums);
    while min_heap.elements > 0 {
        sort.push(min_heap.pop().unwrap()); 
    }
    sort
}

fn main() {
    let mut fm = FastMedian::new();
    for &num in &[10, 5, 20, 3, 8, 25, 15] {
        fm.insert(num);
        println!("Inserted: {num}, Median: {:?}", fm.median());
    }

    let data = vec![231, 0, 10, 30, 24];
    let sort = heapsort(data);
    println!("Sorted? {:?}", sort);

}
