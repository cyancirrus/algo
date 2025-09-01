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
        maximum_mul_subarray(&[2,3,-2,4])
        // maximum_subarray(&[-2,1,-3,4,-1,2,1,-5,4])
    );
    println!(
        "result {:?}",
        maximum_add_subarray(&[5,4,-1,7,8])
        // maximum_subarray(&[-2,1,-3,4,-1,2,1,-5,4])
    );
}
