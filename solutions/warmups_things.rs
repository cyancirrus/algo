use std::collections::HashMap;
use std::collections::HashSet;
use std::str::from_utf8;
// use std::str::String;

fn palindrome_partitioning(s:&str) -> Vec<Vec<&str>> {
    let mut result = Vec::new();
    backtrack(s, 0, &mut vec![], &mut result);
    result
}

fn backtrack<'a>(
    s:&'a str,
    start:usize,
    path:&mut Vec<&'a str>,
    result:&mut Vec<Vec<&'a str>>
) {
    if start == s.len() {
        result.push(path.clone());
        return;
    }
    for end in start+1..=s.len() {
        if is_palindrome(s[start..end].as_bytes()) {
            path.push(&s[start..end]);
            backtrack( s, end, path, result);
            path.pop();
        }
    }
}



fn is_palindrome(s:&[u8]) -> bool {
    let n = s.len();
    for i in 0..n / 2 {
        if s[i] != s[n-1-i] {
            return false;
        }
    }
    true
}


// fn palindrome_partitioning(s:&str) -> Vec<Vec<String>> {
//     let s = s.as_bytes();
//     let mut memo = vec![];
//     let mut result = vec![];
//     worker(s, &mut vec![], &mut vec![], &mut memo);
//     for v in memo {
//         let mut collection = vec![];
//         for s in v {
//             collection.push(String::from_utf8(s).unwrap()); 
//         }
//         result.push(collection);
//     }
//     println!("Results {:?}", result);
//     result
// }

// fn worker(s:&[u8], curr:&mut Vec<u8>, prev:&mut Vec<Vec<u8>>, memo:&mut Vec<Vec<Vec<u8>>>) {
//     let match_len = curr.len();
//     if s.is_empty() && curr.is_empty() {
//         memo.push(prev.to_vec());
//         return;
//     }
//     if match_len <= s.len() {
//         if is_palindrome(&curr) && match_len > 0 {
//             let mut prev = prev.clone();
//             prev.push(curr.to_vec());
//             worker(
//                 &s[1..],
//                 &mut vec![], 
//                 &mut prev,
//                 memo
//             );
//         }
//         let mut curr = curr.clone();
//         curr.push(s[0]);
//         worker(
//             &s[match_len..], 
//             &mut curr,
//             prev,
//             memo
//         );
//     }
// }





// gas: available
// cost: cost to next station
fn gas_stations(gas:&[i32], cost:&[i32]) -> i32 {
    let mut tank = 0;
    let mut curr = 0;
    let mut sidx = 0;
    for idx in 0..gas.len() {
        let fill = gas[idx] - cost[idx];
        tank += fill;
        curr += fill;
        if curr < 0 {
            sidx = idx+1;
            curr = 0;
        }
    }
    if tank < 0 {
        return -1
    } else {
        return sidx as i32
    }
}

fn first_missing_positive(nums:&mut [i32]) -> i32 {
    let n = nums.len() as i32;
    for idx in 0..n as usize {
        if 0 < nums[idx] && nums[idx] <= n {
            nums.swap(idx as usize, nums[idx] as usize);
        }
    }
    for (idx, &n) in nums.iter().enumerate().skip(1) {
        if idx as i32 != n {
            return idx as i32;
        }
    }
    return -1
}

fn coin_change(coins:&[usize], target:usize) -> usize {
    let n = target + 1;
    let mut sums = vec![0;n];
    sums[0]=1;
    for &c in coins {
        if c > target {
            continue;
        }
        for amount in 0..n {
            if c + amount  <= target {
                sums[c + amount] += sums[amount];
            }
        }
    }
    sums[target]
}

type Collection = HashMap<u32, Vec<Vec<u32>>>;
fn merge(base:&mut Collection, aux:&mut Collection) {
    for (k,mut v) in  aux.drain() {
        if let Some(entry) = base.get_mut(&k) {
            entry.extend(v);
        } else {
            base.insert(k, v);
        }
    }
}


fn candidates(nums: &[u32], target:u32) -> Vec<Vec<u32>> {
    let mut memo:HashMap<u32,Vec<Vec<u32>>> = HashMap::new();
    let mut seen:HashSet<u32> = HashSet::new();
    for &n in nums {
        if let Some(_) = seen.get(&n) {
            continue;
        }
        seen.insert(n);
        memo.entry(n).or_default().push(vec![n]);
        let mut current: HashMap<u32,Vec<Vec<u32>>> = HashMap::new();
        for (k, v) in &memo {
            let mut components = v.clone();
            if k + n <= target {
                for v in &mut components {
                    v.push(n);
                }
                current.entry(k+n).or_default().extend(components);
            }
        }
        merge(&mut memo, &mut current);
    }
    memo.remove(&target).unwrap_or(vec![])
}


// max multiplicity = 2
fn remove_duplicates(nums:&mut [u32]) -> &[u32] {
    if nums.is_empty() {
        return nums
    }
    // index of thing
    let mut pivot= 0;
    let mut multiplicity = 1;
    for idx in 1..nums.len() {
        if nums[idx] == nums[pivot] {
            if multiplicity == 1 {
                multiplicity += 1;
                pivot += 1;
                nums.swap(pivot, idx);
            }
        } else {
            pivot+=1;
            nums.swap(pivot, idx);
            multiplicity = 1;
        }
    }
    &nums[0..=pivot]
}

fn main() {
    // println!("how does this go {:?}", remove_duplicates(&mut [1,1,1,2,2,3]));
    // println!("how does this go {:?}", remove_duplicates(&mut [1,1,1,2,2,2,3]));
    // println!("how does this go {:?}", remove_duplicates(&mut [1,1,1,1,1,1,1]));
    // println!("how does this go {:?}", remove_duplicates(&mut [1,2,3,4]));
    // println!("how does this go {:?}", remove_duplicates(&mut [1]));
    // println!("how does this go {:?}", remove_duplicates(&mut []));

    // println!("candidates {:?}", candidates(&[2,5,2,1,2], 5));
    // println!("candidates {:?}", candidates(&[2,5,2,1,2], 5));
    // println!("coin change {:?}", coin_change(&[1,5,10], 32));
    // println!("first missing {:?}", first_missing_positive(&mut [1,4,2,5,]));
    // println!("gas {:?}", gas_stations(&[1,2,3,4,5], &[3,4,5,1,2]));
    // println!("gas {:?}", gas_stations(&[2,3,4],&[3,4,3]));
    println!("palindrome part {:?}", palindrome_partitioning("aab"));
}
