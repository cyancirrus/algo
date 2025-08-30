use std::collections::HashMap;

fn binary_search_upper(nums:&[u32], target:u32) -> usize {
    let mut l = 0;
    let mut r = nums.len();
    while l < r  {
        let m = (l + r) / 2;
        if target > nums[m] { l = m + 1; }
        else { r = m; }
    }
    l
}
// note how m <= target
fn binary_search_lower(nums:&[u32], target:u32) -> usize {
    let mut l = 0;
    let mut r = nums.len() - 1;
    while l < r {
        let m = (l + r + 1) / 2;
        if target < nums[m] { r = m - 1; }
        else { l = m; }
    }
    l
}

fn search_matrix(matrix:&[Vec<u32>], target:u32) -> (isize, isize ) {
    if matrix.is_empty() || matrix[0].is_empty() { return (-1, -1); }
    let (m, n) = (matrix.len(), matrix[0].len());
    if target < matrix[0][0] || target > matrix[m-1][n-1] { return (-1, -1); }
    
    let (mut l, mut r ) = (0, m-1);
    while l < r {
        let c = (l + r + 1) / 2;
        if target < matrix[c][0] { r = c - 1; }
        else { l = c };
    }
    let row = l;
    (l, r) = (0, n - 1);
    let mut column = 0;
    while l <= r {
        let c = (l + r) / 2;
        if target < matrix[row][c] { r = c - 1; }
        else if matrix[row][c] < target { l = c + 1; }
        else { column = c; break; }
    }
    if matrix[row][column] == target {
        (row as isize, column as isize)
    } else {
        (-1, -1)
    }
}

fn main() {
    // println!(
    //     "result {:?}",
    //     binary_search_lower(&[0,5,15], 2)
    // );
    // let arr = [1, 3, 3, 5, 7, 9];
    // for t in 0..=10 {
    //     println!("target {t}: lower index = {}", binary_search_upper(&arr, t));
    // }
    // for t in 0..=10 {
    //     println!("target {t}: lower index = {}", binary_search_lower(&arr, t));
    // }

    // println!(
    //     "result {:?}",
    //     search_matrix(&[vec![0, 2, 4],vec![5, 12, 14], vec![15,30, 60]], 7)
    // );
    println!(
        "result {:?}",
        search_matrix(&[vec![0, 2, 4],vec![5, 12, 14], vec![15, 30, 60]], 6)
    );
    // println!(
    //     "result {:?}",
    //     search_matrix(&[vec![0], vec![5], vec![15]], 2)
    // );

}
