use std::collections::HashMap;
// 1 3 5 7
// 1 (3 5 7)
// (1 3 5) 7
// (1 3 5 7)


// 1 3 5 7 9
// (1 3 5) 7 9
// 1 (3 5 7) 9
// 1 3 (5 7 9)
// (1 3 5 7 ) 9
// 1 (3 5 7 9)
// (1 3 5 7 9)

// fn arithmetic_slices(nums:&[i32]) -> i32 {
//     if nums.len() <= 2 { return 0 };
//     let mut res = 0;
//     let mut cnt = 2;
//     let mut diff = nums[1] - nums[0];
    
//     for i in 2..nums.len() {
//         let d = nums[i] -nums[i-1];
//         cnt += 1;
//         if d == diff && cnt >= 3 {
//             res += cnt - 2; 
//         } else {
//             cnt = 2;
//             diff = d;
//         }
//     }
//     res
// }



fn arithmetic_subsequence(nums:&[i32]) -> usize {
    let mut cnt = 0;
    fn bt(start:usize, prev:Option<i32>, diff:Option<i32>, size:usize, cnt:&mut usize, nums:&[i32]) {
        if start == nums.len() {
            *cnt += size.saturating_sub(2);
        }

        for i in start..nums.len() {
            let k = nums[i];
            match (prev, diff) {
                (None, None) => bt(start + 1, Some(k), None, 1, cnt, nums),
                (Some(p), None) => bt(start + 1, Some(k), Some(k - p), 2, cnt, nums),
                (Some(p), Some(d)) => if k - p == d {
                        bt(start + 1, Some(k), Some(k -p), size + 1, cnt, nums);
                },
                _ => {}
            }
        }
    }
    bt(0, None, None, 0, &mut cnt, nums);
    cnt
}

fn arithmetic_subsequence_memo(nums:&[isize]) -> usize {
    fn bt(start:usize, prev:isize, diff:isize, nums:&[isize], memo:&mut HashMap<(usize, isize, isize), usize>) -> usize {
        if let Some(&c) = memo.get(&(start, prev, diff)) {
            return c;
        }
        let mut cnt = 0;
        for i in start..nums.len() {
            cnt += match (prev, diff) {
                (isize::MIN, isize::MIN) => bt(i + 1, i as isize, isize::MIN, nums, memo),
                (p, isize::MIN) => bt(i + 1, i as isize, nums[i] - nums[p as usize], nums, memo),
                (p, d) => {
                    if nums[i] - nums[p as usize] == d {
                        1 + bt(i + 1, i as isize, d, nums, memo)
                    } else { 0 }
                }
            };
        }
        memo.insert((start, prev, diff), cnt);
        cnt
    }
    bt(0, isize::MIN, isize::MIN, nums, &mut HashMap::new())
}

fn main() {
    // println!("arithmetic {:?}", arithmetic_subsequence_memo(&[2,4,6,8,10]));
    // println!("arithmetic {:?}", arithmetic_subsequence_memo(&[2,4,6]));
    println!("arithmetic {:?}", arithmetic_subsequence_memo(&[3,6,9,12,15]));
    // println!("arithmetic {:?}", arithmetic_subsequence_memo(&[2,4,6,10,8,3, 8, 10, 12]));
    // println!("arithmetic slices {:?}", arithmetic_slices(&[1,2,3,4,5]));
}
