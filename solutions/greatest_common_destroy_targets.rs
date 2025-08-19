use std::collections::HashMap;

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
}
