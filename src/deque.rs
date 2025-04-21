use std::ptr::NonNull;
use std::marker::PhantomData;

pub struct LinkedList<T> {
    front: Link<T>,
    back: Link<T>,
    len: usize,
    _boo: PhantomData<T>,
}

pub struct Iter<'a, T> {
    front: Link<T>,
    back: Link<T>,
    len: usize,
    _boo: PhantomData<&'a T>
}

type Link<T> = Option<NonNull<Node<T>>>;

struct Node<T> {
    front: Link<T>,
    back: Link<T>,
    elem: T,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            front: None,
            back: None,
            len: 0,
            _boo: PhantomData,
        }
    }
}

impl <T> LinkedList <T> {
    pub fn iter(&self) -> Iter<T> {

    }
}


impl <T> LinkedList<T>{
    pub fn push_front(&mut self, elem:T) {
        unsafe {
            let new = NonNull::new_unchecked(Box::into_raw(Box::new(Node {
                front: None,
                back: None,
                elem, 
            })));
            if let Some(old) = self.front {
                (*old.as_ptr()).front = Some(new);
                (*new.as_ptr()).back = Some(old);
            } else {
                self.back = Some(new);
            }
            self.front = Some(new);
            self.len += 1;
        }
    }
    pub fn pop_front(&mut self) -> Option<T> {
        unsafe {
            self.front.map(|node| {
                let boxed_node = Box::from_raw(node.as_ptr());
                let result = boxed_node.elem;

                self.front = boxed_node.back;
                if let Some(new) = self.front {
                    (*new.as_ptr()).front = None;
                } else {
                    self.back = None;
                }

                self.len -= 1;
                result
            })
        }
    }
    pub fn len(&self) -> usize {
        self.len
    }

    pub fn front(&self) -> Option<&T> {
        unsafe {
            self.front.map(|node| {
                & (*node.as_ptr()).elem
            })
        }
    }
    pub fn front_mut(&mut self) -> Option<&mut T> {
        unsafe {
            self.front.map(|node| {
                &mut (*node.as_ptr()).elem
            })
        }
    }
    pub fn back(&self) -> Option<&T> {
        unsafe {
            self.back.map(|node| {
                &(*node.as_ptr()).elem
            })
        }
    }
    pub fn back_mut(&mut self) -> Option<&mut T> {
        unsafe {
            self.back.map(|node| {
                &mut (*(node.as_ptr())).elem
            })
        }
    }
}

impl <T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        while let Some(_) = self.pop_front(){}
    }
}
