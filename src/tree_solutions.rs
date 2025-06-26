use crate::tree::{Tree, Link, Node};
use std::fmt::{Display, Debug};
use std::collections::VecDeque;
use std::mem;

impl <T> Tree <T> 
where T:Eq + PartialEq,
{
    fn equal(base:&Link<T>, other:&Link<T>) -> bool {
        match (base, other) {
            (None, None) => true,
            (Some(b), Some(o)) => {
                b.elem == o.elem
                    && Self::equal(&b.left, &o.left)
                    && Self::equal(&b.right, &o.right)
            }
            (_,_) => false,
        }
    }
}

impl<T> PartialEq for Tree<T> 
where T:Eq + PartialEq
{
    fn eq(&self, other:&Tree<T>) -> bool{
        Self::equal(&self.root, &other.root)
    }
}
impl <T> Tree <T>
where T:Debug + Display + Copy,
{


    pub fn zigzag(&self) {
        let mut layer = VecDeque::new();
        let mut next = VecDeque::new();
        let mut odd = true;
        if let Some(root) = &self.root {
            layer.push_back(root);
        }
        while !layer.is_empty() {
            while let Some(node) = match odd {
                true => layer.pop_front(),
                false => layer.pop_back(),
            } {
                println!("{}", node.elem);
                if let Some(left) = &node.left {
                    next.push_back(left);
                }
                if let Some(right) = &node.right {
                    next.push_back(right);
                }
            }
            mem::swap(&mut layer, &mut next);
            odd = !odd;
        }
    }
}
