#![allow(warnings)]
// fn next_permutation(mut perm:Vec<i32>) -> Vec<i32> {
//     let len = perm.len();
//     for upper in 1..perm.len() {
//         for lower in 0..upper {
//             // Iterate from least significant element first
//             if perm[len - lower - 1] > perm[len - upper - 1] {
//                 perm.swap(len - lower - 1, len - upper - 1);
//                 // Simple swap
//                 // perm[len - upper  ..].reverse();
//                 for straighten in len-upper..(len + upper) / 2 {
//                     // The array will be completely backwards until that point
//                     // So we just need to reverse the elements
//                     perm.swap(len - upper + straighten - 1, len - straighten)
//                 }
//                 return perm
//             }
//         }
//     }
//     perm.into_iter().rev().collect()
// }

fn next_permutation(perm:&mut Vec<i32>) -> &mut Vec<i32> {
    if perm.len() < 2 {
        return perm
    }
    let mut i = perm.len() - 2;
    let mut j = perm.len() - 1;
    // Detect first digit with greater than bit
    while i > 0 && perm[i] >= perm[i+1] {
        i-=1;
    }
    // No sign-change found degenerate case end permutation and reset state
    if i == 0 && perm[i] >= perm[i+1] {
        perm.reverse();
        return perm
    }
    // Need to the first number which is greater than pivot point
    while perm[j] <= perm[i] {
        j-=1
    }
    perm.swap(i, j);
    // Increment after the value where you swapped
    perm[i+1..].reverse();
    perm
}

fn factorial(n:usize) -> usize {
    (1..=n).product()
}


fn illustrate_solution(mut perm:Vec<i32>) {
    let len = perm.len();
    let mut _perm = &mut perm;
    for i in 0..factorial(len) + 1 {
        println!("Permutation {} :: {:?}", i, _perm);
        _perm = next_permutation(_perm);
    }
}

fn main() {
    // let perm = vec![1,2,3,4];
    let perm = vec![1,2,3];
    // let perm = vec![1,2,3];
    illustrate_solution(perm);
}
