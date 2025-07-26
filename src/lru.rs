use std::collections::BTreeMap;
use std::collections::HashMap;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::VecDeque;
use std::hash::Hash;


struct Node<T> {
    key:usize,
    value:T,
    prev:Link<T>,
    next:Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;


struct LruCache <T>{
    capacity:usize,
    order:VecDeque<usize>,
    cache:HashMap<usize, T>,
}

impl <T> LruCache <T> 
where T: Hash
{
    fn new(capacity:usize) -> Self {
        Self {
            capacity,
            order:VecDeque::with_capacity(capacity),
            cache:HashMap::new(),
        }
    }
    fn _refresh_(&mut self, key:usize) {
        if let Some(pos) = self.order.iter().position(|&k| k == key) {
            self.order.remove(pos);
        }
        self.order.push_back(key);
    }
    fn get(&mut self, key:usize) -> Option<&T> {
        if self.cache.contains_key(&key) {
            self._refresh_(key);
            Some(&self.cache[&key])
        } else {
            None
        }
    }
    fn put(&mut self, key:usize, value:T) {
        if self.cache.contains_key(&key) {
            self.cache.insert(key, value);
            self._refresh_(key);
        } else {
            if self.order.len() == self.capacity {
                if let Some(stale) = self.order.pop_front() {
                    self.cache.remove(&stale);
                }
            }
            self.cache.insert(key, value);
            self.order.push_back(key);
        }
    }
}
