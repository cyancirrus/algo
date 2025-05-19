use std::mem;

fn unique_obstacle_paths(grid:&[Vec<usize>]) -> usize {
    if grid.is_empty() || grid[0].is_empty() { return 0; }
    let m = grid.len();
    let n = grid[0].len();
    
    let mut dp = vec![0;n];
    dp[0] = 1;
    for i in 0..m {
        for j in 0..n {
            if grid[i][j] == 1 {
                dp[j]= 0;
            } else if j > 0 {
                dp[j] += dp[j-1];
            }
        }
    }
    dp[n-1]
}


// fn unique_obstacle_paths(grid:&[Vec<usize>]) -> usize {
//     if grid.is_empty() || grid[0].is_empty() { return 0; }
//     let m = grid.len();
//     let n = grid[0].len();
    
//     // initialized to one but goes through initialization loop
//     let mut prev = vec![1;m];
//     let mut curr = vec![0;m];
//     for i in 1..m {
//         if grid[m-i][i] == 1 {
//             prev[i] = 0;
//         } else {
//             prev[i] = prev[i-1];
//         }
//     }
//     // solving from bottom to the top
//     for i in (0..m-1).rev() {
//         // grid index is reversed but so is index
//         for j in 0..n {
//             // maps from 1 <-> 0
//             let robot_0 = grid[i][j]^1 as usize;
//             curr[j] = if j+1 == n {
//                 prev[j]
//             } else {
//                 curr[j+1] + prev[j]
//             };
//             curr[j] = curr[j] * robot_0;
//         }
//         mem::swap(&mut prev, &mut curr);
//     }
//     prev[0]
// }

fn main() {
    assert_eq!(1, unique_obstacle_paths(&[vec![0,1],vec![0,0]]));
    assert_eq!(2, unique_obstacle_paths(&[vec![0,0,0],vec![0,1,0],vec![0,0,0]]));
    assert_eq!(0, unique_obstacle_paths(&[vec![1,0],vec![0,0]]));
    assert_eq!(0, unique_obstacle_paths(&[]));
}



// fn unique_obstacle_paths(grid:&[Vec<usize>]) -> usize {
//     // hopefully short circuilts
//     if grid.is_empty() || grid[0].is_empty() { return 0; }
//     let m = grid.len();
//     let n = grid[0].len();
    
//     // initialized to one but goes through initialization loop
//     let mut prev = vec![1;m];
//     let mut curr = vec![0;m];
//     for i in 1..m {
//         if grid[m-i][i] == 1 {
//             prev[i] = 0;
//         } else {
//             prev[i] = prev[i-1];
//         }
//     }
//     println!("Prev {:?}", prev);
//     println!("Curr {:?}", curr);
//     println!("-------");
//     for i in (0..m-1).rev() {
//         println!("Prev {:?}", prev);
//         for j in 0..n {
//             // 1<->0
//             // let robot_0 = grid[m-i-1][j]^1 as usize;
//             let robot_0 = grid[i][j]^1 as usize;
//             curr[j] = if j+1 == n {
//                 prev[j]
//             } else {
//                 curr[j+1] + prev[j]
//             };
//             curr[j] = curr[j] * robot_0;
//             println!("Curr {:?}", curr);
//         }
//         println!("Curr {:?}", curr);
//         println!("-------");
//         mem::swap(&mut prev, &mut curr);
//     }
//     prev[0]
// }
