use std::collections::HashMap;


fn minimum_round(nums:&[u32]) -> isize {
    let n = nums.len();
    let mut freq:HashMap<u32, isize> = HashMap::with_capacity(n);
    for &n in nums { *freq.entry(n).or_default()+=1; }
    let mut rounds = 0;
    for &k in freq.keys() {
        if k == 1 { return -1; }
        rounds += (k as isize + 2) / 3;
    }
    rounds 
}



// 1 -> 3 + 2 -> 3
fn climbing_stairs_ii(n:usize) -> usize {
    if n == 0 { return 0; }
    let mut prev1 = 1;
    let mut prev2 = 0;

    for _ in 0..n {
        let temp = prev2 + prev1;
        prev2 = prev1;
        prev1 = temp;
    }
    prev1
}
// 3 = ones + twos
// 4 = ones + twos
// new = prev + prev2, prev2 = prev1

fn climbing_stairs(n:usize) -> usize {
    if n < 3 { return n; }
    climbing_stairs(n-1) + climbing_stairs(n-2)
}


fn main() {
    println!(
        "result {:?}",
        climbing_stairs_ii(4)
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
