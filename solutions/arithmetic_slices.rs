use std::mem;

fn tri_sum(len:usize) -> usize {
    if len >= 2 {
        (len - 1)*len/2
    } else {
        0
    }
}

fn arithmetic_slices(nums:&[i32]) -> usize {
    if nums.len() < 3 { return 0 };
    let mut diff = nums[1] - nums[0];
    let mut length = 0;
    let mut total = 0;
    for i in 1..nums.len() {
        let d = nums[i] - nums[i-1];
        if d == diff {
            length += 1;
        } else {
            total+=tri_sum(length);
            diff = d;
            length = 1;
        }
    }
    total+=tri_sum(length);
    total
}


fn main() {
    assert_eq!(0, arithmetic_slices(&[9]));
    assert_eq!(1, arithmetic_slices(&[1,2,3])); // 1
    assert_eq!(3, arithmetic_slices(&[1,2,3,4])); // 2 + 1
    assert_eq!(6, arithmetic_slices(&[1,2,3,4,5])); // 3 + 2 + 1
    assert_eq!(10, arithmetic_slices(&[1,2,3,4,5,6])); // 4 + 3 + 2 + 1
    assert_eq!(10, arithmetic_slices(&[5,2,1,2,3,4,5,6])); // 4 + 3 + 2 + 1
}

// -- a sequence of increasing length n long is (n-2)(n-1)/2 (triangle sum)
//     - (3-2)(3-1)/2 = 1 * 2/2 = 1
//     - (6-2)(6-1)/2 = 4 * 5 / 2 = 10


// so essentially all we need to do is to track a diff
// a length and a aggregator


