// fn insert_interval(nums:&[(u32, u32)], new:(u32, u32)) -> Vec<(u32, u32)> {
//     // assumed
//     // nums.sort_unstable_by(|a, b| a.0.cmp(&b.0).then(a.1.cmp(&b1)));
//     let (mut l, mut u) = new;
//     let mut inserted = false;
//     let mut res = Vec::with_capacity(nums.len() + 1);
//     for &(cl, cu) in nums {
//         if cu < l {
//             res.push((cl, cu));
//         } 
//         else if u < cl {
//             if !inserted {
//                 res.push((l,u));
//                 inserted = true;
//             }
//             res.push((cl, cu));
//         }
//         else {
//             l = l.min(cl);
//             u = u.max(cu);
//         }
//     }
//     if !inserted {
//         res.push((l, u));
//     }
//     res
// }

// fn insert_interval(nums:&[(u32, u32)], new:(u32, u32)) -> Vec<(u32, u32)> {
//     // assumed
//     // nums.sort_unstable_by(|a, b| a.0.cmp(&b.0).then(a.1.cmp(&b1)));
//     let (mut l, mut u) = new;
//     let n = nums.len();
//     let mut hotspot = false;
//     let mut res = Vec::with_capacity(n + 1);
//     for i in 0..n {
//         let (cl, cu) = nums[i];
//         if cl <= u && l <= cu {
//             l = l.min(cl);
//             u = u.max(cu);
//             hotspot = true;
//             continue;
//         } else if hotspot {
//             res.push((l, u));
//             res.extend(&nums[i..]);
//             break;
//         }
//         res.push((cl, cu));
//     }
//     res
// }

fn merge_intervals(nums:&mut [(u32, u32)]) -> &[(u32, u32)] {
    nums.sort_unstable_by(|&a, &b| a.0.cmp(&b.0).then(a.1.cmp(&b.1)));
    if nums.is_empty() { return nums; }
    let (mut l, mut u) = nums[0];
    let mut curs = 0;

    for i in 1..nums.len() {
        let (cl, cu) = nums[i];
        if cl <= u {
            u = u.max(cu);
        } else {
            nums[curs] = (l, u);
            curs += 1;
            l = cl; u = cu;
        }
    }
    nums[curs] = (l, u);
    &nums[0..=curs]
}


// fn merge_intervals(nums:&mut [(u32, u32)]) -> &[(u32, u32)] {
//     if nums.is_empty() { return nums; }
//     let n = nums.len();
//     let mut idx= 0;
//     let mut curs = 0;

//     while idx < n {
//         let (mut l, mut u) = nums[idx];
//         idx += 1;
//         while idx < n {
//             let (cl, cu) = nums[idx];
//             // if  l <= cu && cl <= u {
//             if  cl <= u {
//                 idx += 1;
//                 l = l.min(cl);
//                 u = u.max(cu);
//                 continue;
//             }
//             break;
//         }
//         nums[curs] = (l, u);
//         curs += 1;
//         idx += 1;
//     }

//     &nums[0..curs]
// }


fn gray_code(n:usize) -> Vec<usize> {
    let l = 1usize.checked_shl(n as u32).expect("n too large for usize");
    let mut res = vec![0;l];
    for i in 1..l {
        res[i] = i ^ (i >> 1);
    }
    res
}

fn main() {
    println!("gray code {:?}", gray_code(2));

}
