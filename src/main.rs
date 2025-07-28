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

type Link <T> = Option<NonNull<Node<T>>>;


struct Node <T> {
    key:usize,
    val:T, 
    prev:Link<T>,
    next:Link<T>,
}

struct LinkedList <T> {
    len: usize,
    head: Link<T>,
    tail: Link<T>,
    _ghost: PhantomData<T>,
}

struct MapCursor<T> {
    // key -> pointer in LinkedList
    capacity:usize,
    list: LinkedList<T>,
    position: HashMap<usize, NonNull<Node<T>>>,
    _ghost: PhantomData<T>,
}

impl <T> MapCursor <T> {
    fn new() -> Self {
        Self {
            capacity:1024,
            list: LinkedList::new(),
            position: HashMap::new(),
            _ghost: PhantomData,
        }
    }
    fn new_from_list(&self, list:LinkedList<T>) -> Self {
        Self {
            capacity:1024,
            list,
            position: HashMap::new(),
            _ghost: PhantomData, 
        }
    }
    fn _stitch_(&mut self, key:usize) -> Option<NonNull<Node<T>>> {
        unsafe {
            if let Some(node) = self.position.get(&key) {
                let pnode = (*node.as_ptr()).prev;
                let nnode = (*node.as_ptr()).next;
                match (pnode, nnode) {
                    (Some(prev), Some(next)) => {
                        (*prev.as_ptr()).next = Some(next);
                        (*next.as_ptr()).prev = Some(prev);
                    },
                    (Some(prev), None) => {
                        (*prev.as_ptr()).next = None;
                        self.list.tail = None;
                    },
                    (None, Some(next)) => {
                        (*next.as_ptr()).prev = None;
                        self.list.head = Some(next);
                    }
                    (None, None) => {
                        self.list.head = None;
                        self.list.tail = None;
                    }
                };
                return Some(*node)
            }
        }
        None
    }
    fn update(&mut self, key:usize, val:T) {
        unsafe {
            if let Some(node) = self._stitch_(key) {
                (*node.as_ptr()).next = None;
                (*node.as_ptr()).prev = None;
                self.list.append(node);
                self.position.insert(key, node);
            } else if self.list.len < self.capacity {
                self.position.insert(
                    key, 
                    self.list.push_back(key, val)
                );
            }
        }
    }
    fn remove(&mut self, key:usize) {
        unsafe {
            if let Some(node) = self._stitch_(key) {
                self.position.remove(&key);
                self.list.len -= 1;
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
    fn push_front(&mut self, key:usize, val:T) {
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
        }
    }
    fn append(&mut self, node:NonNull<Node<T>>) {
        unsafe {
            if let Some(old) = self.tail {
                (*old.as_ptr()).next = Some(node);
                (*node.as_ptr()).prev = Some(old);
            } else {
                self.head = Some(node);
            }
            self.tail = Some(node);
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
    fn pop_front(&mut self) -> Option<T> {
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
                Some(bnode.val)
            } else {
                None
            }
        }
    }
    fn pop_back(&mut self) -> Option<T> {
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
                Some(bnode.val)

            } else {
                None
            }
        }
    }
    // fn pop_back(&mut self) -> Option<T> {
    //     unsafe {
    //         self.tail.map(|node| {
    //             // Ensure memory is freed using box
    //             let bnode = Box::from_raw(node.as_ptr());
    //             let result = bnode.val;
    //             self.tail = bnode.prev;
    //             if let Some(new) = self.tail {
    //                 (*new.as_ptr()).next = None
    //             } else {
    //                 self.head = None;
    //             }
    //             self.len -= 1;
    //             result   
    //         })
    //     }
    // }
    // fn pop_front(&mut self) -> Option<T> {
    //     unsafe {
    //         self.head.map(|node| {
    //             // Ensure memory is freed using box
    //             let bnode = Box::from_raw(node.as_ptr());
    //             let result = bnode.val;
    //             self.head = bnode.next;
    //             if let Some(new) = bnode.next {
    //                 (*new.as_ptr()).prev = None;
    //             } else {
    //                 self.tail = None;
    //             }
    //             self.len -= 1;
    //             result
    //         })
    //     }
}

impl <T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        while let Some(_) = self.pop_back() {
        }
    }
}


fn main() {
    let mut abc = vec![0,1,2];
    abc[0]=100;
}
