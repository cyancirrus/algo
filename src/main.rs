fn game_of_life(grid:&mut Vec<Vec<u8>>) -> &Vec<Vec<u8>> {
    if grid.is_empty() || grid[0].is_empty() { return grid; }
    let m = grid.len();
    let n = grid[0].len();
    for x in 0..m {
        let mut neighbors = 0;
        // initialize add in the (0,0) for consistency
        for (dx, dy) in [ (!0,0), (1,0), (0,0)] {
            let nx = x.wrapping_add(dx);
            let ny = 0usize.wrapping_add(dy);
            if nx < m && ny < n {
                neighbors += if grid[nx][ny] & 1 == 1 { 1 } else { 0 };
            }
        }
        for y in 0..n {
            // remove the neighbors specific to the previous
            for (dx, dy) in [(1, !1), (!0, !1), (0, !1), (0, 0)] {
                let nx = x.wrapping_add(dx);
                let ny = y.wrapping_add(dy);
                if nx < m && ny < n {
                    neighbors -= if grid[nx][ny] & 1 == 1 { 1 } else { 0 };
                }
            }
            // add the neighbors from the previous
            for (dx, dy) in [ (0,!0), (!0,1), (0,1), (1,1), ] {
                let nx = x.wrapping_add(dx);
                let ny = y.wrapping_add(dy);
                if nx < m && ny < n {
                    neighbors += if grid[nx][ny] & 1 == 1 { 1 } else { 0 };
                }
            }
            println!("neighbors {neighbors:?}");
            // bit number 2 is next
            grid[x][y] += match neighbors {
                2 => grid[x][y] << 1,
                3 => 0b10 , 
                _ => 0b00,
            };
        }
    }
    // update the result
    for x in 0..m {
        for y in 0..n { grid[x][y] >>= 1; }
    }
    grid
}


// fn game_of_life(grid:&mut Vec<Vec<u8>>) -> &Vec<Vec<u8>> {
//     if grid.is_empty() || grid[0].is_empty() { return grid; }
//     let m = grid.len();
//     let n = grid[0].len();
//     for x in 0..m {
//         for y in 0..n {
//             let mut neighbors:u8 = 0;
//             for (dx, dy) in [
//                 (1,0),  (0,1), (1,1), (!0,1),
//                 (1,!0), (!0,0), (0,!0), (!0,!0), 
//             ] {
//                 let nx = x.wrapping_add(dx);
//                 let ny = y.wrapping_add(dy);
//                 if nx < m && ny < n {
//                     neighbors += if grid[nx][ny] & 1 == 1 { 1 } else { 0 };
//                 }
//             }
//             grid[x][y] += match neighbors {
//                 2 => grid[x][y] << 1,
//                 3 => 0b10 , 
//                 _ => 0b00,
//             };
//         }
//     }
//     for x in 0..m {
//         for y in 0..n { grid[x][y] >>= 1; }
//     }
//     grid
// }

fn main() {
    println!("game of like {:?}",  game_of_life(&mut vec![vec![0,1,0],vec![0,0,1],vec![1,1,1],vec![0,0,0]]));
    // println!("game of like {:?}",  game_of_life(&mut vec![vec![0,0,1],vec![1,1,1],vec![0,0,0]]));
    // println!("game of like {:?}",  game_of_life(&mut vec![vec![1,1,1],vec![0,0,0]]));
    // println!("game of like {:?}",  game_of_life(&mut vec![vec![0,1], vec![1,1]]));
    // println!("game of like {:?}",  game_of_life(&mut vec![vec![1,1], vec![1,0]]));
    // println!("testing {:?}", !1 + 2);
}
