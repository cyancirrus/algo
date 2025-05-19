fn min_path_sum(grid:&mut [Vec<usize>]) -> usize {
    if grid.is_empty() || grid[0].len() == 0 { return 0 };
    let m = grid.len();
    let n = grid[0].len();

    for i in 0..m {
        for j in 0..n {
            grid[i][j] += if i > 0 && j > 0 {
                grid[i-1][j].min(grid[i][j-1])
            } else if j > 0 {
                grid[i][j-1]
            } else if i > 0 {
                grid[i-1][j]
            } else {
                0
            };
        }
    }
    grid[m-1][n-1]
}


fn main() {
    assert_eq!(7, min_path_sum(&mut [vec![1,3,1],vec![1,5,1],vec![4,2,1]]));
}

