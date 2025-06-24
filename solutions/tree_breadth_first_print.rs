use crate::tree::{Tree, Link};
use std::fmt::{Display, Debug};
use std::collections::VecDeque;

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

    pub fn pbreadth(&self) {
        let mut queue = VecDeque::new();
        if let Some(root) = &self.root {
            queue.push_back(root);
        }
        while let Some(node) = queue.pop_front() {
            println!("{}", node.elem);

            if let Some(left) = &node.left {
                queue.push_back(left);
            }
            if let Some(right) = &node.right {
                queue.push_back(right);
            }
        }
    }
    // tried using a heap based structure but required metadata and bad for large trees
    // fn pbreadth(&self, size:usize) {
    //     let mut result = vec![None;size];
    //     Self::_pbreadth_(&self.root, 0, &mut result);
    //     println!("Data appears as {:?}", result);
    // }
    // fn _pbreadth_(curr:&Link<T>, index:usize, result:&mut Vec<Option<T>>) {
    //     if let Some(n) = curr {
    //         result[index] = Some(n.elem);
    //         Self::_pbreadth_(&n.left, 2*index + 1, result);
    //         Self::_pbreadth_(&n.right, 2*index + 2, result);

    //     }
    // }
}
