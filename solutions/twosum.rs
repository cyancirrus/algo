use std::collections::HashMap;


fn two_sum(nums:Vec<i32>, target:i32) -> (usize, usize) {
    let mut seen:HashMap<i32, usize> = HashMap::with_capacity(nums.len());

    for (i, val) in nums.iter().enumerate() {
        if let Some(j) = seen.get(&(target - val)) {
            return (*j, i)
        }
        else {
            seen.insert(nums[i], i);
        }
    }
    // handle the case when none are matched
    (usize::MAX, usize::MAX)
}



fn main() {
    let nums = vec![2, 7, 11, 15,];
    println!("The found result is {:?}", two_sum(nums, 9));
    println!("hello world");
}
