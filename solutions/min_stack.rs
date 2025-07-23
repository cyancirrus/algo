use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::hash::Hash;
use std::cmp::Ordering;

#[derive(Eq, PartialEq)]
struct MinNode <T> {
    val:T,
}

impl <T> Ord for MinNode <T> 
where T:Eq + PartialEq + Ord
{
    fn cmp(&self, other:&Self) -> Ordering {
        other.val.cmp(&self.val)
    }
}

impl <T> PartialOrd for MinNode<T> 
where T:Eq + PartialEq + Ord
{
    fn partial_cmp(&self, other:&Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct MinStack <T> {
    min:BinaryHeap<MinNode<T>>,
    stack:Vec<T>,
    discarded:HashMap<T, usize>,
}

impl <T> MinStack <T> 
where T: Hash + Eq + PartialEq + Ord + Copy
{
    fn new() -> Self {
        Self {
            min:BinaryHeap::new(),
            stack:Vec::new(),
            discarded:HashMap::new(),
        }
    }
    fn top(&self) -> Option<&T> {
        Some(self.stack.last()?)
    }
    fn pop(&mut self) -> Option<T> {
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
    fn push(&mut self, val:T) {
        self.stack.push(val);
        self.min.push(MinNode{val});
    }
    fn get_min(&mut self) -> Option<T> {
        while let Some(h) = self.min.peek() {
            if let Some(d) = self.discarded.get_mut(&h.val) {
                *d -= 1;
                if *d == 0 {
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
