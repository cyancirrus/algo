use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::hash::Hash;
use std::cmp::Ordering;


struct MinMaxStack <T> {
    stack:Vec<T>,
    min:Vec<T>,
    max:Vec<T>,
}

impl <T> MinMaxStack <T> 
where T: Eq + PartialEq + Ord + Copy
{
    fn new() -> Self {
        Self {
            stack:Vec::new(),
            min:Vec::new(),
            max:Vec::new(),
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
            if let Some(h) = self.max.last()  {
                if *h == v {
                    self.max.pop();
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
        if self.max.is_empty() || val >= *self.max.last().unwrap() {
            self.max.push(val);
        }
    }
    fn get_min(&mut self) -> Option<&T> {
        self.min.last()
    }
    fn get_max(&mut self) -> Option<&T> {
        self.max.last()
    }
}


fn main() {
}
