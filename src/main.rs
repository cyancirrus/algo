use std::mem;

fn climb_stairs(n:usize) -> usize {
    let (mut low, mut high) = (0, 1);
    for _ in 1..=n {
        let val = low + high;
        mem::swap(&mut low,&mut high);
        high = val;
    }
    high
}

// fn climb_stairs(n:usize) -> usize {
//     let mut dp = vec![0;n+1];
//     dp[0]=1;
//     dp[1]=1;
//     for i in 2..=n {
//         dp[i] = dp[i-1] + dp[i-2]
//     }
//     dp[n]
// }


// i could cache this and this would be top down
// f(n) = f(n-1) + f(n-2);
// this is similar to the coin change problem
// fn climb_stairs(n:usize) -> usize {
//     // simple version lets build up to dp
//     if n==1 {
//         return 1;
//     } else if n == 2 {
//         return 2;
//     }
//     climb_stairs(n-1) + climb_stairs(n-2)
// }

fn main() {
    assert_eq!(1, climb_stairs(0)); // [1]
    assert_eq!(1, climb_stairs(1)); // [1]
    assert_eq!(2, climb_stairs(2)); // [1+1], [2]
    assert_eq!(3, climb_stairs(3)); // [1+1+1], [1+2], [2+1]
    assert_eq!(5, climb_stairs(4)); // 5 ways
}

