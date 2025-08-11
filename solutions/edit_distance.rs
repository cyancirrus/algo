use std::mem;

fn edit_distance(x:&str, y:&str) -> usize {
    let m = x.len();
    let mut prev:Vec<usize> = (0..=m).collect();
    let mut curr:Vec<usize> = vec![0;m+1];
    for (idx, chj) in y.chars().enumerate() {
        curr[0] = 1 + idx;
        for (idx, chi) in x.chars().enumerate() {
            curr[idx+1] = prev[idx];
            curr[idx+1] = {(
                curr[idx] + 1).min(
                prev[idx+1] + 1).min(
                prev[idx] + usize::from(chi != chj))
            }
        }
        mem::swap(&mut prev, &mut curr);
    }
    prev[m]
}

// fn edit_distance(x:&str, y:&str) -> usize {
//     let n = y.chars().count();
//     let mut prev: Vec<usize> = (0..=n).collect();
//     let mut curr: Vec<usize> = vec![0;n+1];
//     for chi in x.chars() {
//         curr[0] = prev[0] + 1;
//         for (jdx, chj) in y.chars().enumerate() {
//             if chi == chj {
//                 curr[jdx+1] = prev[jdx];
//             } else {
//                 curr[jdx+1] = 1 + {
//                     curr[jdx].min(
//                     prev[jdx+1]).min(
//                     prev[jdx])
//                 }
//             }
//         }
//         mem::swap(&mut prev, &mut curr);
//     }
//     prev[n]
// }

fn edit_distance_v1(x:&str, y:&str) -> usize {
    let m = x.len();
    let n = y.len();
    let mut dp = vec![vec![0;n+1];m+1];

    for j in 1..n {
        dp[0][j] = 1;
    }
    for i in 1..m {
        dp[i][0] = 1;
    }
    
    for (idx, chi) in x.chars().enumerate() {
        for (jdx, chj) in y.chars().enumerate() {
            if chi == chj {
                dp[idx+1][jdx+1] = dp[idx][jdx];
            } else {
                dp[idx+1][jdx+1] = 1 + {
                    dp[idx][jdx + 1].min(
                    dp[idx+1][jdx]).min(
                    dp[idx][jdx])
                }
            }
        }
    }
    dp[m][n]
}


fn main() {
    println!("edit distance {:?}", edit_distance("hello", "hello"));
    println!("edit distance {:?}", edit_distance("ros", "horse"));
    println!("edit distance {:?}", edit_distance("horse", "ros"));
    println!("edit distance {:?}", edit_distance("intention", "execution"));
}
