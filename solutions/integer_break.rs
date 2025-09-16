use std::collections::HashSet;
use std::collections::VecDeque;

// fn integer_break(n:usize) -> usize {
//     if n <= 2 { return 1; }
//     let mut max = 1;
//     for denom in 2..=n/2 {
//         let base = n / denom;
//         let remain = n % denom;
//         max = max.max( (base + 1).pow(remain as u32) * base.pow((denom - remain) as u32) );
//     }
//     max
// }

fn integer_break(n:usize) -> usize {
    if n == 0 { return 0; }
    if n <= 3 { return n-1;}
    let k = (n/3) as u32;
    match n % 3 {
        0 => 3usize.pow(k),
        1 => 3usize.pow(k - 1) * 4,
        2 => 3usize.pow(k) * 2,
        _ => unreachable!(),
    }
}

// fn integer_break(n:usize) -> usize {
//     if n <= 2 { return 1; }
//     let mut max = 1;

//     for denom in n/3..=(n + 2)/3 {
//         let base = n / denom;
//         let remain = n % denom;
//         max = max.max( (base + 1).pow(remain as u32) * base.pow((denom - remain) as u32) );
//     }
//     max
// }

// fn integer_break(n:usize) -> usize {
//     if n == 2 { return 1; }
//     if n == 3 { return 2; }
//     let mut prod = 1;
//     let mut n = n;
//     while n > 4 {
//         prod *= 3;
//         n-=3;
//     }
//     prod * n
// }

fn main() {
    println!(
        "result {:?}",
        integer_break(10)
        // is_interleaving_dp("aabcc", "abcd", "aabcc")
        // maximum_mul_subarray(&[2,-1])
        // can_be_valid("))()))","010100")
        // can_be_valid("))","01")
        // maximum_subarray(&[-2,1,-3,4,-1,2,1,-5,4])
    );
    // println!(
    //     "result {:?}",
    //     // maximum_mul_subarray(&[2,-1])
    //     maximum_mul_subarray(&[2,3,-2,4])
    //     // maximum_subarray(&[-2,1,-3,4,-1,2,1,-5,4])
    // );
    // println!(
    //     "result {:?}",
    //     maximum_add_subarray(&[5,4,-1,7,8])
    //     // maximum_subarray(&[-2,1,-3,4,-1,2,1,-5,4])
    // );
}
