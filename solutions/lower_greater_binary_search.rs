
// NOTE: want to do a relaxed read and pathsum ii looks interesting for lnr and queue for tree

// fn bs(nums:&[u32], target:u32) -> i32 {
//     let (mut l, mut h) = (0, nums.len().saturating_sub(1));

//     while l <= h {
//         let m = ( l + h )/ 2;
//         if nums[m] == target {
//             return m as i32;
//         }
//         else if nums[m] < target {
//             l = m+1;
//         }
//         else if m == 0 {
//             break;
//         }
//         else if nums[m] > target  {
//             h = m-1;
//         }
//     }
//     -1
// }
// fn bs_closest(nums:&[u32], target:u32) -> isize {
//     let (mut l, mut h) = (0, nums.len().saturating_sub(1));
//     while l <= h {
//         let m = ( l + h ) / 2;
//         if nums[m] == target {
//             return m as isize;
//         }
//         else if l == h {
//             if nums[m] < target {
//                 return (m + 1) as isize;
//             } else {
//                 return (m - 1) as isize;
//             }
//         }
//         else if nums[m] < target {
//             l = m+1;
//         }
//         else if m == 0 {
//             break;
//         }
//         else if nums[m] > target  {
//             h = m;
//         }
//     }
//     -1
// }


// fn search_range(nums:&[u32], target:u32) -> (isize, isize) {
//     let lower = bs_closest(nums, target.saturating_sub(1));
//     let upper = bs_closest(&nums[( lower  + 1 ) as usize..], target+1);
//     return (lower + 1, upper + lower + 1 )
// }


fn lower_bound(nums:&[u32], target:u32) -> isize {
    let (mut l, mut r) = (0, nums.len());
    while l < r {
        let m = l + ( r - l ) / 2;
        if nums[m] < target {
            l = m+1;
        }
        else {
            r = m;
        }
    }
    l as isize
}

fn upper_bound(nums:&[u32], target:u32) -> isize {
    let (mut l, mut r) = (0, nums.len());
    while l < r {
        let m = l + ( r - l ) / 2;
        if nums[m] > target {
            r = m;
        } else {
            l = m+1;
        }
    }
    r as isize
}

fn search_range(nums:&[u32], target:u32) -> (isize, isize) {
    let lb = lower_bound(nums, target);
    let ub = upper_bound(nums, target);
    if lb == ub { (-1, -1) } else { (lb, ub - 1) }
}


fn main() {
// println!("lb {:?}", upper_bound(&[1,3,4,6,7,9,11],5));
// println!("lb {:?}", upper_bound(&[1,3,4,6,7,9,11],0));
// println!("bfs {:?}", bs_closest(&[1,3,4,6,7,9,11], 12));
println!("bfs {:?}", search_range(&[1,3,4,4,4,6,7,9,11], 4));
// println!("bfs {:?}", search_range(&[1,3,4,4,4,6,7,9,11], 12));
}
