// fn max_profit(s:&[i32]) -> i32 {
//     let n = s.len();
//     if n <= 1 {return 0};
//     let mut buy = s[0];
//     let mut profit = 0;
//     for i in 1..s.len() {
//         let p = s[i] - buy; 
//         if p > profit {
//             profit = p;
//         }
//         if p < 0 {
//             buy = s[i];
//         }
//     }
//     profit
// }

fn max_profit(s:&[i32]) -> i32 {
    let n = s.len();
    if n <= 1 {return 0};
    let mut buy = s[0];
    let mut profit = 0;
    for &stock in &s[1..] {
        profit = profit.max(stock-buy);
        buy = buy.min(stock);
    }
    profit
}

// fn buy_and_sell(s:&[i32]) -> i32 {
//     if s.is_empty() { return 0;}
//     let mut profit = 0;
//     let mut buy = s[0];
//     let mut potential_buy = i32::MAX;
//     let mut potential_sell= 0;

//     for &stock in &s[1..] {
//         if buy < potential_sell {
//             if potential_buy < stock {
//                 profit += potential_sell - buy;
//                 buy = potential_buy;
//                 potential_sell = stock;
//                 potential_buy = i32::MAX;
//             } else if potential_sell < stock {
//                 potential_sell = stock;
//             } else if stock < potential_buy {
//                 potential_buy = stock;
//             }
//         } else if stock < buy {
//             buy = stock; 
//         } else if potential_sell < stock {
//             potential_sell = stock;
//         }
//     }
//     profit += (potential_sell - buy).max(0);
//     profit
// }

fn buy_and_sell(s:&[i32]) -> i32 {
    if s.is_empty() { return 0; }
    let mut profit = 0;

    for i in 1..s.len() {
        if s[i] > s[i-1] {
            profit += s[i] - s[i-1];
        }
    }
    profit
}

fn main() {
    // assert_eq!(5, max_profit(&[7,1,5,3,6,4]));
    // assert_eq!(0, max_profit(&[7,6,4,3,1]));
    assert_eq!(7, buy_and_sell(&[7,1,5,3,6,4]));
    assert_eq!(0, buy_and_sell(&[7,6,4,3,1]));
    assert_eq!(4, buy_and_sell(&[1, 2, 3, 4, 5]));
}
    
// for &stock in &s[1..] {
//         // have an option to execute
//         if buy < stock {
//             // have potential profit
//             if stock > potential_buy {
//                 profit += potential_sell - buy;
//                 buy = potential_buy;
//                 potential_sell = stock;
//             } else {
//                 buy = stock;
//                 // potential_buy = stock;
//             }
//         } else {
//             if buy > stock

//         }
