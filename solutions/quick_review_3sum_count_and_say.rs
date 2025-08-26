use std::mem;

fn count_and_say(n:usize) -> String {
    let mut curr = vec![1];
    let mut next = vec![];
    for _ in 0..n-1 {
        let mut prev = curr[0];
        let mut cnt = 1;
        for &n in &curr[1..] {
            if n == prev {
                cnt += 1;
            } else {
                next.push(cnt);
                next.push(prev);
                prev = n;
                cnt = 1;
            }
            
        }
        next.push(cnt);
        next.push(prev);
        curr.clear();
        mem::swap(&mut curr, &mut next);
    }
    curr.iter().map(|d| d.to_string()).collect::<String>()
}
// sorting first seems smart
// a + b + c ~= t
// if in sorted order if a + b + n > target (break, ur not going to get closer)
// maybe i can three pointer technique? i < j < k

fn two_sum_closest(nums:&mut [isize], target:isize) -> isize {
    nums.sort();
    let n = nums.len();
    let mut error = isize::MAX;
    let mut closest = 0;
    let (mut l, mut h) = (0, n-1);
    while l < h { 
        let sum = nums[l] + nums[h];
        let err = (target - sum).abs();
        if err < error {
            error = err;
            closest = sum;
        }
        if sum < target {
            l+=1;
        } else {
            h-=1;
        }
    }
    closest
}

fn three_sum_closest(nums:&mut [isize], target:isize) -> isize {
    nums.sort();
    let n = nums.len();
    let mut error = isize::MAX;
    let mut closest = 0;
    for idx in 0..n-2 {
        let (mut l, mut h) = (idx+1, n-1);
        while l < h {
            let sum = nums[idx] + nums[l] + nums[h];
            let err = (target - sum).abs();
            if err < error {
                error = err;
                closest = sum;
            }
            if sum < target {
                l+=1;
            } else {
                h-=1;
            }
        }
    }
    closest
}

// fn three_sum_closest(nums:&mut [isize], target:isize) -> isize {
//     nums.sort();
//     let n = nums.len();
//     let mut error = isize::MAX;
//     let mut closest = 0;
//     // maybe do a btree here?
//     // let mut memo = vec![isize::MIN; n * (n - 1)];
//     for i in 0..n {
//         for j in i+1..n {
//             for k in j+1..n {
//                 let sum = nums[i] + nums[j] + nums[k];
//                 let err = target - sum;
//                 if err.abs() < error {
//                     error = err.abs();
//                     closest = sum;
//                 } else if err < 0 {
//                     break;
//                 }
//             }
//         }
//     }
//     closest
// }
// fn three_sum_closest(nums:&mut [isize], target:isize) -> isize {
//     nums.sort();
//     let n = nums.len();
//     let mut error = isize::MAX;
//     let mut closest = 0;
//     for i in 0..n {
//         for j in i+1..n {
//             for k in j+1..n {
//                 let sum = nums[i] + nums[j] + nums[k];
//                 let err = target - sum;
//                 if err.abs() < error {
//                     error = err.abs();
//                     closest = sum;
//                 }
//             }
//         }
//     }
//     closest
// }

fn main() {
    // println!("count and say {:?}", count_and_say(4));
}
