use std::collections::HashMap;

// fn three_sum(nums:&[i32]) -> Option<[usize;3]> {
//     let mut vals: HashMap<i32,usize> = HashMap::new();
//     let mut two_vals: HashMap<i32,[usize;2]> = HashMap::new();

//     for i in 0..nums.len() {
//         vals.insert(nums[i], i);

//         if let Some(pair) = two_vals.get(&(-nums[i])) {
//             // unpack the 2 array
//             return Some([pair[0], pair[1], i]);
//         }
//         for (&k, &v) in vals.iter() {
//             two_vals.insert(nums[i] + k, [v, i]);
//         }
//     }
//     None
// }

fn three_sum_space(target:i32, nums:&mut [i32]) -> Option<[i32;3]> {
    // O(nlogn) << O(n^3)
    nums.sort();
    let len = nums.len();
    for i in 0..len - 2{
        if nums[i] + nums[i+1] + nums[i+2] > target {
            return None;
        }
        for j in i+1..len -1  {
            // all considered results would be too large
            if nums[i] + nums[j] + nums[j+1] > target {
                break; // nothing ahead will be better
            }
            let psum = nums[i] + nums[j];
            let search = binary_search(target - psum, &nums[j+1..]);
            if let Some(result) = search {
                return Some([nums[i], nums[j], result])
            }
        }
    }
    None
}
fn binary_search(target:i32, nums:&[i32]) -> Option<i32> {
    let len = nums.len();
    if len == 0 { return None};
    let (mut l, mut u) = (0, nums.len() - 1);
    while l <= u {
        let c = (l + u) / 2;
        let num = nums[c];
        if target == num {
            return Some(target)
        }
        if num < target {
            l= c + 1;
        } else if num > target {
            u= c - 1;
        }
    }
    None
}

fn three_sum(nums:&[i32]) -> Option<[usize;3]> {
    let len = nums.len();
    let mut pairs: HashMap<i32,[usize;2]> = HashMap::with_capacity((len^2 + 1)/2);
    for i in 0..len {
        if let Some(pair) = pairs.get(&-nums[i]) {
            return Some([pair[0], pair[1], i]);
        }
        for j in 0..i {
            pairs.insert(nums[i] + nums[j], [j, i]);
        }
    }
    None
}

fn four_sum(nums:&[i32]) -> Option<[usize;4]> {
    let mut trips: HashMap<i32,[usize;3]> = HashMap::new();

    for i in 0..nums.len() {
        if let Some(trip) = trips.get(&-nums[i]) {
            return Some([trip[0], trip[1], trip[2], i]);
        }
        for j in 0..i {
            for k in 0..j {
                // insert indices in appending order
                trips.insert(nums[i] + nums[j] + nums[k], [k, j, i]);
            }
        }

    }
    None
}

fn n_sum(n:usize, target:i32, nums:&[i32]) -> Option<Vec<usize>> {
    if n < 1 || nums.len() < n {return None};

    for i in 0..nums.len() {
        let num = nums[i];
        let missing = target - num;
        if n == 1 {
            if num == target {
                return Some(vec![i])
            }
            continue;
        }
        if let Some(mut sub_result) = n_sum(n - 1, missing, &nums[0..i]) {
            sub_result.push(i);
            return Some(sub_result)

        }
    }
    None
}


fn main() {
    assert_eq!(Some([0,1,3]), three_sum(&vec![0, 2, 4, -2]));
    assert_eq!(None, three_sum(&vec![0, 2, 4, -5]));
    assert_eq!(Some([1,2,4,5]), four_sum(&vec![0, 1, 2, 4, -2, -1]));
    assert_eq!(None, four_sum(&vec![0, 2, 4, -2, -1]));

    println!("Test for thing {:?}", n_sum(4, 0, &vec![0, 2, 4, -2, -1, 1]));
}
