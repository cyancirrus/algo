fn surrounded_regions(grid:&mut Vec<Vec<char>>) -> &mut Vec<Vec<char>> {
    if grid.is_empty() || grid[0].is_empty() { return grid; }
    let m = grid.len();
    let n = grid[0].len();

    fn dfs(
        x:usize, y:usize,
        m:usize, n:usize,
        grid:&mut Vec<Vec<char>>,
    ) {
        if x >= m || y >= n || grid[x][y] != 'O' { return; }
        grid[x][y] = 'S';
        for (dx, dy) in [(1, 0), (0, 1), (!0, 0), (0, !0)] {
            let nx = x.wrapping_add(dx);
            let ny = y.wrapping_add(dy);
            dfs(nx, ny, m, n, grid);
        }
    }
    
    for i in 0..m {
        dfs(i, 0, m, n, grid);
        dfs(i, n-1, m, n, grid);
    }
    for j in 0..n {
        dfs(0, j, m, n, grid);
        dfs(m-1, j, m, n, grid);
    }
    for i in 0..m {
        for j in 0..n {
            grid[i][j] = match grid[i][j] {
                'S' => 'O',
                'X' => 'X',
                c => c,
            }
        }
    }
    grid
}



// fn surrounded_regions(grid:&mut Vec<Vec<char>>) -> &mut Vec<Vec<char>> {
//     if grid.is_empty() || grid[0].is_empty() { return grid; }
//     let m = grid.len();
//     let n = grid[0].len();
//     let mut explored = vec![vec![false;n];m];

//     for i in 1..m {
//         for j in 1..n {
//             if grid[i][j] == 'O' && !explored[i][j] {
//                 let mut group = vec![];
//                 let touches_boarder = dfs( i,j, m,n, grid, &mut explored, &mut group);
//                 if !touches_boarder {
//                     for (x,y) in group {
//                         grid[x][y] = 'X';
//                     }
//                 }

//             }
//         }
//     }
//     grid

// }

// fn dfs(
//     x:usize, y:usize,
//     m:usize, n:usize,
//     grid:&mut Vec<Vec<char>>,
//     explored:&mut Vec<Vec<bool>>,
//     group:&mut Vec<(usize, usize)>
// ) -> bool {
//     group.push((x, y));
//     explored[x][y] = true;
//     let mut touches_boarder = x == 0 || x == m - 1 || y == 0 || y == n - 1 ;
//     for (dx, dy) in [(1, 0), (0, 1), (!0, 0), (0, !0)] {
//         let nx = x.wrapping_add(dx);
//         let ny = y.wrapping_add(dy);
//         if nx < m && ny < n && !explored[nx][ny] && grid[nx][ny] == 'O' {
//             if dfs(nx, ny, m, n, grid, explored, group) {
//                 touches_boarder = true;
//             }
//         }
//     }
//     touches_boarder
// }


fn main() {
    // println!("test {:?}" , wordbreak("leetcode", vec!["leet","code"]));
    // println!("test {:?}" , wordbreak_iter("leetcode", vec!["leet","code"]));
    // permutation_sequence(4, 9);
}
