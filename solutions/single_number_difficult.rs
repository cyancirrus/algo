//🔹 1. Numerical Stability & Floating Point
    // IEEE-754, error propagation, Kahan summation
    // Matrix condition numbers, stability of ops like QR/SVD
    // When to use fixed-point or arbitrary precision
    // Why it matters: You’re building tools that manipulate math. Understanding rounding error and representational quirks separates black-box ML from real tool-building.
// 🔹 4. Compilers / DSLs / Codegen
// 🔹 5. Control Theory & Signal Processing

use std::collections::HashMap;
use std::mem;

fn maxor_iter(nums:&[u32]) -> u8 {
    let mut max:u32 = 0;
    let mut sums:HashMap<u32, u8> = HashMap::new();
    let mut curr:HashMap<u32, u8> = HashMap::new();

    for n in nums {
        curr.insert(*n, 1);
        for (s,c) in &sums  {
            max = max.max(s|n);
            *curr.entry(s|n).or_insert(1)+=c;
        }
        sums.extend(curr.drain());
    }
    sums[&max]
}

// fn maxor_iter(nums:&[u32]) -> u8 {
//     let mut max:u32 = 0;
//     let mut sums:HashMap<u32, u8> = HashMap::new();
//     let mut curr:HashMap<u32, u8> = HashMap::new();

//     for n in nums {
//         let mut curr = sums.clone();
//         curr.insert(*n, 1);
//         for (s,c) in &sums  {
//             max = max.max(s|n);
//             *curr.entry(s|n).or_default()+=c;
//             // println!("curr[sn] {}", curr[&(s|n)]);
//         }

//         mem::swap(&mut sums, &mut curr);
//     }
//     println!("sums {:?}", sums);
//     println!("max {:?}", max);
//     sums[&max]
// }



// bitwise or can repeatedly combine
fn maxor_subarray(nums:&[u32]) -> u8 {
    let mut memo = HashMap::new();
    let max = nums.into_iter().fold(0, |x, y| x | y );

    fn dfs(idx:usize, val:u32, nums:&[u32], max:u32, memo:&mut HashMap<(usize, u32), u8>) -> u8{
        if idx == nums.len() {
            return if val == max { 1 } else { 0 };
        }
        if let Some(&mem) = memo.get(&(idx, val)) {
            return mem;
        }
        let total = {
            dfs(idx+1, val | nums[idx], nums, max, memo)
            + dfs(idx+1, val, nums, max, memo)
        };
        memo.insert((idx, val), total);
        total

    }
    dfs(0, 0, nums, max, &mut memo)
}

// maximum and == max value
// and is interesection
// then essentially from the max value see the number of copies

fn longest_subarray(nums:&[u32]) -> u8 {
    let mut cnt = 0;
    let mut max = u32::MIN;
    let mut pcnt = 0;
    for &n in nums {
        if n > max {
            max = n;
            cnt = 1;
            pcnt = 1;
        } else if n == max {
            cnt += 1;
            if cnt > pcnt {
                pcnt = cnt;
            }
        } else {
            cnt = 0;
        }
    }
    pcnt
}



fn main() {
    assert_eq!(2, maxor_iter(&[3,1]));
    assert_eq!(6, maxor_iter(&[3,2,1,5]));
    // assert_eq!(6, maxor_subarray(&[3,2,1,5]));
    // assert_eq!(2, maxor_subarray(&[3,1]));
    // assert_eq!(2,longest_subarray(&[1,2,3,3,2,2]));
    // assert_eq!(2,longest_subarray(&[1,2,3,3,2,3]));
    // assert_eq!(3,longest_subarray(&[1,2,3,3,3,2,3]));
    // assert_eq!(1,longest_subarray(&[1,2,3,4]));
    // assert_eq!(2, longest_subarray(&[1, 5, 2, 5, 5]));
}
