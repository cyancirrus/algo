fn palindrome_partitioning(s:&str) -> usize {
    if s.is_empty() { return 0 };
    let mut dp = vec![usize::MAX;s.len()];
    backtrack(s, 0, 0, &mut s.len(), &mut dp)
}

fn backtrack(
    s:&str,
    start:usize,
    cuts:usize,
    min:&mut usize,
    dp:&mut [usize]
) -> usize {
    if dp[start] < usize::MAX {
        return dp[start]
    }
    if cuts >= *min {
        return cuts
    }
    let mut min_cuts = usize::MAX;
    for end in (start + 1..=s.len()).rev() {
        if is_palindrome(s[start..end].as_bytes()) {
            let new_cuts = if end == s.len() {
                *min = (*min).min(cuts);
                0
            } else {
                1+backtrack(s, end, cuts+1, min, dp)
            };
            min_cuts = min_cuts.min(new_cuts);
        }
    }
    dp[start] = min_cuts;
    min_cuts
}

// fn palindrome_partitioning(s:&str) -> usize {
//     let mut min_cuts = s.len();
//     backtrack(s, 0, &mut 0, &mut min_cuts);
//     min_cuts
// }

// fn backtrack(s:&str, start:usize, cuts:&mut usize, min:&mut usize) {
//     for end in (start + 1..=s.len()).rev() {
//         if is_palindrome(s[start..end].as_bytes()) {
//             if end == s.len() {
//                 *min = *min.min(cuts);
//             } else if *cuts < *min {
//                 *cuts += 1;
//                 backtrack(s, end, cuts, min);
//                 *cuts -= 1;
//             }
//         }
//     }
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
    println!("partitioning {:?}", palindrome_partitioning("hello"));
    println!("partitioning {:?}", palindrome_partitioning("aab"));
    println!("partitioning {:?}", palindrome_partitioning("a"));
    println!("partitioning {:?}", palindrome_partitioning("ab"));
    println!("partitioning {:?}", palindrome_partitioning(""));
    println!("partitioning {:?}", palindrome_partitioning("aaaa"));
    println!("partitioning {:?}", palindrome_partitioning("abccbadz"));
    println!("partitioning {:?}", palindrome_partitioning("aasdlkfjajjhdfjhjuhewkbccbadz"));
}
