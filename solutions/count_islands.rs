use std::collections::HashMap;
use std::collections::HashSet;

fn count_islands(grid:&[Vec<usize>]) -> usize {
    if grid.is_empty() || grid[0].len() == 0 { return 0 };
    let m = grid.len();
    let n = grid[0].len();
    let mut cnt = 0;

    let mut seen:HashSet<(usize,usize)> = HashSet::new();
    for i in 0..m {
        for j in 0..n {
            if !seen.contains(&(i,j)) && grid[i][j] == 1 {
                // don't need to pass any values if check grid and seen
                cnt +=1;
                dfs(i, j, grid, &mut seen);
            }
        }
    }
    cnt
}


fn dfs(x:usize,y:usize, grid:&[Vec<usize>], seen:&mut HashSet<(usize, usize)>) {
    // simulated twos compiment for !0
    seen.insert((x,y));
    if grid[x][y] == 0 { return};
    let m = grid.len();
    let n = grid[0].len();
    let dirs = [(1, 0), (0,1), (!0, 0), (0, !0)];

    for (dx,dy) in dirs {
        let nx = x.wrapping_add(dx);
        let ny = y.wrapping_add(dy);
        if nx < m && ny < n && !seen.contains(&(nx,ny)) {
            dfs(nx, ny, grid, seen);
            // no rollback wanted i think?
        }
    }
}


fn main() {
    let g = &[
      vec![1,1,1,1,0],
      vec![1,1,0,1,0],
      vec![1,1,0,0,0],
      vec![0,0,0,0,0]
    ];
    assert_eq!(1, count_islands(g));

  let g = &[
      vec![1,1,0,0,0],
      vec![1,1,0,0,0],
      vec![0,0,1,0,0],
      vec![0,0,0,1,1]
    ];
    assert_eq!(3, count_islands(g));
}
