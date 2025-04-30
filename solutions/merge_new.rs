use std::collections::LinkedList;

fn merge<T:PartialOrd>(mut base:LinkedList<T>, mut other:LinkedList<T>) -> LinkedList<T> {
    let mut new = LinkedList::new();
    while !base.is_empty() && !other.is_empty() {
        if base.front() <= other.front() {
            new.push_back(base.pop_front().unwrap());
        } else {
            new.push_back(other.pop_front().unwrap());
        }
    }
    new.append(&mut base);
    new.append(&mut other);
    new
}


fn main() {
    let mut l1 = LinkedList::from([2,2,4]);
    let mut l2 = LinkedList::from([1,3,4]);
    println!("Result {:?}", merge(l1, l2));
}
