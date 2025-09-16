use std::collections::HashSet;
use std::collections::VecDeque;

fn perfect_squares(n:usize) -> usize {
    let mut counts:Vec<usize> = (0..=n).collect();
    for factor in 2.. {
        let square = factor * factor;
        if square > n { break; }
        for base in square..=n {
            counts[base] = counts[base].min(counts[base-square] + 1);
        }
    }
    counts[n]
}
// fn perfect_squares(n:usize) -> usize {
//     let mut counts:Vec<usize> = (0..=n).collect();
//     for factor in 2..=n/2 {
//         let square = factor * factor;
//         if square > n  { break; }
//         for base in 0..=n-square {
            
//             counts[square + base] = counts[square + base].min(counts[base] + 1);
//         }
//         println!("counts {counts:?}");
//     }
//     println!("counts {counts:?}");
//     counts[n]
// }


fn main() {
    println!(
        "result {:?}",
        perfect_squares(12)
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
