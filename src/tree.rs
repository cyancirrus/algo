#![allow(warnings)]
use std::fmt::Debug;
use std::mem;
use std::ptr;
use std::collections::VecDeque;

pub type Link<T> = Option<Box<Node<T>>>;

#[derive(Debug, Eq, PartialEq)]
pub struct Node<T> {
    pub elem: T,
    pub left: Link<T>,
    pub right: Link<T>,
}
#[derive(Debug)]
pub struct Tree<T> {
    pub root: Link<T>,
}

pub struct Iter<T> {
    next: Link<T>, 
}

impl <T:PartialOrd>  Tree<T> 
where T: Copy
{
    pub fn new() -> Self {
        Self { root: None }
    }
    pub fn from_vec(mut data:Vec<Option<T>>) -> Self {
        data.reverse();
        let mut tree = Tree::new();
        let mut queue = VecDeque::new();
        queue.push_back(&mut tree.root);
        while let Some(slot) = queue.pop_front(){
            if let Some(datum) = data.pop() {
                if let Some(value) = datum {
                    *slot = Some(Box::new(Node {
                        elem:value,
                        left:None,
                        right:None
                    })); 
                    let node_ref = slot.as_mut().unwrap();
                    queue.push_back(&mut node_ref.left);
                    queue.push_back(&mut node_ref.right);
                }
            }
        }
        tree
    }
    pub fn to_vec(&self) -> Vec<Option<T>> {
        let mut serial= Vec::new();
        let mut queue = VecDeque::new();
        
        if let Some(root) = self.root.as_deref() {
            queue.push_back(Some(root));
        }

        while let Some(node) = queue.pop_front() {
            if let Some(n) = node {
                queue.push_back(n.left.as_deref());
                queue.push_back(n.right.as_deref());
                serial.push(Some(n.elem));
            } else {
                serial.push(None);
            }
        }
        while serial.last() == Some(&None) {
            serial.pop();
        }
        serial
    }
    pub fn insert(&mut self, elem:T) {
        let mut curr_node = &mut self.root;
        while let Some(node) = curr_node {
            if elem < node.elem {
                curr_node = &mut node.left;
            } else {
                curr_node = &mut node.right;
            }
        }
        *curr_node = Some(Box::new(Node {
            elem,
            left: None,
            right:None,
        }));
    }
    pub fn min(&self) -> Option<&T> {
        let mut curr_node = &self.root;
        while let Some(node) = curr_node {
            curr_node = &node.left;
        }
        curr_node.as_ref().map(|node| &node.elem)
    }
    pub fn max(&self) -> Option<&T> {
        let mut curr_node = &self.root;
        while let Some(node) = curr_node {
            curr_node = &node.right;
        }
        curr_node.as_ref().map(|node| &node.elem)
    }
    pub fn swap(&mut self, first:Option<*mut Node<T>>, second:Option<*mut Node<T>>) {
        if let (Some(f), Some(s)) = (first, second) {
            unsafe {
                mem::swap(&mut (*f).elem, &mut (*s).elem)
            }
        }
    }
    pub fn contains(&self, elem:T) -> bool {
        let mut curr_node = &self.root;
        while let Some(node) = curr_node {
            if elem < node.elem {
                curr_node = &node.left;
            } else if elem > node.elem {
                curr_node = &node.right;
            } else {
                return true
            }
        }
        false
    }
    pub fn floor_strict(&self, elem:T) -> Option<&T> {
        let mut val = None;
        let mut curr_node = &self.root;
        while let Some(node) = curr_node {
            if elem < node.elem {
                curr_node = &node.left;
            } else if elem > node.elem {
                val = Some(&node.elem);
                curr_node = &node.right;
            } else {
                // find the minimimum closest value even if we match
                curr_node = &node.left;
            }
        }
        val
    }
    pub fn ceiling_strict(&self, elem:T) -> Option<&T> {
        let mut val = None;
        let mut curr_node = &self.root;
        while let Some(node) = curr_node {
            if elem < node.elem {
                val = Some(&node.elem);
                curr_node = &node.left;
            } else if elem > node.elem {
                curr_node = &node.right;
            } else {
                // find the minimimum closest value even if we match
                curr_node = &node.right;
            }
        }
        val
    }
}

impl<T> Tree<T> 
where T:Copy
{
    pub fn in_order(&self) -> Vec<T> {
        let mut data = vec![];
        self._in_order_(&self.root, &mut data);
        data
    }
    fn _in_order_(&self, node:&Link<T>, data:&mut Vec<T>) {
        if let Some(node) = node {
            self._in_order_(&node.left, data);
            data.push(node.elem);
            self._in_order_(&node.right, data);
        }

    }
}

impl<T> Tree<T> {
    pub fn into_iter(self) -> Iter<T> {
        Iter{ next:self.root }
    }
    pub fn iter(&mut self) -> InOrderIter<'_, T> {
        InOrderIter::new(self.root.as_deref())
    }

}
impl <T:Debug> Tree<T> {    
    pub fn display(&self) {
            self._display(&self.root);
    }
    fn _display(&self, curr_node:&Link<T>) {
        if let Some(node) = curr_node {
            self._display(&node.left);
            println!("Node elem {:?}", node.elem);
            self._display(&node.right);
        }
    }
}

pub struct InOrderIter<'a, T> {
    stack: Vec<&'a Node<T>>,
    current: Option<&'a Node<T>>,
}

impl <'a, T> InOrderIter<'a,T> 
{
    pub fn new(root: Option<&'a Node<T>>) -> Self {
        InOrderIter {
            stack: Vec::new(),
            current: root,
        }
    }
}

impl <'a, T> Iterator for InOrderIter<'a, T> 
where T:Copy + std::ops::Add,
{
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        while let Some(node) = self.current {
            self.stack.push(&node);
            self.current = node.left.as_deref();
        }
        let node = self.stack.pop()?;
        self.current = node.right.as_deref();
        Some(&node.elem)
    }
}

pub struct LevelOrderIter <'a, T> {
    stack: VecDeque<&'a Node<T>>,
}

impl <'a, T> LevelOrderIter <'a, T> {
    pub fn new(root:Option<&'a Node<T>>) -> Self {
        let mut stack = VecDeque::new();
        if let Some(root) = root {
            stack.push_back(root);
        }
        LevelOrderIter {
            stack,
        }
    }
}
impl <'a, T> Iterator for LevelOrderIter <'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(node) = self.stack.pop_front() {
            if let Some(left) = node.left.as_deref() {
                self.stack.push_back(&left);
            }
            if let Some(right) = node.right.as_deref() {
                self.stack.push_back(&right);
            }
            return Some(&node.elem)
        }
        None
    }
}

pub struct PreOrderIter<'a, T> {
    stack: Vec<&'a Node<T>>,
    current: Option<&'a Node<T>>,
}

impl <'a, T> PreOrderIter <'a, T> {
    pub fn new(root:Option<&'a Node<T>>) -> Self {
        Self {
            stack:Vec::new(),
            current: root,
        }
    }
}

impl <'a, T> Iterator for PreOrderIter<'a,T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        let mut node = self.current;
        loop {
            if let Some(existence) = node {
                self.current = existence.left.as_deref();
                self.stack.push(existence);
                return Some(&existence.elem)
            } 
            if let Some(existence) = self.stack.pop() {
                node = existence.right.as_deref();
            } else {
                return None
            }
        }
    }
}

pub struct PostOrderIter <'a, T> {
    stack: Vec<&'a Node<T>>,
    curr: Option<&'a Node<T>>,
    prev: Option<&'a Node<T>>,
}

impl <'a,T> PostOrderIter <'a, T> {
    pub fn new(root:Option<&'a Node<T>>) -> Self {
        Self {
            stack:Vec::new(),
            curr:root,
            prev:None,
        }
    }
}

impl <'a, T> Iterator for PostOrderIter <'a, T> 
{
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(node) = self.curr {
                // Process the left branch
                self.stack.push(&node);
                self.curr = node.left.as_deref();
                continue;
            }
            if let Some(curr) = self.stack.pop() {
                if {
                    // Safety check prior to unwrap
                    self.prev.is_none() || curr.right.is_none()
                    || ptr::eq(self.prev.unwrap(), curr.right.as_deref().unwrap()) 
                }{
                    // Right branch either does not exist or we have already visited
                    self.prev = Some(curr);
                    self.curr = curr.right.as_deref();
                    return Some(&curr.elem);
                }
                if let Some(right) = curr.right.as_deref() {
                    // Add the right sub-branch to iterator
                    self.stack.push(&curr);
                    self.stack.push(&right);
                    self.curr = Some(&right);
                    continue;
                }
            }
            return None;
        }
    }
}
