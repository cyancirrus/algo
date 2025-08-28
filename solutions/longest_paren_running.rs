// "(()(())"

// ((())
// ()())
// ())((
// ()(()

// ((())
// 1 2 3 3 3
// 0 0 0 1 2 

// ()(() 
// 1 1 2 3 3
// 0 1 1 1 2

// (()()
// 1 2 2 3 3
// 0 0 1 1 2

// )()((


fn longest_valid_parens(s:&str) -> usize {
    let mut max = 0;
    let mut left = 0;
    let mut right = 0;
    for c in s.chars() {
        if c == '(' { left += 1; }
        else { right += 1; }

        if left == right {
            max = max.max(2 * right);
        } else if right > left {
            left = 0;
            right = 0;
        }
    }
    let mut left = 0;
    let mut right = 0;
    for c in s.chars().rev() {
        if c == '(' { left += 1; }
        else { right += 1; }

        if left == right {
            max = max.max(2 * left);
        } else if left > right {
            left = 0;
            right = 0;
        }
    }
    max
}

// fn longest_valid_parens(s:&str) -> usize {
//     let mut stack:Vec<isize> = vec![-1];
//     let mut max_len = 0;
//     for (i, c) in s.chars().enumerate() {
//         if c == '(' {
//             stack.push(i as isize);
//         } else {
//             stack.pop();
//             if let Some(&last) = stack.last() {
//                 max_len = max_len.max(i - last as usize);
//             } else {
//                 stack.push(i as isize);
//             }
//         }
//     }
//     max_len
// }


// fn longest_valid_parens(s:&str) -> usize {
//     let n = s.len();
//     let sb = s.as_bytes();
//     let mut max = 0;
//     let mut levels = vec![0;n + n];
//     let mut left = 0;
//     let mut right = 0;
//     for i in 0..n {
//         if sb[i] == b'(' {
//             left += 1;
//         } else {
//             right += 1;
//             levels[n + left - right] += 1;
//         }
//     }
//     for i in 0..2n {
//         let c = levels[i];
//         max = max.max(c);
//     }
//     max << 1
// }

fn main() {
    // println!("longest valid parens {:?}", longest_valid_parens(")()()("));
    // println!("longest valid parens {:?}", longest_valid_parens("(()(()"));
    println!("longest valid parens {:?}", longest_valid_parens("(()(())"));
}
