use std::mem;

fn perfect_squares(n:usize) -> usize {
    let mut comb= vec![n;n+1];
    comb[0]=0;
    for b in 1.. {
        let square = b*b;
        if square > n { break };
        for amt in square..=n {
            comb[amt] = comb[amt].min(comb[amt - square] + 1);
        }
    }
    comb[n]
}

// -- the problem isn't gready ie
// 12 - 9 = 3, doesn't work
// -- change problem worked with an amount, where each amount was tracked with it
// we could imagine something similar like
// 10 = 1 + 1 + ... 1
// 10 = 4 + 4 + 2
// 2 :: 2
// 123456789
// 12....345
// so it's going to be like a min that's running
// so lets take (n+1)/2 up to the thing and then do
// dp[amt] = dp[amt - b] + j
// -- doesn't need to be too complex lets just use the change problem pattern


fn main() {
    assert_eq!(1, perfect_squares(4));
    assert_eq!(3, perfect_squares(3));
    assert_eq!(3, perfect_squares(12));
    assert_eq!(2, perfect_squares(13));
}
