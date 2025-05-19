fn coin_change(amount:usize, coins:&mut [usize]) -> usize {
    let mut dp = vec![0;amount+1];
    dp[0]=1;
    for &d in coins.iter() {
        for amt in d..=amount {
            dp[amt] += dp[amt - d];
        }
    }
    dp[amount]
}

// final solution personal solution
fn coin_change_homebrew(amount:usize, coins:&mut [usize]) -> usize {
    // requires a sort
    coins.sort();
    let mut dp = vec![0;amount + 1];
    for &d in coins.iter().rev() {
        let mut amt = 0;
        loop {
            if amt + d > amount {
                break;
            }
            dp[amt+d] += 1.max(dp[amt]);
            amt += d;
        }
    }
    dp[amount]
}
// fn coin_change(amount:usize, coins:&[usize]) -> i32 {
//     // problem constraint will be dense when there is a 1 coind
//     // just want to get cache up before worrying about hashmap borrow rules
//     let mut cache = vec![0;(amount + 1) as usize];
//     let result = _coin_change_(amount, &coins, &mut cache);
//     result
// }

// fn coin_change(amount:usize, coins:&mut [usize]) -> usize {
//     // sloppy loops but works
//     coins.sort();
//     let mut dp = vec![0;(amount + 1) as usize];
//     dp[0] = 0;
//     for &d in coins.iter().rev() {
//         let mut amt = d;
//         loop {
//             println!("amt Amount ({}, {})", amt, amount);
//             if amt > amount {
//                 break;
//             } else if amt <= amount {
//                 let mut inc = 1;
//                 if amt > d {
//                     inc = inc.max(dp[amt-d] );
//                 }
//                 dp[amt] += inc;
//             }
//             amt += d;
//         }
//         println!("What does the dp look like {:?}", dp);
//     }
//     println!("What does the dp look like {:?}", dp);
//     dp[amount]
// }
// fn coin_change(amount:usize, coins:&mut [usize]) -> i32 {
//     coins.sort();
//     let mut dp = vec![0;(amount + 1) as usize];
//     for &d in coins.iter() {
//         let mut amt = d;
//         loop {
//             println!("amt Amount ({}, {})", amt, amount);
//             if amt > amount {
//                 break;
//             } else if amt < amount {
//                 dp[amt] += 1;
//             } else if amt == amount {
//                 println!("hello world");
//                 dp[amount] += dp[amount - d];
//             }
//             amt += d;
//         }
//         println!("What does the dp look like {:?}", dp);
//     }
//     dp[amount]
// }
// fn _coin_change_(amount:usize, coins:&[usize], d:usize, cache:&mut [Vec<i32>]) -> i32 {
//     // okay this sucks lets get a cache for it and then see if we can't linearize it
//     let mut cnt = 0;
//     if cache[d][ amount as usize ] != -1 {
//         return cache[d][ amount as usize ] 
//     }
//     for i in d..coins.len() {
//         let d = coins[i];
//         if amount > d {
//             cnt += _coin_change_(amount - d, coins, i, cache);
//         } else if amount == d {
//             cnt += 1;
//         }
//     }
//     cache[d][amount as usize] = cnt;
//     cnt
// }


fn main() {
    // assert_eq!(1, coin_change(1, &mut [1]));
    // assert_eq!(0, coin_change(1, &mut [10]));
    // assert_eq!(4, coin_change(10, &mut [1, 5, 10]));
    assert_eq!(4, coin_change(5, &mut [1, 2, 5]));
    // assert_eq!(4, coin_change(10, &mut [10, 5, 1]));
    // assert_eq!(3, coin_change(10, &mut [5, 1]));
    // assert_eq!(3, coin_change(10, &mut [1, 5]));
}

// fn coin_change(amount:i32, coins:&[i32]) -> i32 {
//     // problem constraint will be dense when there is a 1 coind
//     // just want to get cache up before worrying about hashmap borrow rules
//     // let mut cache = vec![-1;(amount + 1) as usize];
//     let mut cache = vec![vec![-1; (amount + 1) as usize];coins.len()];
//     let result = _coin_change_(amount, coins,0, &mut cache);
//     println!("Cache {:?}", cache);
//     result
// }

// fn _coin_change_(amount:i32, coins:&[i32], d:usize, cache:&mut [Vec<i32>]) -> i32 {
//     // okay this sucks lets get a cache for it and then see if we can't linearize it
//     let mut cnt = 0;

//     if cache[d][ amount as usize ] != -1 {
//         return cache[d][ amount as usize ] 
//     }
//     for i in d..coins.len() {
//         let d = coins[i];
//         if amount - d == 0 {
//             cnt += 1;
//         } else if amount > d {
//             cnt += _coin_change_(amount - d, coins, i, cache);
//         }
//     }
//     cache[d][amount as usize] = cnt;
//     cnt
// }

// this one didn't work didn't understand why it was tricky and things

// fn coin_change(amount:i32, coins:&[i32]) -> i32 {
//     // okay this sucks lets get a cache for it and then see if we can't linearize it
//     let mut cnt = 0;
//     for &d in coins {
//         if amount > d {
//             cnt += coin_change(amount - d, coins);
//         } else if amount - d == 0 {
//             cnt += 1;
//         }
//     }
//     cnt
// }
