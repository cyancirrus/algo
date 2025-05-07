use std::cmp::min;
use std::mem::swap;

// fn levenshtein(word1:&str, word2:&str) -> usize {
//     // Did not get this one on my own 
//     let m = word1.len();
//     let n = word2.len();
//     let w1:Vec<char> = word1.chars().collect();
//     let w2:Vec<char> = word2.chars().collect();
    
//     let mut dp = vec![vec![0;n + 1]; m + 1];
//     for i in 0..m {
//         dp[i][0] = i; // deleting all of w1
//     }
//     for j in 0..n {
//         dp[0][j] = j;
//     }

//     for i in 1..=m {
//         for j in 1..=n {
//             if w1[i-1] == w2[j-1] {
//                 dp[i][j] = dp[i-1][j-1]; // matches advance
//             } else {
//                 dp[i][j] = 1 + std::cmp::min(
//                         dp[i-1][j-1], // insert
//                         std::cmp::min(
//                             dp[i - 1][j], // delete left
//                             dp[i][j-1], // delete right
//                         )
//                 )
//             }
//         }
//     }

//     dp[m][n]
// }

// fn levenshtein(word1:&str, word2:&str) -> usize {
//     let m = word1.len() + 1;
//     let n = word2.len() + 1;
//     let w1:&[u8] = word1.as_bytes();
//     let w2:&[u8] = word2.as_bytes();
    
//     let mut dp = vec![0;m * n];
//     for i in 1..m {
//         dp[i * n] = i; // deleting all of w1
//     }
//     for j in 1..n {
//         dp[j] = j; // inserting all of w2
//     }
//     for i in 1..m {
//         for j in 1..n {
//             if w1[i-1] == w2[j-1] {
//                 dp[i * n + j] = dp[(i-1) * n + j - 1]; // matches advance
//             } else {
//                 dp[i * n + j] = 1 + std::cmp::min(
//                         dp[(i-1) * n + j - 1], // replace
//                         std::cmp::min(
//                             dp[(i - 1) * n + j], // delete left
//                             dp[i * n + j - 1 ], // insert right
//                         )
//                 );
//             }
//         }
//     }
//     dp[m * n - 1]
// }

fn levenshtein(word1:&str, word2:&str) -> usize {
    let m = word1.len() + 1;
    let n = word2.len() + 1;
    let w1:&[u8] = word1.as_bytes();
    let w2:&[u8] = word2.as_bytes();
    let mut cur= vec![0;n];
    let mut pre= vec![0;n];
    for j in 1..n {
        pre[j] = j;
    }
    for i in 1..m {
        cur[0] = 1;
        for j in 1..n {
            if w1[ i - 1 ] == w2[ j - 1 ] {
                cur[ j ] = pre[ j - 1 ]; // matches advance
            } else {
                cur[ j ] = 1 + min(
                    // if statement for incrementing row index
                    pre[ j - 1 ] , // replace
                        min(
                            pre[ j ], // delete left
                            cur[ j - 1 ], // insert right
                        )
                );
            }
            swap(&mut cur, &mut pre)
        }
    }
    pre[ n - 1 ]
}


fn main() {
    assert_eq!(4, levenshtein("hello", "world"), "basic case");
    assert_eq!(1, levenshtein("orose", "rose"), "orose rose");
}
