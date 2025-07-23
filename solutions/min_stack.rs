use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::cmp::Ordering;

#[derive(Eq, PartialEq)]
struct MinNode {
    val:i8,
}

impl Ord for MinNode {
    fn cmp(&self, other:&Self) -> Ordering {
        other.val.cmp(&self.val)
    }
}

impl PartialOrd for MinNode {
    fn partial_cmp(&self, other:&Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct MinStack {
    min:BinaryHeap<MinNode>,
    stack:Vec<i8>,
    discarded:HashMap<i8, usize>,
}

impl MinStack {
    fn new() -> Self {
        Self {
            min:BinaryHeap::new(),
            stack:Vec::new(),
            discarded:HashMap::new(),
        }
    }
    fn top(&self) -> Option<&i8> {
        if self.stack.len() > 0 {
            return Some(self.stack.last()?)
        } else {
            None
        }
    }
    fn pop(&mut self) -> Option<i8> {
        if let Some(v) = self.stack.pop() {
            if let Some(h) = self.min.peek()  {
                if h.val == v {
                    self.min.pop();
                } else {
                    *self.discarded.entry(v).or_default()+=1;
                }
            }
            Some(v)
        } else {
            None
        }
    }
    fn push(&mut self, val:i8) {
        self.stack.push(val);
        self.min.push(MinNode{val});
    }
    fn get_min(&mut self) -> Option<i8> {
        while let Some(h) = self.min.peek() {
            if let Some(d) = self.discarded.get_mut(&h.val) {
                if *d > 0 {
                    *d -= 1;
                } else {
                    self.discarded.remove(&h.val);
                }
                self.min.pop();
            } else {
                return Some(h.val)
            }
        }
        None
    }
}


fn main() {
}
