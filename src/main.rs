fn palindrome_three_partition(s:&str) -> bool {
    let n = s.len();
    let s_bytes = s.as_bytes();
    let mut memo = vec![vec![false;n];n];
    for i in 0..n {
        for j in i..n {
            // result for like one character memo[i][i] why i..=j
            memo[i][j] = is_palindrome_no_heap(s_bytes, i, j);
        }
    }

    for lower in 0..n-2 {
        for mid in lower+1..n-1 {
            if memo[0][lower] && memo[lower+1][mid] && memo[mid+1][n-1] {
                return true
            }
        }
    }
    false
}

fn is_palindrome_no_heap(s:&[u8], i:usize, j:usize) -> bool {
    let n = j - i + 1;
    for idx in 0..n/2 {
        if s[ idx + i ] != s[ j - idx ] {
            return false
        }
    }
    true
}


fn palindrome_partitioning(s:&str) -> bool {
    partitioning(s, 3)
}

fn partitioning(s:&str, target:usize) -> bool {
    let mut dp = vec![vec![false;s.len()+1];s.len()+1];
    let (success, _) = backtrack(s, 0, 0, target, &mut dp);
    success
}

fn backtrack(
    s:&str,
    start:usize,
    splits:usize,
    target:usize,
    dp:&mut [Vec<bool>],
) -> (bool, usize) {
    if start == s.len() {
        return (splits==target, splits)
    }
    if target > splits {
        let need = target - splits;
        if dp[start][need] {
            return (true, start + need);
        }
    }
    for end in start+1..=s.len() {
        if is_palindrome(s[start..end].as_bytes()) {
            let (ok, cumulative_splits) = backtrack(s, end, splits + 1, target, dp);
            if ok {
                return (ok, cumulative_splits);
            } else if cumulative_splits > 0 {
                dp[ start ][ cumulative_splits - splits ] = true;
            }
        }
    }
    (false, 0)
}



// fn partitioning(s:&str, target:usize) -> bool {
//     let mut dp = vec![0;s.len()];
//     let (success, _) = backtrack(s, 0, 0, target, &mut dp);
//     println!("dp {dp:?}");
//     success
// }

// fn backtrack(
//     s:&str,
//     start:usize,
//     splits:usize,
//     target:usize,
//     dp:&mut [usize],
// ) -> (bool, usize) {
//     if start == s.len() {
//         return (splits == target, splits)
//     }
//     if target > splits {
//         let need = target - splits;
//         if verify_target_bit(dp[start], need) {
//             return (true, target);
//         }
//     }
//     let mut found_splits:usize = 0;
//     for end in start + 1..=s.len() {
//         if is_palindrome(s[start..end].as_bytes()) {
//             let (ok, csplits) = backtrack(s, end, splits + 1, target, dp);
//             if ok {
//                 return (true, csplits)
//             } else if csplits  > 0 {
//                 found_splits |= store_target_bit(csplits - splits);
//             }
//         }
//     }
//     dp[start] |= found_splits;
//     (false, 0)
// }

fn store_target_bit(target: usize)  -> usize {
    if target == 0 || target > 3 {
        return 0 
    }
    1 << (target - 1)
}

fn verify_target_bit(bits:usize, target:usize) -> bool {
    if target == 0 || target > 3 {
        return false;
    }
    ((bits >> (target - 1)) & 1) == 1
}

// fn backtrack(
//     s:&str,
//     start:usize,
//     splits:usize,
//     target:usize,
// ) -> bool {
//     if start == s.len() {
//         return splits == target + 1 
//     }

//     for end in start + 1..=s.len() {
//         if is_palindrome(s[start..end].as_bytes()) {
//             if backtrack(s, end, splits + 1, target, dp) {
//                 return true;
//             }
//         }
//     }
//     false
// }




fn is_palindrome(s:&[u8]) -> bool {
    let n = s.len();
    for i in 0..n/2 {
        if s[i] != s[n-i-1] {
            return false;
        }
    }
    true
}
fn main() {
    // println!("partitioning {:?}", palindrome_partitioning("hello"));
    println!("partitioning {:?}", palindrome_partitioning("aacd"));
    println!("partitioning {:?}", palindrome_partitioning("abc"));
    println!("partitioning {:?}", palindrome_three_partition("dabc"));
    // println!("partitioning {:?}", palindrome_partitioning("a"));
    // println!("partitioning {:?}", palindrome_partitioning("ab"));
    // println!("partitioning {:?}", palindrome_partitioning(""));
    // println!("partitioning {:?}", palindrome_partitioning("bbaaaabb"));
    // println!("partitioning {:?}", palindrome_partitioning("aabckccfj"));
    // println!("partitioning {:?}", palindrome_partitioning("abccbadz"));
    // println!("partitioning {:?}", palindrome_partitioning("aasdlkfjajjhdfjhjuhewkbccbadz"));
}
