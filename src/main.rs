mod tree;
mod tree_solutions;
use tree::Tree;



fn main() {
    let mut a:Tree<usize> = Tree::new();
    a.insert(123);
    a.insert(10);
    a.insert(500);
    println!("In order elements appear as {:?}", a.in_order());
    println!("Is a equal to a {:?}", a == a);
    // a.iter().map(|&elem| elem + 100);
    // println!("Tree appears as {:?}", a);
    // println!("hello world");
}
