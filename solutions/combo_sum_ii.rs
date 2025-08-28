use std::collections::HashSet;

fn combination_sum_unique(nums:&mut [usize], target:usize) -> Vec<Vec<usize>> {
    nums.sort();
    let mut res = vec![];
    fn bt(
        idx:usize,
        curr:usize,
        target:usize,
        nums:&[usize],
        coll:&mut Vec<usize>,
        res:&mut Vec<Vec<usize>>
    ) {
        if curr == target {
            res.push(coll.clone());
            return;
        }
        let mut prev =  usize::MIN;
        for idx in idx..nums.len() {
            let n = nums[idx];
            if n == prev {
                continue;
            }
            if curr + n > target {
                break;
            }
            coll.push(n);
            bt(idx+1, n+curr, target, nums, coll, res); 
            coll.pop();
            prev = n;
        }
    }
    bt(0, 0, target, &nums, &mut vec![], &mut res);
    res
}

// fn combination_sum_unique(nums:&mut [usize], target:usize) -> Vec<Vec<usize>> {
//     nums.sort();
//     let mut res = vec![];
//     let mut seen = HashSet::new();
//     fn bt(
//         idx:usize,
//         curr:usize,
//         target:usize,
//         nums:&[usize],
//         seen:&mut HashSet<Vec<usize>>,
//         coll:&mut Vec<usize>,
//         res:&mut Vec<Vec<usize>>
//     ) {
//         if curr == target {
//             res.push(coll.clone());
//             return;
//         }
//         for idx in idx..nums.len() {
//             let n = nums[idx];
//             if n + curr <= target {
//                 coll.push(n);
//                 if seen.insert(coll.to_vec()){
//                     bt(idx+1, n+curr, target, nums, seen, coll, res); 
//                 }
//                 coll.pop();
//             }
//         }
//     }
//     bt(0, 0, target, &nums, &mut seen, &mut vec![], &mut res);
//     res
// }

fn main() {
    println!("combo sum {:?}", combination_sum_unique(&mut [10,1,2,7,6,1,5], 8));
}
