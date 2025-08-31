use std::mem;

struct Node {
    val:i32,
    next:Link,
}

type Link = Option<Box<Node>>;

fn reverse(mut head:Link) {
    let mut prev = None;
    while let Some(mut node) = head {
        head = node.next.take();
        node.next = prev;
        prev = Some(node);
    }
}


fn search_insert(nums:&[u32], target:u32) -> usize {
    let (mut l, mut r) = (0, nums.len());
    while l < r {
        let m = (l + r) / 2;
        if nums[m] <= target { l = m + 1;}
        else { r = m; }
    }
    r
}


fn main() {
    println!(
        "result {:?}",
        search_insert(&[1,3,4,4,5,6,10], 4)
    );
}
