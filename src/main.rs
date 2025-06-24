mod tree;
mod tree_solutions;
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
    // a.iter().map(|&elem| elem + 100);
    // println!("Tree appears as {:?}", a);
    // println!("hello world");
}
