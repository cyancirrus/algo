struct TreeNode {
    val:u32, 
    left:Link,
    right:Link,
}

type Link = Option<Box<TreeNode>>;

fn houserobber_iii(root:Link) -> u32 {
    fn dfs(link:Link) -> (u32, u32) {
        if let Some(n) = link {
            let (ls, lt) = dfs(n.left);
            let (rs, rt) = dfs(n.right);
            let skip = rs.max(rt) + ls.max(lt);
            let take = n.val + rs + ls;
            (skip, take)
        } else {
            (0, 0)
        }
    }
    let (skip, take) = dfs(root);
    skip.max(take)
}

fn house_robber(nums:&[u32]) -> u32 {
    if nums.is_empty() { return 0; }
    if nums.len() == 1 { return nums[0]; }
    let (mut s, mut t) = (0, nums[0]);

    for &n in &nums[1..] {
        let p = s;
        s = s.max(t);
        t = p + n;
    }
    t.max(s)
}

fn house_robber_other(nums:&[u32]) -> u32 {
    if nums.is_empty() { return 0; }
    if nums.len() == 1 { return nums[0]; }
    let (mut pre2, mut pre1) = (nums[0], nums[0].max(nums[1]));

    for &n in &nums[2..] {
        let temp = pre1.max(n + pre2);
        pre2 = pre1;
        pre1 = temp;
    }
    pre1
}



// fn houserobber_iii(root:Link) -> u32 {
//     fn recurse(link:Link) -> (u32, u32) {
//         if let Some(node) = link {
//             let (lpre2, lpre1) = recurse(node.left);
//             let (rpre2, rpre1) = recurse(node.right);
//             let temp = (lpre1 + rpre1).max(node.val + lpre2 + rpre2);
//             return (lpre1 + rpre1, temp);
//         }
//         (0, 0)
//     }
//     recurse(root).1
// }



fn main() {
    println!(
        "result {:?}",
        // maximum_mul_subarray(&[2,-1])
        house_robber(&[2,7,9,3,1])
        // maximum_subarray(&[-2,1,-3,4,-1,2,1,-5,4])
    );
    // println!(
    //     "result {:?}",
    //     // maximum_mul_subarray(&[2,-1])
    //     product_array_except_self(&[1,2,3,4])
    //     // maximum_subarray(&[-2,1,-3,4,-1,2,1,-5,4])
    // );
    // println!(
    //     "result {:?}",
    //     // maximum_mul_subarray(&[2,-1])
    //     maximum_mul_subarray(&[2,3,-2,4])
    //     // maximum_subarray(&[-2,1,-3,4,-1,2,1,-5,4])
    // );
    // println!(
    //     "result {:?}",
    //     maximum_add_subarray(&[5,4,-1,7,8])
    //     // maximum_subarray(&[-2,1,-3,4,-1,2,1,-5,4])
    // );
}
