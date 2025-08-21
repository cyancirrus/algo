// b c
// c b
// a b c -> bc, ac, ab
//  (r[0]) (l[1] * r[1]) * (l[2])


fn cons_product_matrix(grid:&[Vec<u32>]) -> Vec<Vec<u32>> {
    if grid.is_empty() || grid[0].is_empty() { return vec![] };
    let ( m, n ) = ( grid.len(), grid[0].len() );
    let mut curs = vec![vec![1;n];m];
    let mut accum = 1;
    for i in 0..m {
        for j in 0..n {
            curs[i][j] = accum;
            accum *= grid[i][j];
        }
    }
    accum = 1;
    for i in (0..m).rev() {
        for j in (0..n).rev() {
            curs[i][j] *= accum;
            accum *= grid[i][j];
        }
    }
    curs
}









// 1, 2, 3, 4
fn product_except_self(nums:&[i32]) -> Vec<i32> {
    let n = nums.len();
    let mut curs = vec![1;n];
    let mut accum = 1;
    for i in 1..n {
        accum *= nums[i-1];
        curs[i] *= accum;
    }
    let mut accum = 1;
    for i in 1..n {
        accum *= nums[n-i];
        curs[n-i-1] *= accum;
    }
    curs
}
// fn product_except_self(nums:&[i32]) -> Vec<i32> {
//     let n = nums.len();
//     let mut left = vec![1;n];
//     let mut right = vec![1;n];
//     for i in 1..n {
//         left[i] = left[i-1] * nums[i-1];
//         right[n-1-i] = right[n-i] * nums[n-i];
//     }
//     for i in 0..n {
//         left[i] *= right[i];
//     }
//     left
// }

fn main() {
    // println!("product except self {:?}", product_except_self(&[1,2,3,4]));
    println!("for matrices can just do it {:?}", cons_product_matrix(&[vec![1,2],vec![3,4]]));
}
