use std::collections::LinkedList;
use std::mem;
// In going forward reverse
// 1,2,3,4,5
// 2,1,3,4,5
// 2,3,1,4,5
// 3,2,1,4,5
// Reverse going back down

fn reverse_k_nodes(i:usize, mut l: LinkedList<u16>) -> LinkedList<u16> {
    let mut vals = Vec::with_capacity(i);
    let mut reversed = LinkedList::new();
    for _ in 0..i {
        if let Some(v) = l.pop_front() {
            vals.push(v);

        }
    }
    for s in vals.iter().rev() {
        reversed.push_back(*s);
    }
    reversed.extend(l);
    reversed
    
}

fn reverse_k(i:usize, l:&mut [u16]) -> &[u16] {
    for i in 0..i-1 {
        l.swap(i, i+1);
    }
    for i in (0..i-2).rev() {
        l.swap(i, i+1);
    }
    l
}


fn main() {
    assert_eq!(LinkedList::from([2,1,3,4,5]), reverse_k_nodes(2, LinkedList::from([1,2,3,4,5])));
    assert_eq!(LinkedList::from([3,2,1,4,5]), reverse_k_nodes(3, LinkedList::from([1,2,3,4,5])));
    assert_eq!([2,1,3,4,5], reverse_k(2, &mut [1,2,3,4,5]));
    assert_eq!([3,2,1,4,5], reverse_k(3, &mut [1,2,3,4,5]));
}
