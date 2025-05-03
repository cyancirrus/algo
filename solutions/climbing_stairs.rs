use std::collections::HashMap;

// fn climbing_stairs(n:i32) -> usize {
//     if n == 0 {
//         1 
//     } else if n > 0 {
//         climbing_stairs(n - 1) + climbing_stairs(n-2)
//     } else {
//         0
//     }
// }


// fn climbing_stairs(n:i32) -> usize {
//     fn climb(n:i32, memo:&mut HashMap<i32, usize>) -> usize {
//         if n < 0 {
//             0
//         } else if n == 0 {
//             1
//         } else if let Some(&v) = memo.get(&n) {
//             v 
//         } else {
//             let res = climb(n-1, memo) + climb(n -2, memo);
//             memo.insert(n, res);
//             res
//         }
//     }
//     climb(n, &mut HashMap::new())
// }

fn climbing_stairs(n:i32) -> usize {
    let mut memo = HashMap::new();
    memo.insert(0, 1);
    memo.insert(1, 1);
    for i in 2..=n {
        let one = memo.get(&(i-1)).unwrap_or(&0);
        let two = memo.get(&(i-2)).unwrap_or(&0);
        memo.insert(i, one + two);
    }
    memo[&n]
}


fn main() {
    println!("Climbing stairs {}", climbing_stairs(5));
}
