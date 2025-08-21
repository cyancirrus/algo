// b c
// c b
// a b c -> bc, ac, ab
//  (r[0]) (l[1] * r[1]) * (l[2])


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
    println!("product except self {:?}", product_except_self(&[1,2,3,4]));
}
