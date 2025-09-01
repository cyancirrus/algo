// fn maximum_add_subarray(nums:&[i32]) -> i32 {
//     let mut max = 0;
//     let mut sum = 0;
//     for &n in nums {
//         if n > 0 { sum += n; continue; }
//         max = max.max(sum);
//         sum = sum + n;
//         sum = sum.max(0);
//     }
//     max = max.max(sum);
//     max
// }

// 2 6 4 1 6
// 6 1 4 6 2
// 6 6 
// 6 10
//
//

fn house_robber(nums:&[u32]) -> u32 {
    if nums.is_empty() { return 0; }
    if nums.len() == 1 { return nums[0]; }
    let mut prev2 = nums[0];
    let mut prev1 = nums[1].max(nums[0]);
    
    for &n in &nums[2..] {
        let temp = prev1.max(n + prev2);
        prev2 = prev1;
        prev1 = temp;
    }
    prev1
}


fn product_array_except_self(nums:&[i32]) -> Vec<i32> {
    if nums.is_empty() { return vec![]; }
    let n = nums.len();
    let mut res = vec![1;n];
    let mut run = 1;
    for i in 1..n {
        run *= nums[i-1];
        res[i] = run;
    }
    let mut run = 1;
    for i in (0..n-1).rev() {
        run *= nums[i+1];
        res[i] *= run;
    }
    res
}

fn maximum_add_subarray(nums:&[i32]) -> i32 {
    let mut max = i32::MIN;
    let mut sum = 0;
    for &n in nums {
        sum = n.max(n + sum);
        max = max.max(sum);
    }
    max
}
fn maximum_mul_subarray(nums:&[i32]) -> i32 {
    let mut max = nums[0]; 
    let mut pos = nums[0];
    let mut neg = nums[0];
    for &n in &nums[1..] {
        let tmp = pos;
        pos = n.max(tmp * n).max(neg * n);
        neg = n.min(tmp * n).min(neg * n);
        max = max.max(pos);
    }
    max
    
}

fn main() {
    println!(
        "result {:?}",
        // maximum_mul_subarray(&[2,-1])
        // house_robber(&[2,7,9,3,1])
        house_robber(&[1,6,4,1,6])
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
