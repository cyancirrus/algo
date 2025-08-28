use std::collections::HashSet;

fn combination_sum_reconstruct(nums:&[usize], target:usize) -> Vec<Vec<usize>> {
    let mut paths = vec![vec![];target+1];
    let mut res = vec![]; 
    paths[0].push((0, 0));
    for &n in nums {
        for i in 0..=target-n {
            if !paths[i].is_empty() {
                paths[i+n].push((i, n));
            }
        }
    }
    fn bt(
        idx:usize,
        bound:usize,
        paths:&Vec<Vec<(usize,usize)>>,
        coll:&mut Vec<usize>,
        res:&mut Vec<Vec<usize>>,
    ){
        if idx == 0 {
            res.push(coll.clone());
            return;
        }
        for &(pre, num) in &paths[idx] {
            if num >= bound {
                coll.push(num);
                bt(pre, num, paths, coll, res);
                coll.pop();
            }
        }
    }
    bt(target, usize::MIN, &paths, &mut vec![], &mut res);
    res
}

fn combination_sum(nums:&[usize], target:usize) -> Vec<Vec<usize>> {
    let mut res = vec![];
    fn bt(
        i:usize,
        nums:&[usize],
        curr:usize,
        target:usize,
        coll:&mut Vec<usize>,
        res:&mut Vec<Vec<usize>>
    ){
        if curr == target {
            res.push(coll.clone());
            return;
        }
        for idx in i..nums.len() {
            let n = nums[idx];
            if n + curr <= target {
                coll.push(n);
                bt(idx, nums, curr+n, target, coll, res);
                coll.pop();
            }
        }
    }
    bt(0, nums, 0, target, &mut vec![], &mut res);
    res
}


// fn combination_sum(nums:&[usize], target:usize) -> Vec<Vec<usize>> {
//     let mut seen:Vec<Vec<Vec<usize>>> = vec![vec![];target+1];
//     for &n in nums {
//         seen[n].push(vec![n]);
//         for i in 0..=target-n {
//             if !seen[i].is_empty() {
//                 let mut temp = seen[i].clone();
//                 for i in 0..temp.len() {
//                     temp[i].push(n);
//                 }
//                 seen[i+n].extend(temp);
//             }
//         }
//     }
//     seen.pop().unwrap()
// }

fn possible_combination_sum(nums:&[u8], target:u8) -> bool {
    let mut seen = vec![false;target as usize +1];
    seen[0] = true; 
    for &n in nums {
        for i in 0..=target-n {
            if seen[i as usize] {
                if i + n == target {
                    return true;
                }
                seen[(i+n) as usize] = true;
            }
        }
    }
    false
}


fn main() {
    println!("possible combo sum {:?}", possible_combination_sum(&[2,3,5], 7));
    println!("possible combo sum {:?}", combination_sum(&[2,3,5], 8));
    println!("possible combo sum {:?}", combination_sum_reconstruct(&[2,3,5], 8));
}
