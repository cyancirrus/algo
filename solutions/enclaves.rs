use std::collections::HashSet;

fn enclaves(grid: &mut [Vec<bool>]) -> usize {
    if grid.is_empty() || grid[0].len() == 0 { return 0 };
    let m = grid.len();
    let n = grid[0].len();
    let mut enclave = 0;
    let mut connected:HashSet<(usize, usize)> = HashSet::new();
    for i in 0..m {
        dfs(i, 0, grid, &mut connected);
        dfs(i, n-1, grid, &mut connected);

    }
    for j in 0..n {
        dfs(0, j, grid, &mut connected);
        dfs(m-1, j, grid, &mut connected);
    }
    for i in 1..m-1 {
        for j in 1..n-1 {
            if grid[i][j] & !connected.contains(&(i,j)) {
                enclave += 1
            }
        }
    }
    enclave
}

fn dfs(x:usize, y:usize, grid:&mut [Vec<bool>], seen:&mut HashSet<(usize,usize)>) {
    // edges can be initialized to be 0 need to check
    if seen.contains(&(x,y)) || !grid[x][y] { return };
    seen.insert((x,y));
    let dirs = [(1, 0), (0,1), (!0, 0), (0, !0)];
    let m = grid.len();
    let n = grid[0].len();

    for (dx, dy) in dirs {
        let nx = x.wrapping_add(dx);
        let ny = y.wrapping_add(dy);
        
        if nx < m && ny < n && grid[nx][ny] {
            dfs(nx, ny, grid, seen);
        }
    }
}




fn main() {
    let mut g = vec![
      vec![true,true,true,true,false],
      vec![true,true,false,true,false],
      vec![true,true,false,false,false],
      vec![false,false,false,false,false]
    ];
    assert_eq!(0, enclaves(&mut g));

  let mut g = vec![
      vec![true,true,false,false,false],
      vec![true,true,false,false,false],
      vec![false,false,true,false,false],
      vec![false,false,false,true,true]
    ];
    assert_eq!(1, enclaves(&mut g));
}
