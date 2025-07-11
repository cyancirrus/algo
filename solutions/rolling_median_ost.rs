#![allow(dead_code)]
use std::mem;
use num_traits::ToPrimitive;

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
    pub size: usize,
    pub root: Link<T>,
}


impl <T> OsTree<T> 
where T: Ord + PartialOrd

{
    fn new() -> Self {
        Self {
            root:None,
            size:0,
        }
    }
    fn insert(&mut self, elem:T) {
        self.size+=1;
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

    fn remove<'a>(&mut self, elem:T) 
    where T:Copy
    {
        if let Some(root) =  Self::_remove_(&mut self.root, elem) {
            self.size-=1;
            let c = root.elem;
            Self::_decrement_(&mut self.root, c);
        }
    }
    fn _decrement_<'a>(mut link: &mut Link<T>, elem:T) {
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
    fn _remove_(link: &mut Link<T>, elem: T) -> Option<&Node<T>> {
        let mut bnode = match link.take() {
            Some(node) => node,
            None => return None,
        };
        if elem < bnode.elem {
            Self::_remove_(&mut bnode.left, elem);
            *link = Some(bnode);
            return link.as_deref();
        } else if bnode.elem < elem {
            Self::_remove_(&mut bnode.right, elem);
            *link = Some(bnode);
            return link.as_deref();
        }

        if bnode.multiplicity > 1 {
            bnode.multiplicity -= 1;
            *link = Some(bnode);
            return link.as_deref();
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
        link.as_deref()
    }
    fn median(&self) -> f32 
    where T: ToPrimitive{
        // adjusted to make size retrieve indices
        let m_lower = (self.size - 1)/ 2;
        let m_upper = (self.size)/ 2;
        let mut med = self.select(m_lower).unwrap().to_f32().unwrap();
        if m_lower != m_upper {
            med = (med + self.select(m_upper).unwrap().to_f32().unwrap()) / 2f32
        };
        med
    }
}


fn window_median(k:usize, data:&[u32]) -> Vec<f32> {
    let n = data.len();
    let mut ost =  OsTree::new();
    let mut medians = Vec::with_capacity(n-k + 1);
    for i in 0..k {
        ost.insert(data[i]);
    }
    medians.push(ost.median());
    for i in k..n {
        ost.remove(data[i-k]);
        ost.insert(data[i]);
        medians.push(ost.median());
    }
    medians
}


fn main() {
    let data = vec![1, 3, 2, 4, 5];
    let k = 3;
    assert_eq!(vec![2f32, 3f32, 4f32], window_median(k, &data));
}

