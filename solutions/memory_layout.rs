// fn remove_element(target:i32, nums:&mut [Option<i32>]) -> usize {
//     // target is 4 bytes
//     // &mut ie nums is 8 bytes as it is a pointer
//     nums.iter_mut().map(|elem| {
//         if *elem == Some(target) {
//             *elem = None;
//             1
//             // usize = 8 bytes as it usize == u64
//         } else {
//             0
//             // usize = 8 bytes as it usize == u64
//         }
//     }).sum()
// }

// fn main(){
//     println!("hello world");
//     let mut nums:[Option<i32>;6] = [None; 6];
//     nums[0] = Some(3);
//     // cannot contain i32 (4 bytes) and None, need 1 byte for none and then allignment
//     // Option<i32> therefore is therefore 8 bytes
//     nums[1] = Some(2);
//     nums[2] = Some(2);
//     nums[3] = Some(3);

//     let test = remove_element(3, &mut nums);
//     println!("test {}", test);
// }

fn remove_element<T:PartialEq>(target:T, nums:&mut [Option<&T>]) -> usize {
    // target bytes depends upon the type T, for i32 it is 4 bytes, for i8 it is 1 byte
    // nums :: n * 8 bytes as pointer is always 8 bytes
    // nums is an immutable reference to a mutable slice
    let mut count = 0;
    // 8 bytes
    for position in nums {
        if let Some(elem) = position {
            if **elem == target {
                count+=1;
                // usize = 8 bytes as it usize == u64
                *position = None;
            }
        }
    }
    count
}


fn main(){
    println!("hello world");
    let mut nums:[Option<&i32>;6] = [None; 6];
    nums[0] = Some(&3);
    // nums size is i32 (4 bytes * 6 = 48 bytes);
    // each underlying data is 4 bytes which the pointer references
    // nums contains pointers into data, x0000_0000 means None
    nums[1] = Some(&2);
    nums[2] = Some(&2);
    nums[3] = Some(&3);

    let test = remove_element(3, &mut nums);
    println!("test {}", test);
    println!("nums looks like {:?}", nums);
}
