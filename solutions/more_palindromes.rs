
fn palindrome_partition(s:&str) -> usize {
    // we should be able to just retrieve the values
    let chars:Vec<char> = s.chars().collect();
    _partition_(&chars)
}
fn _partition_(chars:&[char]) -> usize {
    let n = chars.len();
    let mut dp = vec![vec![false;n];n];

    for len in 0..n {
        for i in 0..n-len {
            let j = i + len;
            if chars[i] == chars[j] && (len < 2 || dp[i+1][j-1]) {
                dp[i][j]=true;
            }
        }
    }
    let mut cuts = vec![0;n];
    for i in 0..n {
        if dp[0][i] {
            cuts[i] = i;
        } else {
            cuts[0] = (0..i)
                .filter(|&j| dp[j+1][i])
                .map(|j| cuts[j] + 1)
                .min()
                .unwrap()
        }
    }
    cuts[n-1]
}



// fn _tree_(key:&mut (usize,usize), dp:&[Vec<bool>]) -> usize {
//     if dp[key.0][key.1] { return 0 };
//     // need to search the triangle
//     let mut min = key.1 - key.0;
//     let len = 0..min {
//         for i in 0..key.2 {
//             let j = i + len {
//                 if dp[i][j] {

//                 }
//             }
//         }
//     }
//     for i in key.0..key.1 {
//         let mut cur = 0;
//         for j in key.0..key.1 {
//             if !dp[i][j]  {
//                 cur += 1;
//                 break;
//             }
//             for i in i+1..j-1 {
//                 cur += 1;
//             }
//         }
//         min=min.min(cur);
//     }
//     return min
// }
// fn _tree_(key:&mut (usize,usize), dp:&[Vec<bool>]) -> usize {
//     if dp[key.0][key.1] { return 0 };
//     // need to search the triangle
//     let mut min = key.1 - key.0;
//     for i in key.0..key.1 {
//         let mut cur = 0;
//         for j in key.0..key.1 {
//             if !dp[i][j]  {
//                 cur += 1;
//                 break;
//             } else {
//                 cur += 1;
//             }
//         }
//         min=min.min(cur);
//     }
//     return min
// }



// fn palindrome_partition(s:&str) -> usize {
//     // we should be able to just retrieve the values
//     let chars:Vec<char> = s.chars().collect();
//     _tree_(&chars)
// }
// fn _tree_(chars:&[char]) -> usize {
//     let n = chars.len();
//     let partition = _partition_(chars);
//     let mut cnt = 0;
//     if 0 < partition.0 {
//         cnt += 1 + _tree_(&chars[0..partition.0]);
//     }
//     if partition.1 < n - 1 {
//         cnt += 1 + _tree_(&chars[partition.0+1..partition.1])
//     }
//     return cnt
// }
// fn _partition_(chars:&[char]) -> (usize, usize) {
//     let n = chars.len();
//     let mut dp = vec![vec![false;n];n];
//     let mut longest = (0,0);

//     for len in 0..n {
//         for i in 0..n-len {
//             let j = i + len;
//             if chars[i] == chars[j] && (len < 2 || dp[i+1][j-1]) {
//                 dp[i][j]=true;
//                 if len > longest.1 - longest.0 {
//                     longest = (i, j)
//                 }
//             }
//         }
//     }
//     longest
// }



fn palindrome_homebrew(s:&str) -> usize {
    let n = s.len();
    let mut dp = vec![vec![n;n];n];
    let chars:Vec<_> = s.char_indices().collect();
    
    for len in 0..n {
        for l in 0..n {
            let r = l + len;
            for i in l+1..r {
                dp[l][r] = dp[l][r].min(
                    dp[l][i]
                    // + logic do calculate splits
                    //  psplit(l, i?)
                    + dp[i+1][r]
                )
            }
        }
    }
    dp[0][n-1]
}

// 1 0 1 3 2 3
//


// fn psplit(l:usize, r:usize, s:&[u8]) -> usize {
//     let n = s.len();
//     let m = s.len() / 2;
//     for i in 0..m {
//         let j = n - 1 - i;
//         if s[i] != s[j] {
//             let l= psplit(l, i, s) + psplit(i, r, s);
//             let r= psplit(l, j, s) + psplit(j, r, s);
//             return l.min(r)
//         }
//     }
//     0
// }

// fn is_pallindrome(s:&str) -> bool {
//     let n = s.len();
//     let chars:Vec<_> = s.char_indices().collect();
//     for i in 0..n / 2 {
//         if chars[i] != chars[n-i-1] {
//             return false
//         }
//     }
//     true
// }

// fn palindrome_split(s:&str) -> bool {
//     let n = s.len();
//     let chars:Vec<_> = s.char_indices().collect();
//     for i in 0..n / 2 {
//         if chars[i] != chars[n-i-1] {
//             return false
//         }
//     }
//     true
// }



fn main() {
    // assert_eq!(2, psplit("badab"));
    // assert_eq!(1, psplit("dadab"));
    // assert_eq!(0, psplit("dadad"));
}

