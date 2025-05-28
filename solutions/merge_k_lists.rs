use std::collections::LinkedList;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Eq, PartialEq)]
struct HeapEntry {
    val: u16,
    idx: usize,
}

impl Ord for HeapEntry {
    fn cmp(&self, other: &Self) -> Ordering {
        // Other first to make min heap as default is max heap
        other.val.cmp(&self.val)
    }
}

impl PartialOrd for HeapEntry {
    fn partial_cmp(&self, other:&Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}


fn merge_k_sorted_lists(lists:&mut Vec<LinkedList<u16>>) -> LinkedList<u16> {
    let mut heap = BinaryHeap::new();
    let mut merged = LinkedList::new();
    // Initialize the heap
    for idx in 0..lists.len() {
        if let Some(val) = lists[idx].pop_front() {
            heap.push(HeapEntry {val, idx }) 
        }
    }
    while let Some(m) = heap.pop() {
        merged.push_back(m.val);
        if let Some(val) = lists[m.idx].pop_front() {
            heap.push( HeapEntry {val, idx:m.idx } );
        }
    }
    merged
}


fn main() {
    assert_eq!(LinkedList::from([]), merge_k_sorted_lists(&mut vec![LinkedList::from([])]));
    assert_eq!(LinkedList::from([]), merge_k_sorted_lists(&mut vec![]));
    assert_eq!(LinkedList::from([1,1,2,3,4,4,5,6]), merge_k_sorted_lists(&mut vec![LinkedList::from([1,4,5]),LinkedList::from([1,3,4]),LinkedList::from([2,6])]));
}
