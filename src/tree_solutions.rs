use crate::tree::{Tree, Link};
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
// mod tree;
// mod tree_solutions;
// use std::collections::VecDeque;
// use tree::{Tree, Node, LevelOrderIter, PreOrderIter, PostOrderIter};
// use std::rc::Rc;

// fn main (){
//     let mut a:Tree<usize> = Tree::new();
//     a.insert(123);
//     a.insert(10);
//     a.insert(500);
//     a.insert(1);
//     a.insert(5);
//     a.insert(2);
//     a.insert(25);
//     println!("In order elements appear as {:?}", a.in_order());
//     // println!("Is a equal to a {:?}", a == a);
//     a.zigzag();
//     let data = VecDeque::from([
//         Some(0),
//         None,
//         Some(1),
//         None,
//         Some(2),
//         Some(3)
//     ]);
//     let t = Tree::from_vec(data.clone());
//     println!("Tree {t:?}");
//     assert_eq!(data, t.to_vec());
//     for val in LevelOrderIter::new(t.root.as_deref()) {
//         println!("{}", val);
//     }
//     for val in PreOrderIter::new(t.root.as_deref()) {
//         println!("{}", val);
//     }
//     for val in PostOrderIter::new(t.root.as_deref()) {
//         println!("{}", val);
//     }
//     // a.iter().map(|&elem| elem + 100);
//     // println!("Tree appears as {:?}", a);
//     // println!("hello world");
// }
