fn burst_balloons(_nums:&[u32]) -> u32 {
    let len = _nums.len();
    let pad = len + 2;
    let mut nums =  vec![1];
    let mut cache = vec![vec![-1;pad];pad];
    nums.extend(_nums.iter());
    nums.push(1);
    _burst_balloons_(0, pad-1, &nums, &mut cache)
}

fn _burst_balloons_(l:usize, r:usize, nums:&[u32], cache:&mut Vec<Vec<i32>>) -> u32 {
    if r <= l{ return 0 };
    let mut max = 0;
    if cache[l][r] != -1 { return cache[l][r] as u32 };
    for i in l+1..r {
        let cur = {
            _burst_balloons_(l, i, &nums, cache)
            + nums[l] * nums[i] * nums[r]
            + _burst_balloons_(i, r, &nums, cache)
        };
        max = max.max(cur);
    }
    cache[l][r]=max as i32;
    max
}

fn balloons(nums:&[u32]) -> u32 {
    let n = nums.len();
    let mut padded = vec![1];
    padded.extend_from_slice(nums);
    padded.push(1);
    let mut dp = vec![vec![0;n+2];n+2];
    for length in 2..=n + 1 {
        for l in 0..=n + 1 - length {
            let r = l + length;
            for i in l + 1..r {
                dp[l][r] = dp[l][r].max(
                    dp[l][i] 
                    + padded[l] * padded[i] * padded[r]
                    + dp[i][r]
                );
            }
        }
    }
    dp[0][n+1]
}


fn main() {
    // assert_eq!(167, burst_balloons(&[3, 1, 5, 8]));
    // assert_eq!(10, burst_balloons(&[1,5]));
    // assert_eq!(167, balloons(&[3, 1, 5, 8]));
    assert_eq!(10, balloons(&[1,5]));
}
