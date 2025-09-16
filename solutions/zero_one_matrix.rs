use std::collections::HashSet;
use std::collections::VecDeque;

fn zero_one(grid:&[Vec<usize>]) -> Vec<Vec<usize>> {
    if grid.is_empty() || grid[0].is_empty() { return vec![] };
    let (m, n) = (grid.len(), grid[0].len()); 
    let mut distances = vec![vec![m * n;n];m];
    // check left and up and self
    for i in 0..m {
        for j in 0..n {
            if grid[i][j] == 1 { distances[i][j] = 0; continue; }
            if i > 0 { distances[i][j] = distances[i][j].min(distances[i-1][j] + 1); }
            if j > 0 { distances[i][j] = distances[i][j].min(distances[i][j-1] + 1); }
        }
    }
    // check right and down
    for i in (0..m).rev() {
        for j in (0..n).rev() {
            if i + 1 < m { distances[i][j] = distances[i][j].min(distances[i+1][j] + 1); }
            if j + 1 < n { distances[i][j] = distances[i][j].min(distances[i][j+1] + 1); }
        }
    }
    distances
}


fn main() {
    println!(
        "result {:?}",
        // zero_one(&[vec![0,0,0],vec![0,1,0],vec![1,1,1]])
        zero_one(&[vec![0,0,0],vec![0,1,0],vec![1,1,1]])
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
