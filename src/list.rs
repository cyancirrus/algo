use std::mem;
type Link<T> = Option<Box<Node<T>>>;

#[derive(Debug)]
pub struct List<T> {
    head:Link<T>,
}
pub struct IntoIter<T>(List<T>);

pub trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}

impl<T> List<T> {
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}


#[allow(dead_code)]
struct Node<T>{
    val: T,
    next: Link<T>,
}

#[allow(dead_code)]
impl <T>List<T> {
    pub fn new() -> Self {
        Self { head: None }
    }

    pub fn push(&mut self, val:T) {
        let new_node = Box::new( 
            Node { val: val, next: self.head.take() }
        );
        self.head = Some(new_node);
    }
    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.val
        })
    }
    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.val)
    }
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| {&mut node.val})

    }
}

impl <T> Drop for List <T> {
    fn drop(&mut self) {
        let mut cur_link = mem::replace(&mut self.head, None);
        while let Some(mut boxed_node) = cur_link {
            cur_link = mem::replace(&mut boxed_node.next, None);
        }
    }
}

