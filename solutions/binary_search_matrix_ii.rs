use std::collections::HashMap;

fn binary_search_lower_row(
    matrix:&[Vec<u32>],
    row:usize,
    bound:usize,
    target:u32
) -> usize {
    let (mut l, mut r) = (0, bound);
    while l < r {
        let m = (l + r + 1) / 2;
        if target < matrix[row][m] { r = m - 1; }
        else { l = m; }
    }
    return l
}

fn binary_search_upper_col(
    matrix:&[Vec<u32>],
    rows:usize,
    col:usize,
    bound:usize,
    target:u32
) -> usize {
    let (mut d, mut u) = (bound, rows);
    while d < u {
        let m = (d + u) /2;
        if matrix[m][col] < target { d = m + 1; }
        else { u = m; }
    }
    return d
}

fn search_matrix_ii(matrix:&[Vec<u32>], target:u32) -> bool {
    if matrix.is_empty() || matrix[0].is_empty() { return false; }
    let (m, n) = (matrix.len(), matrix[0].len());
    let (mut row, mut col) = (0, n-1);
    while row < m {
        col = binary_search_lower_row(matrix, row, col, target);
         if matrix[row][col] == target { return true; }
        row = binary_search_upper_col(matrix, m, col, row, target);
        if row == m { return false; }
        if matrix[row][col] == target { return true; }
    }
    false
}

fn search_matrix_linear_ii(matrix:&[Vec<u32>], target:u32) -> bool {
    if matrix.is_empty() || matrix[0].is_empty() { return false; }
    let (m, n) = (matrix.len(), matrix[0].len());
    let (mut r, mut c) = (0, n-1);

    while r < m {
        if matrix[r][c] == target { return true; }
        else if matrix[r][c] < target { r += 1; }
        else { 
            if c == 0 {return false; }
            c-=1;
        }
    }
    false
}

fn main() {
    // println!(
    //     "result {:?}",
    //     search_matrix_ii(
    //         &[vec![1,4,7,11,15],vec![2,5,8,12,19],vec![3,6,9,16,22],vec![10,13,14,17,24],vec![18,21,23,26,30]],
    //         11
    //     )
    // );
    println!(
        "result {:?}",
        search_matrix_ii(
            &[vec![1,4,7,11,15],vec![2,5,8,12,19],vec![3,6,9,16,22],vec![10,13,14,17,24],vec![18,21,23,26,30]],
            20
        )
    );


}
