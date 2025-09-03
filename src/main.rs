use std::collections::HashMap;



// fn generate_parens(n:usize) -> Vec<String> {
//     let mut res = vec![];
    
//     fn dfs(
//         opens:usize,
//         remain:usize,
//         cur:&mut Vec<u8>,
//         res:&mut Vec<Vec<u8>>,
//     ) {
//         if remain == 0 {
//             res.push(cur.clone());
//             return
//         }
//         if remain - opens > 1 {
//             cur.push(b'(');
//             dfs(opens + 1, remain - 1, cur, res);
//             cur.pop();
//         }
//         if opens > 0 {
//             println!("remain {remain:?}");
//             cur.push(b')');
//             dfs(opens - 1, remain - 1, cur, res);
//             cur.pop();
//         }
//     }
//     dfs(0, n << 1, &mut vec![], &mut res);

//     res.into_iter()
//         .map(|bs| String::from_utf8(bs).unwrap())
//         .collect()
// }
fn generate_parens(n:usize) -> Vec<String> {
    let mut res = vec![];
    
    fn dfs(
        left:usize,
        right:usize,
        cur:&mut Vec<u8>,
        res:&mut Vec<Vec<u8>>,
    ) {
        if left == 0 && right == 0 {
            res.push(cur.clone());
            return
        }
        if left > 0 {
            cur.push(b'(');
            dfs(left - 1, right, cur, res);
            cur.pop();
        }
        if right > left {
            cur.push(b')');
            dfs(left, right - 1, cur, res);
            cur.pop();
        }
    }
    dfs(n, n, &mut vec![], &mut res);
    res.into_iter()
        .map(|bs| String::from_utf8(bs).unwrap())
        .collect()
}

fn main() {
    println!(
        "result {:?}",
        // maximum_mul_subarray(&[2,-1])
        generate_parens(3)
        // maximum_subarray(&[-2,1,-3,4,-1,2,1,-5,4])
    );
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
