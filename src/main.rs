fn rob(nums:&mut [usize]) -> usize {
    let n = nums.len();
    if n == 0 { return 0 };
    if n == 1 { return nums[0] };
    if n >= 3 {
        nums[n-3] += nums[n-1];
    }
    for i in (0..n-3).rev() {
        nums[i] += nums[i+2].max(nums[i+3]);
    }
    nums[0].max(nums[1])
}


fn main() {
    assert_eq!(rob(& mut[100,2,3,100]), 200);  // Rob houses 1 and 3 (1-based)
    assert_eq!(rob(& mut[1,2,3,1]), 4);  // Rob houses 1 and 3 (1-based)
    assert_eq!(rob(& mut[2,7,9,3,1]), 12); // Rob houses 2, 4, and 5
}
