fn remove_duplicates_from_array(nums:&mut [i32]) -> &[i32] {
    if nums.is_empty() {
        return nums;
    }
    let mut k:usize=1;
    for i in 1..nums.len() {
        if nums[i] != nums[k-1] {
            nums.swap(i, k);
            k+=1;
        }
    }
    &nums[0..k]
}

// fn remove_duplicates_from_array(nums:&mut [i32]) -> &[i32] {
//     if nums.is_empty() {
//         return nums;
//     }
//     let mut k:usize=2;
//     for i in 2..nums.len() {
//         if nums[i] != nums[k-2] {
//             nums.swap(i, k);
//             k+=1;
//         }
//     }
//     &nums[0..k]
// }





fn main() {
    assert_eq!(&[1,2,3], remove_duplicates_from_array(&mut [1,1,1,1,2,2,3,]), "basic case");
    assert_eq!(&[1,2,3], remove_duplicates_from_array(&mut [1,2,2,3,]), "basic case");
    assert_eq!(&[0,1,2,3], remove_duplicates_from_array(&mut [0, 1,2,2,3,]), "basic case");
    assert_eq!(&[0,1,2,3], remove_duplicates_from_array(&mut [0,1,2,3,]), "basic case");
}
