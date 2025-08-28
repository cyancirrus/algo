use std::mem;

fn unique_paths_iii(grid:&[Vec<isize>]) -> usize {
    if grid.is_empty() || grid[0].is_empty() { return 0; }
    let m = grid.len();
    let n = grid[0].len();
    let mut pathing = vec![vec![false;n];m];
    let mut start:(usize, usize) = (0,0);
    let mut squares = 0;
    for i in 0..m {
        for j in 0..n {
            match grid[i][j] {
                1 => { start = (i, j); },
                0 => { squares += 1; }
                _ => {}
            }
        }
    }
    fn bt(
        x:usize, y:usize,
        m:usize, n:usize,
        count:usize,
        squares:usize,
        pathing:&mut Vec<Vec<bool>>,
        grid:&[Vec<isize>],
    ) -> usize {
        let mut total = 0; 
        for (dx, dy) in [(1,0),(0,1),(!0,0),(0,!0)] {
            let nx = x.wrapping_add(dx);
            let ny = y.wrapping_add(dy);
            if nx < m && ny < n && !pathing[nx][ny]{
                match grid[nx][ny] {
                    0 => {
                        pathing[nx][ny] = true;
                        total += bt(nx,ny, m, n, 1+count, squares, pathing, grid);
                        pathing[nx][ny] = false;
                    },
                    2 => { if count == squares { return 1; } }
                    _ => { continue; }
                }
            }
        }
        total
    }
    bt(start.0, start.1, m, n, 0, squares, &mut pathing, grid)
}

fn unique_paths_ii(grid:&[Vec<usize>]) -> usize {
    if grid.is_empty() || grid[0].is_empty() { return 0; }
    if grid[0][0] == 1 { return 0; }
    let m = grid.len();
    let n = grid[0].len();
    let mut dp = vec![0;n];
    dp[0]=1;

    for x in 0..m {
        for y in 0..n {
            if x == 0 && y == 0 { continue; }
            if grid[x][y] == 1 { dp[y] = 0; continue; }
            if y != 0 { dp[y] += dp[y-1]; }
        }
    }
    dp[n-1]
}


// fn unique_paths(m:usize, n:usize) -> usize {
//     let mut dp = vec![1;n];
//     for _ in 1..m {
//         for y in 1..n {
//             dp[y] += dp[y-1];
//         }
//     }
//     dp[n-1]
// }
// fn unique_paths(m:usize, n:usize) -> usize {
//     let mut dp = vec![vec![1;n];m];
    
//     for x in 1..m {
//         for y in 1..n {
//             let mut sum = 0;
//             for (dx, dy) in [(!0,0),(0,!0)] {
//                 let nx = x.wrapping_add(dx);
//                 let ny = y.wrapping_add(dy);
//                 if nx < m && ny < n {
//                     sum += dp[nx][ny];
//                 }
//             }
//             dp[x][y] = sum;
//         }
//     }
//     println!("dp {dp:?}");
//     dp[m-1][n-1]


// }


fn main() {
    // println!("unique paths {:?}", unique_paths_iii(&[vec![1,0,0,0],vec![0,0,0,0],vec![0,0,2,-1]]));
    println!("unique paths {:?}", unique_paths_iii(&[vec![1,0,0,0],vec![0,0,0,0],vec![0,0,0,2]]));
    // println!(" unique paths {:?}", unique_paths_ii(&[vec![0,0,0],vec![0,1,0],vec![0,0,0]]));
    // println!("unique paths {:?}", unique_paths(3, 7));
}
