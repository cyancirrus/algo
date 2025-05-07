// fn largest_subarray(nums:&[i32]) -> &[i32] {
//     // current max
//     let mut max:i32 = 0;
//     let mut max_l:usize = 0;
//     let mut max_r:usize = 0;
//     // rolling value for potential sum
//     let mut max_ps= 0;
//     // buffer to track index
//     let mut buff:i32 = 0;
//     let mut buff_l:usize = 0;
//     let mut buff_r:usize;

//     for i in 0..nums.len() {
//         let num = nums[i];
//         max_ps += num;
//         if buff > 0 {
//             buff_r = i + 1;
//             buff += num;
//         } else {
//             buff_l = i;
//             buff_r = i + 1;
//             buff = num;
//         }
//         if max_ps > max {
//             max = max_ps;
//             max_r = i + 1;
//         }
//         if buff > max {
//             max = buff;
//             max_ps = buff;
//             max_l = buff_l;
//             max_r = buff_r;
//             buff = 0;
//         }

//     }
//     &nums[max_l..max_r]
// }

fn largest_subarray(nums:&[i32]) -> &[i32] {
    // current max
    let mut max:i32 = 0;
    let mut max_l:usize = 0;
    let mut max_r:usize = 0;
    // potential sum to track index
    let mut psum:i32 = 0;
    let mut psum_l:usize = 0;
    let mut psum_r:usize;

    for (i, num) in nums.iter().enumerate() {
        if psum < 0 {
            psum = *num;
            psum_l = i;
            psum_r = i + 1;
        } else {
            psum += num;
            psum_r = i + 1;
        }
        if psum > max {
            max = psum;
            max_l = psum_l;
            max_r = psum_r;
        }

    }
    &nums[max_l..max_r]
}


fn main() {
    assert_eq!(&[1,2,3], largest_subarray(&[1,2,3]));
    assert_eq!(&[1,2,-2,3], largest_subarray(&[1,2,-2,3]));
    assert_eq!(&([] as [i32;0]), largest_subarray(&[-1, -2, -3]));
}
