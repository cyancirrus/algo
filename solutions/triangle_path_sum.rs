// min path sum
fn triangle(tri:&[Vec<i8>]) -> i8 {
    if tri.is_empty() {
        return 0
    } else if tri.len() == 1 {
        return tri[0][0]
    }
    let n = tri.len();
    let mut dp = vec![0;n+1];
    for layer in (0..n).rev() {
        for idx in 0..=layer   {
            dp[idx] = tri[layer][idx] + dp[idx].min(dp[idx+1]);
        }
    }
    dp[0]
}

fn main() {
    assert_eq!(-10, triangle(&[vec![-10]]));
    assert_eq!(11, triangle(&[vec![2],vec![3,4],vec![6,5,7],vec![4,1,8,3]]));
}
