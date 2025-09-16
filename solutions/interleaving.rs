use std::collections::HashMap;



fn is_interleaving_dp(s1:&str, s2:&str, s3:&str) -> bool {
    let (sb1, sb2, sb3) = (s1.as_bytes(), s2.as_bytes(), s3.as_bytes());
    let (m, n) = (sb1.len(), sb2.len());
    let mut dp = vec![vec![false;n+1];m+1];
    dp[0][0] = true;
    if m+n != sb3.len() { return false; }
    for i in 1..=m { dp[i][0] = dp[i-1][0] && sb1[i-1] == sb3[i-1]; }
    for j in 1..=n { dp[0][j] = dp[0][j-1] && sb2[j-1] == sb3[j-1]; }
    for i in 1..=m {
        for j in 1..=n {
            dp[i][j] = {
                (dp[i-1][j] && sb1[i-1] == sb3[i + j - 1]) ||
                (dp[i][j-1] && sb2[j-1] == sb3[i + j - 1])
            };
        }
    }
    println!("dp {dp:?}");
    dp[m][n]
}
// fn is_interleaving_dp(s1:&str, s2:&str, s3:&str) -> bool {
//     let (sb1, sb2, sb3) = (s1.as_bytes(), s2.as_bytes(), s3.as_bytes());
//     let (m, n) = (sb1.len(), sb2.len());
//     let mut dp = vec![vec![false;n+1];m+1];
//     dp[0][0] = true;
//     for i in 0..=m {
//         for j in 0..=n {
//             if i + j < sb3.len() {
//                  if i < m {dp[i+1][j] |= dp[i][j] && sb1[i] == sb3[i + j];}
//                  if j < n {dp[i][j+1] |= dp[i][j] && sb2[j] == sb3[i + j];}
//             } else {
//                  if i < m {dp[i+1][j] |= dp[i][j];}
//                  if j < n {dp[i][j+1] |= dp[i][j];}
//             }
//         }
//     }
//     println!("dp {dp:?}");
//     dp[m][n]
// }



fn is_interleaving(s1:&str, s2:&str, s3:&str) -> bool {
    fn bt(
        i:usize, j:usize,
        sb1:&[u8], sb2:&[u8], sb3:&[u8]
    ) -> bool {
        if i + j == sb3.len() { return true; }
        let mut result = false;
        if i < sb1.len() && sb1[i] == sb3[i + j] {
            result |= bt(i + 1, j, sb1, sb2, sb3);
        }
        if j < sb2.len() && sb2[j] == sb3[i + j] {
            result |= bt(i, j+1, sb1, sb2, sb3);
        }
        result
    }
    bt(
        0, 0,
        s1.as_bytes(), s2.as_bytes(), s3.as_bytes()
    )
}



fn main() {
    println!(
        "result {:?}",
        is_interleaving_dp("aabcc", "dbbca", "aadbbcbcac")
        // is_interleaving_dp("aabcc", "abcd", "aabcc")
        // maximum_mul_subarray(&[2,-1])
        // can_be_valid("))()))","010100")
        // can_be_valid("))","01")
        // maximum_subarray(&[-2,1,-3,4,-1,2,1,-5,4])
    );
    // println!(
    //     "result {:?}",
    //     // maximum_mul_subarray(&[2,-1])
    //     maximum_mul_subarray(&[2,3,-2,4])
    //     // maximum_subarray(&[-2,1,-3,4,-1,2,1,-5,4])
    // );
    // println!(
    //     "result {:?}",
    //     maximum_add_subarray(&[5,4,-1,7,8])
    //     // maximum_subarray(&[-2,1,-3,4,-1,2,1,-5,4])
    // );
}
