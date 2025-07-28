//🔹 1. Numerical Stability & Floating Point
    // IEEE-754, error propagation, Kahan summation
    // Matrix condition numbers, stability of ops like QR/SVD
    // When to use fixed-point or arbitrary precision
    // Why it matters: You’re building tools that manipulate math. Understanding rounding error and representational quirks separates black-box ML from real tool-building.
// 🔹 4. Compilers / DSLs / Codegen
// 🔹 5. Control Theory & Signal Processing
use std::ptr::NonNull;
use std::collections::HashMap;
use std::marker::PhantomData;
use std::fmt::Debug;

type Link <T> = Option<NonNull<Node<T>>>;

struct Node <T> {
    key:usize,
    val:T, 
    prev:Link<T>,
    next:Link<T>,
}

#[derive(Debug)]
struct LinkedList <T> {
    len: usize,
    head: Link<T>,
    tail: Link<T>,
    _ghost: PhantomData<T>,
}

struct LruCache <T> {
    capacity:usize,
    entries: LinkedList<T>,
    position: HashMap<usize, NonNull<Node<T>>>,
    _ghost: PhantomData<T>,
}


impl <T> LruCache <T>
{
    fn new(capacity:usize) -> Self {
        Self {
            capacity,
            entries: LinkedList::new(),
            position: HashMap::new(),
            _ghost: PhantomData,
        }
    }
    fn get(&mut self, key:usize) -> Option<&T>
        where T:Debug
    {
        println!("position appears as {:?}", self.position);
        // println!("entries appears as {:?}", self.entries.head);
        unsafe {
            if let Some(node) = self.position.remove(&key) {
                self.entries.detach_node(node);
                self.entries.append_node(node);
                self.position.insert(key, node);
                Some(&(*node.as_ptr()).val)
            } else {
                None
            }
        }
    }
    fn update(&mut self, key:usize, val:T)
        where T:Debug
    {
        unsafe {
            if let Some(node) = self.position.remove(&key) {
                self.entries.detach_node(node);
                (*node.as_ptr()).next = None;
                (*node.as_ptr()).prev = None;
                (*node.as_ptr()).val = val;
                self.entries.append_node(node);
                self.position.insert(key, node);
            } else {
                if self.entries.len >= self.capacity {
                    println!("hello world");
                    if let Some(node) = self.entries.pop_front() {
                        println!("node popped {:?}", node.key);
                        self.position.remove(&node.key);
                    }
                }
                self.position.insert(
                    key, 
                    self.entries.push_back(key, val)
                );

            }
        }
        println!("position appears as {:?}", self.position);
    }
    fn remove(&mut self, key:usize) {
        unsafe {
            if let Some(node) = self.position.remove(&key) {
                self.entries.detach_node(node);
                self.entries.len -= 1;
                drop(
                    Box::from_raw(node.as_ptr())
                )
            }
        }
    }
}

impl <T> LinkedList <T> {
    fn new() -> Self {
        Self {
            len: 0, 
            head:None,
            tail:None,
            _ghost: PhantomData,
        }
    }
    fn len(&self) -> usize {
        self.len
    }
    fn push_front(&mut self, key:usize, val:T) -> NonNull<Node<T>> {
        unsafe {
            let new = NonNull::new_unchecked(Box::into_raw(Box::new(Node {
                key,
                val,
                prev:None,
                next:None,
            })));
            if let Some(old) = self.head {
                (*new.as_ptr()).next = Some(old);
                (*old.as_ptr()).prev = Some(new);
            } else {
                self.tail = Some(new);
            }
            self.head = Some(new);
            self.len += 1;
            new
        }
    }
    fn append_node(&mut self, node:NonNull<Node<T>>) {
        unsafe {
            println!("node key {:?}", (*node.as_ptr()).key);
            if let Some(old) = self.tail {
                (*old.as_ptr()).next = Some(node);
                (*node.as_ptr()).prev = Some(old);
            } else {
                self.head = Some(node);
            }
            self.tail = Some(node);
        }
    }
    fn detach_node(&mut self, node:NonNull<Node<T>>) {
        unsafe {
            let pnode = (*node.as_ptr()).prev;
            let nnode = (*node.as_ptr()).next;
            match (pnode, nnode) {
                (Some(prev), Some(next)) => {
                    (*prev.as_ptr()).next = Some(next);
                    (*next.as_ptr()).prev = Some(prev);
                },
                (Some(prev), None) => {
                    (*prev.as_ptr()).next = None;
                    self.tail = None;
                },
                (None, Some(next)) => {
                    (*next.as_ptr()).prev = None;
                    self.head = Some(next);
                }
                (None, None) => {
                    self.head = None;
                    self.tail = None;
                }
            }
        }
    }
    fn push_back(&mut self, key:usize, val:T) -> NonNull<Node<T>> {
        unsafe {
            let new = NonNull::new_unchecked(Box::into_raw(Box::new(Node {
                key,
                val,
                prev:None,
                next:None,
            })));
            if let Some(old) = self.tail {
                (*old.as_ptr()).next = Some(new);
                (*new.as_ptr()).prev = Some(old);
            } else {
                self.head = Some(new);
            }
            self.tail = Some(new);
            self.len += 1;
            new
        }
    }
    fn pop_front(&mut self) -> Option<Node<T>> {
        unsafe {
            if let Some(node) = self.head.take() {
                let bnode = Box::from_raw(node.as_ptr());
                self.head = bnode.next;
                if let Some(new) = bnode.next {
                    (*new.as_ptr()).prev = None;
                } else {
                    self.tail = None;
                }
                self.len -= 1;
                Some(*bnode)
            } else {
                None
            }
        }
    }
    fn pop_back(&mut self) -> Option<Node<T>> {
        unsafe {
            if let Some(node) = self.tail.take() {
                let bnode = Box::from_raw(node.as_ptr());
                self.tail = bnode.prev;
                if let Some(new) = bnode.prev {
                    (*new.as_ptr()).next = None;
                } else {
                    self.head = None;
                }
                self.len -= 1;
                Some(*bnode)
            } else {
                None
            }
        }
    }
}

impl <T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        while let Some(_) = self.pop_back() {
        }
    }
}


fn main() {
    let mut cache = LruCache::new(3);
    cache.update(1, "a");
    cache.update(2, "b");
    cache.update(3, "c");
    assert_eq!(cache.get(1), Some(&"a"));
    assert_eq!(cache.get(2), Some(&"b"));
    assert_eq!(cache.get(3), Some(&"c"));
    cache.update(2, "bb"); // Should move 2 to MRU
    assert_eq!(cache.get(2), Some(&"bb"));
    cache.remove(2);
    assert_eq!(cache.get(2), None);
    println!("---------");
    cache.update(4, "d");
    assert_eq!(cache.get(4), Some(&"d"));
    println!("-----------");
    cache.update(5, "e"); // Should evict LRU (which is 1)
    assert_eq!(cache.get(1), None);         // evicted
    // assert_eq!(cache.get(3), Some(&"c"));   // still in
    // assert_eq!(cache.get(5), Some(&"e"));   // just added
}
