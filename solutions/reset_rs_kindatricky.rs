use std::collections::HashMap;
use std::mem;

// take or don't take
// 1 2 3 4
// 1 2 3 4 5
// 10 1 3 12 5

fn maxproduct(nums:&[i32]) -> i32 {
    if nums.len() <= 1 { return i32::MIN; }
    let mut max = i32::MIN;
    let mut s1 = nums[0];
    let mut s2 = nums[0];
    let mut reset = false;
    for &num in &nums[1..] {
        if num == 0 {
            reset = true;
            s1 *= num;
            s2 *= num;
        } else if reset {
            s1 = num; s2 = num;
            reset = false;
            continue;
        } else {
            s1 *= num;
            s2 *= num;
            reset = false;
        }
        max = max.max(s1).max(s2);
    }
    max
}


// fn maxproduct(nums:&[i32]) -> i32 {
//     let mut max:i32 = i32::MIN;
//     let mut pos:Option<i32> = None;
//     let mut neg:Option<i32> = None;
//     let mut len = 0;
//     for &num in nums.iter() {
//         len += 1;
//         match (num, &mut pos, &mut neg) {
//             (0, _, _ ) => {
//                 pos = None;
//                 neg = None;
//                 len = 0;
//             },
//             (_, None, None) => {
//                 pos = Some(num);
//                 if num < 0 { mem::swap(&mut pos, &mut neg); }
//             },
//             (_, None, Some(n)) => {
//                 *n *= num;
//                 if num < 0 { mem::swap(&mut pos, &mut neg); }
//             },
//             (_, Some(p), None) => {
//                 *p *= num;
//                 if num < 0 { mem::swap(&mut pos, &mut neg); }
//             },
//             (_, Some(p), Some(n)) => {
//                 *p *= num;
//                 *n *= num;
//                 if num < 0 { mem::swap(&mut pos, &mut neg); }
//             }
//         }
//         match (len, pos, neg) {
//             (0, _, _) | (1, _, _) => { },
//             (_, Some(ref p), Some(ref n)) => { max = max.max(*p).max(*n) },
//             (_, Some(ref p), None) => { max = max.max(*p); },
//             (_, None, Some(ref n)) => { max = max.max(*n) },
//             (_, _, _) => {},
//         }
//     }
//     max
// }


fn takemaxdp(nums:&mut [u32]) -> u32 {
    if nums.len() == 0 { return 0; }
    if nums.len() == 1 { return nums[0]; }
    if nums.len() == 2 { return nums[0].max(nums[1]); }
    for i in (0..=nums.len()-3).rev() {
        nums[i] = nums[i+1].max(nums[i] + nums[i + 2]);
    }
    nums[0]
}

fn takemax(nums:&[u32]) -> u32 {
    fn bt(i:usize, nums:&[u32], memo:&mut HashMap<usize, u32>) -> u32 {
        if i >= nums.len() {
            return 0;
        }
        if let Some(&r) = memo.get(&i) {
            return r;
        }
        let take = nums[i] + bt(i+2, nums,memo);
        let skip = bt(i+1, nums, memo);
        let total = take.max(skip);
        memo.insert(i, total);
        total
    }
    bt(0, nums, &mut HashMap::new())
}

fn main() {
    // println!("test {:?}", takemaxdp(&mut [3,2,5,10,7]));
    println!("test max {:?}", maxproduct(&[2, 3, -4, 32]));
    // println!("test max {:?}", maxproduct(&[-2,0,1]));
    // println!("test max {:?}", maxproduct(&[2, 3, 0, 0, 10,10, -2,-4]));
    // println!("test max {:?}", maxproduct(&[2, 3]));
    // println!("test {:?}", takemax(&[2,7,9,6,1]));
    // println!("test {:?}", takemax(&[2,7,9,3]));
    // println!("test {:?}", takemax(&[2,7,9,6]));
}
