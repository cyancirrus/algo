use std::collections::HashMap;

// fn can_be_valid(s:&str, l:&str) -> bool {
//     fn bt(
//         i:usize,
//         n:usize,
//         opens:usize,
//         closes:usize,
//         s:&[ u8 ],
//         l:&[ u8 ],
//     ) -> bool {
//         if i == n  { return opens == closes; }
//         if closes > opens || opens > n / 2 { return false; }
//         let locked = l[i] == b'1';
//         if !locked {
//             bt(i+1, n, opens+ 1, closes, s, l)
//                 || bt(i+1, n, opens, closes + 1, s, l) 
//         } else {
//             match s[i] {
//                 b'(' => bt(i+1, n, opens+1, closes, s, l) ,
//                 b')' => bt(i+1, n, opens, closes+1, s, l),
//                 _ => { false }
//             }
//         }
//     }
//     if s.len() & 1 == 1 { return false; }
//     bt(0, s.len(), 0, 0, s.as_bytes(), l.as_bytes())
// }

fn can_be_valid(s:&str, l:&str) -> bool {
    let (mut lo, mut hi) = (0, 0);
    let (sb, lb) = (s.as_bytes(), l.as_bytes());

    for i in 0..sb.len() {
        if lb[i] == b'0' {
            lo -= 1;
            hi += 1;
        } else {
            if sb[i] == b'(' {
                lo += 1;
                hi += 1;
            } else {
                lo -= 1;
                hi -= 1;
            }
        }
        if hi < 0 {
            return false;
        }
        if lo < 0 {
            lo = 0;
        }
    }
    lo == 0

}



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
        can_be_valid("))()))","010100")
        // can_be_valid("))","01")
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
