mod tree;
mod tree_solutions;
use std::collections::VecDeque;
use tree::{Tree, Node};
use std::rc::Rc;



fn main() {
    let mut a:Tree<usize> = Tree::new();
    a.insert(123);
    a.insert(10);
    a.insert(500);
    a.insert(1);
    a.insert(5);
    a.insert(2);
    a.insert(25);
    println!("In order elements appear as {:?}", a.in_order());
    // println!("Is a equal to a {:?}", a == a);
    a.pbreadth();
    a.zigzag();
    let data = VecDeque::from([
        Some(0),
        None,
        Some(1),
        None,
        Some(2),
        Some(3)
    ]);
    let t = Tree::from_vec(data.clone());
    println!("Tree {t:?}");
    assert_eq!(data, t.to_vec());
    // a.iter().map(|&elem| elem + 100);
    // println!("Tree appears as {:?}", a);
    // println!("hello world");
}
