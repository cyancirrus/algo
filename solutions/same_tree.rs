use std::collections::HashMap;

struct TreeNode {
    val: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,

}


type Link = Option<Box<TreeNode>>;

fn same_node(a:&Link, b:&Link) -> bool {
    let mut stack: Vec<(&Link, &Link)> = vec![(a,b)];
    while let Some((l, r)) = stack.pop() {
        match (l, r) {
            (Some(ln), Some(rn)) => {
                if ln.val != rn.val { return false; }
                stack.push((&ln.left, &rn.left));
                stack.push((&ln.right, &rn.right));
            },
            (None, None) => { continue; },
            _ => { return false; },
        }
    }
    true
}


fn main() {
    println!("min window sub {:?}", min_window_substr("ADOBECODEBANC", "ABC"));
}
