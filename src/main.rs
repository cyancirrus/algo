use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::hash::Hash;
use std::cmp::Ordering;


struct MinStack <T> {
    stack:Vec<T>,
    min:Vec<T>,
}

impl <T> MinStack <T> 
where T: Eq + PartialEq + Ord + Copy
{
    fn new() -> Self {
        Self {
            stack:Vec::new(),
            min:Vec::new(),
        }
    }
    fn top(&self) -> Option<&T> {
        self.stack.last()
    }
    fn pop(&mut self) -> Option<T> {
        if let Some(v) = self.stack.pop() {
            if let Some(h) = self.min.last()  {
                if *h == v {
                    self.min.pop();
                }
            }
            Some(v)
        } else {
            None
        }
    }
    fn push(&mut self, val:T) {
        self.stack.push(val);
        if self.min.is_empty() || val <= *self.min.last().unwrap() {
            self.min.push(val);
        }
    }
    fn get_min(&mut self) -> Option<&T> {
        self.min.last()
    }
}


fn main() {
}
