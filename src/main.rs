#![allow(dead_code)]

#[derive(Debug)]
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
    println!("rank 42 {:?}", a.rank(2));

    println!("select {:?}", a.select(3));
}

