fn combination_sum(candidates:&[usize], target:usize) -> Vec<Vec<usize>> {
    let mut sols = vec![];
    bt(0, target, candidates, &mut vec![], &mut sols);
    sols
}

fn bt(start:usize, target:usize, cands: &[usize], nums:&mut Vec<usize>, sols: &mut Vec<Vec<usize>>) {
    if target == 0 {
        sols.push(nums.clone());
        return;
    }
    for i in start..cands.len() {
        let c = cands[i];
        if c == 0 || c > target { break; }
        nums.push(c);
        bt(i, target - c, cands, nums, sols);
        nums.pop();
    }
}



// fn combination_sum(candidates:&[usize], target:usize) -> Vec<Vec<usize>> {
//     let mut sols = vec![];
//     bt(0, target, 0, candidates, &mut vec![], &mut sols);
//     sols
// }

// fn bt(curr:usize, target:usize, order:usize, cands: &[usize], nums:&mut Vec<usize>, sols: &mut Vec<Vec<usize>>) {
//     if curr == target {
//         sols.push(nums.clone());
//         return;
//     }
//     for &c in cands {
//         if {
//             c != 0
//             && curr + c <= target
//             && c >= order 
//         }
//         {
//             nums.push(c);
//             bt(curr + c, target, order.max(c), cands, nums, sols);
//             nums.pop();
//         }
//     }
// }


// fn combination_sum(candidates:&[usize], target:usize) -> Vec<Vec<usize>> {
//     let mut dp:Vec<Vec<Vec<usize>>> = vec![vec![]; target + 1 ];
//     dp[0].push(vec![]); 
//     for &c in candidates {
//         if c == 0 { continue; }
//         for sum in c..=target {
//             let src = sum - c;
//             let base_len = dp[src].len();
//             dp[sum].reserve(base_len);
//             for i in 0..base_len {
//                 let mut comb = dp[src][i].clone();
//                 comb.push(c);
//                 dp[sum].push(comb);
//             }
//         }
//     }
//     dp[target].clone()
// }


// fn combination_sum(candidates:&[u8], target:u8) -> Vec<Vec<u8>> {
//     let mut sums:Vec<Vec<Vec<u8>>> = vec![vec![]; target as usize +1 ];
//     sums[0].push(vec![]); 
//     for &n in candidates {
//         for i in n..=target {
//             let offset = (i - n) as usize;
//             for mut v in sums[offset].clone() {
//                 v.push(n);
//                 sums[i as usize].push(v);
//             }
//         }
//     }
//     sums.pop().unwrap()
// }

fn main() {
    // println!("generate parens {:?}", generate_parentheses(3));
    println!("combination sum {:?}", combination_sum(&[2,3,6,7], 10));
}
