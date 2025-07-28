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

pub struct LruCache <T> {
    capacity:usize,
    entries: LinkedList<T>,
    position: HashMap<usize, NonNull<Node<T>>>,
    _ghost: PhantomData<T>,
}

impl <T> LruCache <T>
{
    pub fn new(capacity:usize) -> Self {
        Self {
            capacity,
            entries: LinkedList::new(),
            position: HashMap::new(),
            _ghost: PhantomData,
        }
    }
    pub fn get(&mut self, key:usize) -> Option<&T> {
        unsafe {
            let node = self.position.get(&key)?;
            self.entries.detach_node(*node);
            self.entries.push_back_node(*node);
            Some(&(*node.as_ptr()).val)
        }
    }
    pub fn update(&mut self, key:usize, val:T) {
        unsafe {
            if let Some(node) = self.position.get(&key) {
                self.entries.detach_node(*node);
                (*node.as_ptr()).val = val;
                self.entries.push_back_node(*node);
            } else {
                if self.entries.len >= self.capacity {
                    if let Some(node) = self.entries.pop_front() {
                        self.position.remove(&node.key);
                    }
                }
                self.position.insert(
                    key, 
                    self.entries.push_back(key, val)
                );

            }
        }
    }
    pub fn remove(&mut self, key:usize) {
        unsafe {
            if let Some(node) = self.position.remove(&key) {
                self.entries.detach_node(node);
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
    fn push_back_node(&mut self, node:NonNull<Node<T>>) {
        unsafe {
            if let Some(old) = self.tail {
                (*old.as_ptr()).next = Some(node);
                (*node.as_ptr()).prev = Some(old);
            } else {
                self.head = Some(node);
            }
            self.tail = Some(node);
            self.len += 1;
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
                    self.tail = Some(prev);
                },
                (None, Some(next)) => {
                    (*next.as_ptr()).prev = None;
                    self.head = Some(next);
                }
                (None, None) => {
                    self.head = None;
                    self.tail = None;
                }
            };
            (*node.as_ptr()).next = None;
            (*node.as_ptr()).prev = None;
            self.len -= 1;
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
            let node = self.head.take()?;
            let bnode = Box::from_raw(node.as_ptr());
            self.head = bnode.next;
            if let Some(new) = bnode.next {
                (*new.as_ptr()).prev = None;
            } else {
                self.tail = None;
            }
            self.len -= 1;
            Some(*bnode)
        }
    }
    fn pop_back(&mut self) -> Option<Node<T>> {
        unsafe {
            let node = self.tail.take()?;
            let bnode = Box::from_raw(node.as_ptr());
            self.tail = bnode.prev;
            if let Some(new) = bnode.prev {
                (*new.as_ptr()).next = None;
            } else {
                self.head = None;
            }
            self.len -= 1;
            Some(*bnode)
        }
    }
}

impl <T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        while let Some(_) = self.pop_back() {
        }
    }
}
