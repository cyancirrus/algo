use std::collections::HashSet;
use std::collections::VecDeque;


type Link = Option<Box<Node>>;
struct Node {
    val:usize, 
    left:Link,
    right:Link,
}
struct BinaryTree {
    head: Link,
}
fn path_sum(tree:BinaryTree, targetsum:usize) -> bool {
    let mut stack: Vec<(usize, &Node)> = Vec::new();
    if let Some(ref node) = tree.head {
        stack.push((node.val, node.as_ref()));
    }
    while let Some((rsum, node)) = stack.pop() {
        if node.left.is_none() && node.right.is_none() && rsum == targetsum {
            return true;
        }
        if let Some(ref l) = node.left {
            stack.push((rsum + l.val, l.as_ref()));
        }
        if let Some(ref r) = node.right {
            stack.push((rsum + r.val, r.as_ref()));
        }
    }
    false
}



fn main() {
    // println!("result of simplify {:?}", simplify_path("/home/user/Documents/../Pictures"));
    // println!("result of simplify {:?}", simplify_path("/home/.."));
    // println!("longest palin {:?}", longest_palindrome("babad"));
    // println!("longest palin {:?}", longest_palindrome("cbbc"));
    // println!("longest palin {:?}", longest_palindrome_string("babad"));
}
