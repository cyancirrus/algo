// 1    3   5   8   1
// 1    3   120 40  1   
// 1    24  

fn burst_balloons(_nums:&[u32]) -> u32 {
    let len = _nums.len();
    let pad = len + 2;
    let mut nums =  vec![1];
    nums.extend(_nums.iter());
    nums.push(1);
    _burst_balloons_(0, pad-1, &nums)
}

fn _burst_balloons_(l:usize, r:usize, nums:&[u32]) -> u32 {
    if r <= l{ return 0 };
    let mut max = 0;
    for i in l+1..r {
        let cur = {
            _burst_balloons_(l, i, &nums)
            + nums[l] * nums[i] * nums[r]
            + _burst_balloons_(i, r, &nums)
        };
        max = max.max(cur);
    }
    max
}

// fn _burst_balloons_(left:usize, right:usize, nums:&[u32]) -> u32 {
//     if right <= left { return 0 };
//     let mut max = 0;
//     for l in left..=right {
//         for r in left..=right {
//             for i in l+1..r {
//                 let cur = {
//                     _burst_balloons_(l, i, &nums)
//                     + nums[l] * nums[i] * nums[r]
//                     + _burst_balloons_(i, r, &nums)
//                 };
//                 if cur > max {
//                     max = cur;
//                 };
//             }
//         }
//     }
//     max
// }

fn balloons(nums:&[u32]) -> u32 {
    let n = nums.len();
    let mut padded = vec![1];
    padded.extend_from_slice(nums);
    padded.push(1);

    let mut memo = vec![vec![-1; n + 2];n+2];
    _balloons_(0, n+1, &padded, &mut memo)
}

fn _balloons_(left:usize, right:usize, nums:&[u32], memo:&mut Vec<Vec<i32>>) -> u32 {
    if left + 1 == right {
        return 0;
    }
    if memo[left][right] != -1 {
        return memo[left][right] as u32;
    }
    let mut max_coins = 0;
    for i in left+1..right {
        let coins = {
            _balloons_(left, i, nums, memo)
            + nums[left] * nums[i] * nums[right]
            + _balloons_(i, right, nums, memo)
        };
        max_coins = max_coins.max(coins);
    }
    max_coins
}


fn main() {
    assert_eq!(167, burst_balloons(&[3, 1, 5, 8]));
    assert_eq!(10, burst_balloons(&[1,5]));
    assert_eq!(167, balloons(&[3, 1, 5, 8]));
    assert_eq!(10, balloons(&[1,5]));
}

// fn burst_balloons(nums:&[u32]) -> u32 {
//     // misunderstood question
//     if nums.len() <= 3 { return nums.iter().product() };
//     let mut sum = nums[0] * nums[1] * nums[2];
//     let mut max = sum;

//     for (i, &num) in nums[3..].iter().enumerate() {
//         let cur = sum * num / nums[i - 3];
//         if cur > max {
//             max = cur;
//         };
//     }
//     max
// }


// neat but it isn't greedy
// prioritize grouping and try to get highest numbers closest to eachother

// struct MaxHeap {
//     data:Vec<(usize,u32)>, // contains mapping index of element, value
//     heap_map:Vec<usize>, // contains mapping ie heap_map[0] (where is element 0 on heap)
// }

// impl MaxHeap {
//     fn heapify(nums:&[u32]) -> Self {
//         todo!()
//     }
//     fn pushdown(){
//         todo!()
//     }
// }

// fn burst_balloons(nums:&[u32]) -> u32 {
//     let len = nums.len();
//     if len <= 3 { return nums.iter().product() };
//     // precompute neighbors
//     let mut neighbors = vec![0;len];
//     neighbors[0] = nums[1] * nums[2];
//     neighbors[len - 1] = nums[len - 1] * nums[len - 2];
//     for i in 1..len - 2 {
//         neighbors[i] = neighbors[i-1] * neighbors[i] * neighbors[i+1];
//     }
//     // heap.pop()
//     // for the index find the alteration and mutate the heap based upon the heapmap
//     todo!()
// }

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
