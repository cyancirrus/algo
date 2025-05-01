// fn zero_rows_and_columns(m:&mut [Vec<i32>]) -> &[Vec<i32>] {
//     // Space with O(n * m)
//     let rows = m.len();
//     let cols:usize;
//     if rows == 0 {
//         return m
//     } else {
//         cols = m[0].len();
//         if cols == 0 {
//             return m;
//         }
//     }
//     let mut changes= vec![0;rows * cols];
//     for i in 0..rows {
//         for j in 0..cols {
//             if m[i][j] == 0 && changes[i*cols + j] == 0 {
//                 for _i in 0..rows {
//                     if m[_i][j] != 0 {
//                         m[_i][j] = 0;
//                         changes[_i * cols + j] = 1;
//                     }
//                 }
//                 for _j in 0..cols {
//                     if m[i][_j] != 0 {
//                         m[i][_j] = 0;
//                         changes[i * cols + _j] = 1;
//                     }
//                 }
//             }
//         }
//     }
//     m
// }

// fn zero_rows_and_columns(m:&mut [Vec<i32>]) -> &[Vec<i32>] {
//     // O(1) space but doesn't work for i32::MIN
//     let rows = m.len();
//     if rows == 0 { return m };
//     let cols = m[0].len();
//     if cols == 0 { return m };

//     for i in 0..rows {
//         for j in 0..cols {
//             if m[i][j] == 0 {
//                 for _i in 0..rows {
//                     if m[_i][j] != 0 {
//                         m[_i][j] = i32::MIN;
//                     }
//                 }
//                 for _j in 0..cols {
//                     if m[i][_j] != 0 {
//                         m[i][_j] = i32::MIN;
//                     }
//                 }
//             }
//         }
//     }
//     for i in 0..rows {
//         for j in 0..cols {
//             if m[i][j] == i32::MIN {
//                 m[i][j] = 0;
//             }
//         }
//     }
//     m
// }

fn zero_rows_and_columns(m:&mut [Vec<i32>]) -> &[Vec<i32>] {
    // O(1) space use the first row/column as storage
    // Compute the changes first and then change
    let rows = m.len();
    if rows == 0 { return m };
    let cols = m[0].len();
    if cols == 0 { return m };
    let mut zero_first_row = false;
    let mut zero_first_col = false;

    for i in 0..rows {
        if m[i][0] == 0 {
            zero_first_col = true;
        }
    }
    for j in 0..cols {
        if m[0][j] == 0 {
            zero_first_row = true;
        }
    }
    for i in 1..rows {
        for j in 1..cols {
            if m[i][j] == 0 {
                m[i][0] = 0;
                m[0][j] = 0;
            }
        }
    }
    for i in 0..rows {
        for j in 0..cols {
            if m[0][j] == 0 || m[i][0] == 0  {
                m[i][j] = 0;
            }
        }
    }
    if zero_first_row {
        for j in 0..cols {
            m[0][j] = 0;
        }
    }
    if zero_first_col {
        for i in 0..rows {
            m[i][0] = 0;
        }
    }
    m
}

fn main () {
    let mut m = vec![vec![1,1,1],vec![1,0,1],vec![1,0,1]];
    // let test = vec![vec![1,0,1],vec![0,0,0],vec![1,0,1]];

    let result = zero_rows_and_columns(&mut m);
    println!("First iteration {:?}", result);
}
