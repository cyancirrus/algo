use std::collections::HashSet;

// fn first_missing(nums:&[i32]) -> i32 {
//     // has O(n) performance but O(n) storage
//     let mut seen = HashSet::with_capacity(nums.len());
//     let mut i = 1;
//     for i in nums {
//         seen.insert(i.clone());
//     }
//     loop {
//         if seen.contains(&i) {
//             i+=1;
//         } else {
//             return i
//         }
//     }
// }

// fn first_missing(nums:&mut [i32]) -> i32 {
//     let len = nums.len();
//     for i in 0..len {
//         let elem = nums[i];
//         if 0 < elem && elem <= len as i32 {
//             println!("Nums appears as {:?}", nums);
//             nums.swap((elem-1) as usize, i);
//         }
//     }
//     println!("Nums appears as {:?}", nums);
//     for i in 0..len {
//         if nums[i] != (i + 1) as i32 {
//             return (i + 1) as i32;
//         }
//     }
//     (len + 1) as i32
// }

fn first_missing(nums:&mut[i32]) -> i32 {
    let len = nums.len();
    for i in 0..len {
        loop {
            let elem = nums[i];
            if 0 < elem && elem <= len as i32 && (i + 1) as i32 != elem {
                nums.swap((elem-1) as usize, i);
            } else {
                break;
            }
        }
    }
    for i in 0..len {
        if nums[i] != (i + 1) as i32 {
            return (i + 1) as i32;
        }
    }
    (len + 1) as i32
}


fn main() {
    assert_eq!(first_missing(&mut vec![0,1,2]), 3);
    assert_eq!(first_missing(&mut vec![2,1,4]), 3);
    assert_eq!(first_missing(&mut vec![2,1,3]), 4);
    assert_eq!(first_missing(&mut vec![2, 3, 4, 1]), 5);
}

