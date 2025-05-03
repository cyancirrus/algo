use std::cmp::Ordering;
// fn matrix_search(target:i32, m:&[Vec<i32>]) -> bool {
//     let rows = m.len();
//     if rows == 0 { return false };
//     let cols = m[0].len();
//     if cols == 0 { return false };
//     // todo make the row search binary
//     for i in 0..rows {
//         if i == rows - 1 || target < m[i+1][0] {
//             let (mut l, mut c, mut u) = (0, cols / 2, cols);
//             while l != u {
//                 if m[i][c] == target {
//                     return true
//                 } else if m[i][c] < target {
//                     l = c + 1;
//                 } else {
//                     u = c;
//                 }
//                 c = ( l + u )/2
//             }
//         }
//     }
//     false
// }

// fn matrix_search(target:i32, m:&[Vec<i32>]) -> bool {
//     let rows = m.len();
//     if rows == 0 { return false };
//     let cols = m[0].len();
//     if cols == 0 { return false };
//     let (mut l, mut u) = (0, rows - 1);
//     let s = loop {
//         let c = ( l + u )/2;
//         if m[c][0] <= target && target <= m[c][cols - 1] {
//             break c;
//         } else if l >= u {
//             return false
//         } else if m[c][0] < target {
//             l = c + 1;
//         } else {
//             u = c - 1;
//         }
//     };
//     (l, u) = (0, cols - 1);
//     while l <= u {
//         let c = ( l + u )/2;
//         if m[s][c] == target {
//             return true
//         } else if m[s][c] < target {
//             l = c + 1;
//         } else {
//             u = c - 1;
//         }
//     }
//     false
// }

fn matrix_search(target:i32, m:&[Vec<i32>]) -> bool {
    if m.is_empty() || m[0].is_empty() {
        return false
    };
    let cols = m[0].len();
    let row = match m.binary_search_by(|row| {
        if target < row[0] {
            Ordering::Greater
        } else if target > row[cols - 1] {
            Ordering::Less
        } else {
            Ordering::Equal
        }
    })
    {
        Ok(idx) => idx,
        Err(_) => return false,
    };

    m[row].binary_search(&target).is_ok()
}


fn main() {

    let matrix = &vec![vec![1,3,5,7],vec![10,11,16,20],vec![23,30,34,60]];
    let target = 11;

    println!("Result {}", matrix_search(target, matrix));
}
