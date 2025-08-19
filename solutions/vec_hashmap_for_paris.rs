use std::collections::HashMap;


fn longest_arithmetic(nums: &[i32]) -> usize {
    let n = nums.len();
    let mut dp:Vec<HashMap<i32, usize>> = vec![HashMap::new();n];
    let mut max = 0;
    
    for i in 0..n {
        for j in 0..i {
            let d = nums[i] - nums[j];
            let len = dp[j].get(&d).unwrap_or(&1) + 1;
            let updated = dp[i].entry(d).or_insert(0);
            *updated = (*updated).max(len);
            max = max.max(len);
        }
    }
    max
}


// fn longest_arithmetic(nums:&[i32]) -> usize {
//     fn bt(i:usize, prev:Option<usize>, diff:Option<i32>, nums:&[i32]) -> usize {
//             println!("hello");
//         let mut longest = 0; 
//         for i in i..nums.len() {
//             longest = longest.max(1 + {
//                 match (prev, diff) {
//                     (None, None) => bt(i + 1, Some(i), None, nums),
//                     (None, _) => 0,
//                     (Some(p), None) => bt(i+1, Some(i), Some(nums[i] - nums[p]), nums), 
//                     (Some(p), Some(d)) => {
//                         if nums[i] == nums[p] + d {
//                             return 1 + bt(i+1, Some(i), diff, nums)
//                         } else {
//                             0
//                         }
//                     },
//                 }
//             });
//         }
//         longest
//     }
//     bt(0, None, None, nums)
// }

fn destroy_targets(nums:Vec<usize>, space:usize) -> usize {
    let mut freq = vec![0;space];
    let mut lows = vec![usize::MAX;space];
    let mut tgx = 0;
    for n in nums {
        let rdx = n % space;
        freq[rdx] += 1;
        lows[rdx] = lows[rdx].min(n);
        if freq[rdx] > freq[tgx] || ( freq[rdx] == freq[tgx] && lows[rdx] < lows[tgx]) {
                tgx = rdx;
        }
    }
    lows[tgx]
}


fn destroy_targets_gcd(nums:&mut [u32]) -> u32 {
    let n = nums.len();
    nums.sort();
    let mut div = vec![0;n];
    if nums[0] <= 1 { return 1; }
    
    for idx in 0..nums.len() { 
        for jdx in 0..idx {
            if nums[idx] / 2 < nums[jdx]  { break; }
            if nums[idx] % nums[jdx] == 0 {
                div[idx] += 1;
            }
        }
    }
    let mut max = 0;
    let mut tgx = 0;
    for (idx, &cnt) in div.iter().enumerate() {
        if cnt < max { continue; }
        max = cnt;
        tgx = idx;
    }
    nums[tgx]
}


fn main() {
    println!("longest arith {:?}", longest_arithmetic(&[9,4,7,2,10]));
    println!("longest arith {:?}", longest_arithmetic(&[3,6,9,12]));
}
