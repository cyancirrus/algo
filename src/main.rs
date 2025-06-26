mod tree;
mod tree_solutions;
use std::collections::VecDeque;
use tree::{Tree, Node, LevelOrderIter, PreOrderIter, PostOrderIter};
use std::rc::Rc;

// learn preorder, inorder, postorder, bfs and implications

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
    for val in LevelOrderIter::new(t.root.as_deref()) {
        println!("{}", val);
    }
    for val in PreOrderIter::new(t.root.as_deref()) {
        println!("{}", val);
    }
    for val in PostOrderIter::new(t.root.as_deref()) {
        println!("{}", val);
    }
    // a.iter().map(|&elem| elem + 100);
    // println!("Tree appears as {:?}", a);
    // println!("hello world");
}
