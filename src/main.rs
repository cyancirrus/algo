#![allow(dead_code)]
use std::collections::HashMap;
use std::collections::VecDeque;
use std::mem::MaybeUninit;

struct RingBuffer<T> {
    data:Vec<MaybeUninit<T>>,
    head:usize,
    tail:usize,
    len:usize,
    capacity:usize,

}

impl<T> RingBuffer<T> {
    fn new(k:usize) -> Self {
        let mut data = Vec::with_capacity(k);
        data.resize_with(k, MaybeUninit::uninit);
        Self {
            data,
            head:0,
            tail:0,
            len:0,
            capacity:k,
        }
    }
    fn push_front(&mut self, value:T) {
        if self.len == self.capacity {
            self.head = (self.head + self.capacity - 1) % self.capacity;
            unsafe {
                self.data[self.head].assume_init_drop();
            }
            self.data[self.head] = MaybeUninit::new(value);
            self.tail = self.head;
        } else {
            self.head = (self.head + self.capacity - 1) % self.capacity;
            self.data[self.head] = MaybeUninit::new(value);
            self.len += 1;
        }
    }
    fn push_back(&mut self, value:T) {
        if self.len == self.capacity {
            unsafe {
                self.data[self.tail].assume_init_drop();
            }
            self.data[ self.tail ] = MaybeUninit::new(value);
            self.tail = (self.tail + 1) % self.capacity;
            self.head = self.tail;
        } else {
            self.data[ self.tail ] = MaybeUninit::new(value);
            self.tail = (self.tail + 1) % self.capacity;
            self.len += 1;
        }
    }
    fn pop_front(&mut self) -> Option<T> {
        if self.len == 0 {
            return None;
        }
        let idx = self.head;
        self.head = (self.head + 1) % self.capacity;
        self.len -= 1; 
        Some( unsafe {
            self.data[idx].assume_init_read()
        })
    }
    fn pop_tail(&mut self) -> Option<T>{
        if self.len == 0 {
            return None;
        }
        self.tail = (self.tail + self.capacity - 1) % self.capacity;
        self.len -= 1; 
        Some( unsafe {
            self.data[self.tail].assume_init_read()
        })
    }
}

impl <T> RingBuffer<T> {
    fn iter<'a>(&'a self) -> Iter<'a, T> {
        Iter::new(&self)
    }
}

impl <T> Drop for RingBuffer<T> {
    fn drop(&mut self) {
        let mut idx = self.head;
        for _ in 0..self.len {
            unsafe {
                self.data[idx].assume_init_drop();
            }
            idx = (idx + 1) % self.capacity;
        }
    }
}

pub struct Iter<'a, T> {
    curs:usize,
    to_yield:usize,
    buff:&'a RingBuffer<T>,
}

impl <'a, T> Iter<'a, T> {
    fn new(ring:&'a RingBuffer<T>) -> Self {
        Self {
            curs:ring.head,
            to_yield: ring.len,
            buff:ring,
        }
    }
}

impl <'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.to_yield == 0 {
            return None;
        }
        self.to_yield -= 1;
        let v = unsafe {
            &*self.buff.data[self.curs].as_ptr()
        };
        self.curs = (self.curs + 1) % self.buff.capacity;
        Some(v)
    }
}

fn main() {
    let mut test = RingBuffer::new(5);
    for i in 0..5 {
        test.push_back(i);
    }
    test.push_back(5);
    test.push_front(11);
    test.pop_tail();
    test.push_back(100);
    test.push_back(100);
    test.push_front(42);
    let c = test.iter();

    for c in c {
        println!("Value {c:?}");
    }
}
