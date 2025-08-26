// fn game_of_life_bitmap(grid:&mut [u64]) {
//     let m = grid.len();
//     let mut next = vec![0u64;m];

//     for x in 0..m {
//         let row = grid[x];
//         let above = if x > 0 { grid[x-1] } else { 0 };
//         let below = if x + 1 < m { grid[x+1] } else { 0 };
        
//         let left_above = (above << 1 ) & !1;
//         let right_above = (above >> 1 ) & !(1 << 63);
//         let horiz_above = left_above.wrapping_add(above).wrapping_add(right_above);

//         let left_row = (row << 1) & !1;
//         let right_row = (row >> 1) & !(1 << 63);
//         let horiz_row = left_row.wrapping_add(row).wrapping_add(right_row);

//         let left_below = (below << 1) & !1;
//         let right_below = (below >> 1) & !(1 << 63);
//         let horiz_below = left_below.wrapping_add(below).wrapping_add(right_below);

//         let total = horiz_above.wrapping_add(horiz_row).wrapping_add(horiz_below);

//         // let lo = a ^ b ^ c;
//         // let hi= (a & b) | (a & c) | (b & c);
//         // count should be like 2 * high + low, well bounded by below it works for anything > 3
//         // eq2 = hi+ hi+ lo; = 
//         // eq2 = hi & !lo;
//         // eq3 = hi & lo;
//         // next = eq3 | (eq2 & alive);

// // (lo1, hi1) = add3(a0, b0, c0)     // add lows
// // (lo2, hi2a)= add3(a1, b1, c1)     // add highs
// // (lo , c0 ) = add3(lo1, lo2, 0)    // combine lows
// // (hi , hi2 )= add3(hi1, hi2a, c0)  // combine highs + carry



//     }
//     *grid = next
// }



fn game_of_life_precomp(grid:&mut Vec<Vec<bool>>) -> &Vec<Vec<bool>> {
    if grid.is_empty() || grid[0].is_empty() { return grid; }
    let m = grid.len();
    let n = grid[0].len();
    let mut horiz = vec![vec![0u8;n];m];
    for x in 0..m {
        let mut hsum = if grid[x][0] { 1 } else { 0 };
        for y in 0..n {
            if y+1 < n && grid[x][y+1]{ hsum += 1; }
            if y > 0 && grid[x][y-1] { hsum -= 1; }
            horiz[x][y] = hsum;
        }
    }
    let mut verticals = vec![0u8;m];
    for y in 0..n {
        let mut vsum = horiz[0][y];
        for x in 0..m {
            if x+1 < m { vsum += horiz[x+1][y]; }
            if x > 0 { vsum -= horiz[x-1][y]; }
            verticals[x] = vsum;
        }
        for x in 0..m {
            let neighbors = verticals[x] - grid[x][y] as u8;
            grid[x][y] = (neighbors == 3) || (neighbors == 2 && grid[x][y]);
        }
    }
    grid
}

fn game_of_life(grid:&mut Vec<Vec<u8>>) -> &Vec<Vec<u8>> {
    if grid.is_empty() || grid[0].is_empty() { return grid; }
    let m = grid.len();
    let n = grid[0].len();
    for x in 0..m {
        for y in 0..n {
            let mut neighbors:u8 = 0;
            for (dx, dy) in [
                (1,0),  (0,1), (1,1), (!0,1),
                (1,!0), (!0,0), (0,!0), (!0,!0), 
            ] {
                let nx = x.wrapping_add(dx);
                let ny = y.wrapping_add(dy);
                if nx < m && ny < n {
                    neighbors += if grid[nx][ny] & 1 == 1 { 1 } else { 0 };
                }
            }
            grid[x][y] += match neighbors {
                2 => grid[x][y] << 1,
                3 => 0b10 , 
                _ => 0b00,
            };
        }
    }
    for x in 0..m {
        for y in 0..n { grid[x][y] >>= 1; }
    }
    grid
}

// fn game_of_life_lol(grid:&mut Vec<Vec<u8>>) -> &Vec<Vec<u8>> {
//     // thought there might be less flops if we reused neighbor
//     if grid.is_empty() || grid[0].is_empty() { return grid; }
//     let m = grid.len();
//     let n = grid[0].len();
//     for x in 0..m {
//         let mut neighbors = 0;
//         // initialize add in the (0,0) for consistency
//         for (dx, dy) in [ (!0,0), (1,0), (0,0)] {
//             let nx = x.wrapping_add(dx);
//             let ny = 0usize.wrapping_add(dy);
//             if nx < m && ny < n {
//                 neighbors += if grid[nx][ny] & 1 == 1 { 1 } else { 0 };
//             }
//         }
//         for y in 0..n {
//             // remove the neighbors specific to the previous
//             for (dx, dy) in [(1, !1), (!0, !1), (0, !1), (0, 0)] {
//                 let nx = x.wrapping_add(dx);
//                 let ny = y.wrapping_add(dy);
//                 if nx < m && ny < n {
//                     neighbors -= if grid[nx][ny] & 1 == 1 { 1 } else { 0 };
//                 }
//             }
//             // add the neighbors from the previous
//             for (dx, dy) in [ (0,!0), (!0,1), (0,1), (1,1), ] {
//                 let nx = x.wrapping_add(dx);
//                 let ny = y.wrapping_add(dy);
//                 if nx < m && ny < n {
//                     neighbors += if grid[nx][ny] & 1 == 1 { 1 } else { 0 };
//                 }
//             }
//             println!("neighbors {neighbors:?}");
//             // bit number 2 is next
//             grid[x][y] += match neighbors {
//                 2 => grid[x][y] << 1,
//                 3 => 0b10 , 
//                 _ => 0b00,
//             };
//         }
//     }
//     // update the result
//     for x in 0..m {
//         for y in 0..n { grid[x][y] >>= 1; }
//     }
//     grid
// }

fn main() {
    println!("game of like {:?}",  game_of_life(&mut vec![vec![0,1,0],vec![0,0,1],vec![1,1,1],vec![0,0,0]]));
    // println!("game of like {:?}",  game_of_life_precomp(&mut vec![vec![0,1,0],vec![0,0,1],vec![1,1,1],vec![0,0,0]]));
    // println!("game of like {:?}",  game_of_life(&mut vec![vec![0,0,1],vec![1,1,1],vec![0,0,0]]));
    // println!("game of like {:?}",  game_of_life_precomp(&mut vec![vec![1,1,1],vec![0,0,0]]));
    // println!("game of like {:?}",  game_of_life(&mut vec![vec![1,1,1],vec![0,0,0]]));
    // println!("game of like {:?}",  game_of_life(&mut vec![vec![0,1], vec![1,1]]));
    // println!("game of like {:?}",  game_of_life(&mut vec![vec![1,1], vec![1,0]]));
    // println!("testing {:?}", !1 + 2);
}
