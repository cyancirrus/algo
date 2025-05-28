use std::mem;
use std::collections::HashMap;



fn rotate_90_one_pass(mat: &mut [Vec<i32>]) {
    let n = mat.len();
    if n == 0 { return; }

    for i in 0..n / 2 {
        for j in i..n - i - 1 {
            // Indices of the 4 cells in the rotation cycle
            let mut temp = mat[i][j];
            mat[i][j] = mat[n - 1 - j][i];
            mat[n - 1 - j][i] = mat[n - 1 - i][n - 1 - j];
            mat[n - 1 - i][n - 1 - j] = mat[j][n - 1 - i];
            mat[j][n - 1 - i] = temp;
        }
    }
}




fn rotate(mat:&mut [Vec<i32>])  {
    // square matrix
    if mat.is_empty()  { return };
    let len = mat.len();
    // transpose
    for i in 0..len/2 {
        for j in i+1..len {
            // split to avoid aliasing
            let (top, bot) = mat.split_at_mut(j);
            let row_i = &mut top[i];
            let row_j = &mut bot[0];
            mem::swap(&mut row_i[j], &mut row_j[i]);
        }
    }
    // reflect column index
    for row in mat.iter_mut() {
        row.reverse();
    }
}

fn main() {
    let test = &mut [vec![1,2],vec![3,4]];
    rotate(test);
    println!("Rotate {:?}", test);
}



// fn can_i_win_naive(max_choose:usize, total:usize) -> bool {
//     // for reuse
//     total / max_choose % 2 == 1
// }

// fn can_i_win(m_choose:u8, total:u8) -> bool {
//     if m_choose * (m_choose + 1) / 2 < total {
//         // nobody can win
//         return false;
//     }
//     // have to enumerate states not a straightforward analytic solution(?)
//     let mut memo: HashMap<u32, bool> = HashMap::new();
//     calc_win(0, 0, total, &mut memo)
// }

// fn calc_win(bmask:u32, curr:u8, total:u8, memo:&mut HashMap<u32, bool>) -> bool {
//     if let Some(res) = memo.get(&bmask) {
//         return *res
//     };

//     for i in 1..total {
//         let mask = 1 << i;
//         // haven't been calculated
//         if bmask & mask == 0 {
//             // if i can't curr-i can't win then win
//             if i >= total || !calc_win(bmask | mask,  curr-i, total, memo) {
//                 memo.insert(bmask, true);
//                 return true
//             }
//         }
//     }
//     memo.insert(bmask, false);
//     false
// }

// what happens if i choose the compliment? ie pivot around the midpoint

// 1 - 10,
// 1 -> 10
// 2 - > 9
// ...
// 11 + 11 + ... = 55

// lower + upper = a constant which can be kept but what happens if it's in the midpoint
// 15
// => 4 -> _ -> 15

// perhaps there are edge cases i do not understand
// - what numbers were currently used
// - 




// fn main() {
//     assert_eq!(1, can_i_win(1, 3));
// }

