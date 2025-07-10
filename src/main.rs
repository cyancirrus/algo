#![allow(dead_code)]
use std::mem;

#[derive(Debug, Clone)]
pub struct Node<T> {
    elem:T,
    left:Link<T>,
    right:Link<T>,
    multiplicity:usize,
    nodes:usize,
}

type Link<T> = Option<Box<Node<T>>>;

#[derive(Debug)]
pub struct OsTree<T> {
    pub root: Link<T>,
}


impl <T> OsTree<T> 
where T: Ord + PartialOrd

{
    fn new() -> Self {
        Self {
            root:None,
        }
    }
    fn insert(&mut self, elem:T) {
        let mut link = &mut self.root;
        while let Some(node) = link {
            node.nodes += 1;
            if elem < node.elem {
                link = &mut node.left;
            } else if node.elem < elem {
                link = &mut node.right;
            } else {
                node.multiplicity += 1;
                return;
            }
        }
        *link = Some(Box::new(Node {
            elem,
            left:None,
            right:None,
            multiplicity:1,
            nodes:1,
        }))
    }

    fn len(&self) -> usize {
        if let Some(node) = &self.root {
            node.nodes
        } else {
            0
        }
    }
    fn rank(&self, elem:T) -> usize {
        let mut rank: usize = 0;
        let mut link = &self.root;
        while let Some(node) = link {
            if elem < node.elem {
                link = &node.left;
            } else if node.elem < elem {
                rank += node.left.as_ref().map_or(0, |l| l.nodes);
                rank += node.multiplicity;
                link = &node.right;
            } else {
                if let Some(left) = &node.left {
                    rank += left.nodes;
                }
                break;
            }
        }
        rank
    }
    fn select(&self, mut idx:usize) -> Option<&T> {
        let mut link = &self.root;
        while let Some(node) = link {
            let left = node.left.as_ref().map_or(0, |l| l.nodes);
            if idx < left {
                link = &node.left;
            } else if idx >= left + node.multiplicity {
                idx -= left + node.multiplicity;
                link = &node.right;
            } else {
                return Some(&node.elem);
            }
        }
        None
    }

    fn remove(&mut self, elem:T) 
    where T:Clone
    {
        if let Some(e) =  Self::_remove_(&mut self.root, elem) {
            Self::_decrement_(&mut self.root, e);
        }
    }
    fn _decrement_(mut link: &mut Link<T>, elem:T) {
        while let Some(node) = link {
            node.nodes -= 1;
            if elem < node.elem {
                link = &mut node.left;
            } else if node.elem < elem {
                link = &mut node.right;
            } else {
                break;
            }
        }
    }
    fn _remove_(link: &mut Link<T>, elem: T) -> Option<T> 
    where T: Clone
    {
        let mut bnode = match link.take() {
            Some(node) => node,
            None => return None,
        };
        if elem < bnode.elem {
            let removed = Self::_remove_(&mut bnode.left, elem);
            *link = Some(bnode);
            return removed;
        } else if bnode.elem < elem {
            let removed = Self::_remove_(&mut bnode.right, elem);
            *link = Some(bnode);
            return removed;
        }

        if bnode.multiplicity > 1 {
            bnode.multiplicity -= 1;
            *link = Some(bnode);
            return Some(elem);
        }

        *link = match (bnode.left.take(), bnode.right.take()) {
            (None, None) => None,
            (Some(left), None) => Some(left),
            (None, Some(right)) => Some(right),
            (Some(left), Some(mut right)) => {
                let mut min_node = &mut right;
                while let Some(ref mut l) = min_node.left {
                    min_node = l;
                }
                mem::swap(&mut bnode.elem, &mut min_node.elem);
                mem::swap(&mut bnode.multiplicity, &mut min_node.multiplicity);
                Self::_remove_(&mut bnode.right, elem);
                bnode.left = Some(left);
                bnode.right = Some(right);
                Some(bnode)
            }
        };
        if let Some(n) = link {
            Some(n.elem.clone());
        }
        None
    }
}

fn main() {

    let mut a = OsTree::new();
    a.insert(32);
    a.insert(42);
    a.insert(12);
    a.insert(1);
    a.insert(1);
    // println!("{a:?}");
    println!("length {:?}", a.len());
    println!("rank 42 {:?}", a.rank(42));

    println!("select {:?}", a.select(3));
    a.remove(13);
    println!("rank 42 {:?}", a.rank(42));
    a.remove(1);
    println!("rank 42 {:?}", a.rank(42));
    a.remove(1);
    println!("rank 42 {:?}", a.rank(42));
    println!("length {:?}", a.len());
    println!("{a:?}");
    // println!("{a:?}");
}

