#[allow(unused)]

pub struct List <T> {
    head: Link<T>,
}

pub trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}

type Link<T>= Option<Box<Node<T>>>;

struct Node<T> {
    elem:T,
    next:Link<T>,
}

pub struct IntoIter<T> {
    list:List<T>,
}

pub struct Iter<'a, T> {
    next:Option<&'a Node<T>>,
}

pub struct IterMut<'a, T> {
    next:Option<&'a mut Node<T>>,
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.list.pop()
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_deref();
            &node.elem
        })
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_deref_mut();
            &mut node.elem
        })
    }
}


impl<T> List<T> {
    pub fn new() -> Self {
        List {
            head: None,
        }
    }
    pub fn push(&mut self, elem:T) {
        let new = Some(Box::new(Node {
            elem,
            next:self.head.take(),
        }));
        self.head = new;
    }
    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter{list: self}
    }
    pub fn iter(&self) -> Iter<T> {
        Iter{next: self.head.as_deref()} 
    }
    pub fn iter_mut(&mut self) -> IterMut<T> {
        IterMut{next: self.head.as_deref_mut()}
    }
    pub fn peak(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem)
    }
    pub fn peak_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.elem)
    }
}

impl <T> Drop for List<T> {
    fn drop(&mut self) {
        while let Some(_) = self.pop() { }
    }
}
