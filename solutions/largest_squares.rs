use std::collections::HashSet;
use std::collections::VecDeque;

fn maximal_square(grid:&[Vec<char>]) -> usize {
    if grid.is_empty() || grid[0].is_empty() { return 0; }
    let (m, n) = (grid.len(), grid[0].len());
    let mut dp = vec![vec![0;n+1];m+1];
    let mut max = 0;
    for i in 1..=m {
        for j in 1..=n {
            if grid[i-1][j-1] == '0' { continue; }
            dp[i][j] = 1 + dp[i-1][j].min(dp[i][j-1]).min(dp[i-1][j-1]);
            max = max.max(dp[i][j]);
        }
    }
    max * max
}
// fn maximal_square(grid:&[Vec<char>]) -> usize {
//     if grid.is_empty() || grid[0].is_empty() { return 0; }
//     let (m, n) = (grid.len(), grid[0].len());
//     let mut max = 0;
//     for i in 0..m {
//         for j in 0..n {
//             if grid[i][j] == '0' { continue; }
//             let mut layer = 0;
//             loop {
//                 if m <= i + layer  || n <= j + layer { break; }
//                 for k in 0..layer { if grid[i+layer][j+k] == '0' {break;} }
//                 for k in 0..=layer { if grid[i+k][j+layer] == '0' {break;} }
//                 max = max.max(layer * layer); 
//                 layer += 1;
//             }
//         }
//     }
//     max
// }

fn main() {
    println!(
        "result {:?}",
        1234
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
