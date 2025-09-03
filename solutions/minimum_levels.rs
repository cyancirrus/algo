use std::collections::HashMap;

fn minimum_levels(nums:&[u32]) -> isize {
    let n = nums.len();
    let mut points = vec![0;n+1];
    for i in (0..n).rev() {
        points[i] = points[i+1] + if nums[i] == 1 {  1 } else { -1 };
    }
    let mut prefix = 0;
    for i in 0..=n {
        if prefix > points[i] { return i as isize; }
        if i < n {
            prefix += if nums[i] == 1 { 1 } else { - 1 };
        }
    }
    return -1
}

fn main() {
    println!(
        "result {:?}",
        minimum_levels(&[0,0,0,0])
        // maximum_mul_subarray(&[2,-1])
        // maximum_subarray(&[-2,1,-3,4,-1,2,1,-5,4])
    );
    println!(
        "result {:?}",
        minimum_levels(&[1,1,1,1,1])
        // maximum_mul_subarray(&[2,-1])
        // maximum_subarray(&[-2,1,-3,4,-1,2,1,-5,4])
    );
    println!(
        "result {:?}",
        minimum_levels(&[1,0,1,0])
        // maximum_mul_subarray(&[2,-1])
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
