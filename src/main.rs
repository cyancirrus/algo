#![allow(warnings)]
fn next_permutation(mut perm:Vec<i32>) -> Vec<i32> {
    let _length = perm.len();
    for upper in 1..perm.len() {
        for lower in 0..upper {
            // Iterate from least significant element first
            if perm[_length - lower - 1] > perm[_length - upper - 1] {
                perm.swap(_length - lower - 1, _length - upper - 1);
                perm[_length - upper  ..].reverse();
                // println!("Reversing: ( {}: {})", _length - upper + 1, _length);
                // for straighten in upper..(lower + _length)/2{
                //     // The array will be completely backwards until that point
                //     // So we just need to reverse the elements
                //     // ** DEBUG STATEMENTS
                //     println!("Length {}, straighten {}, lower {}, upper {}", _length, straighten, lower, upper);
                //     println!("Reversing: ( {}, {} )", straighten+1, _length + lower - straighten);
                //     // ** DEBUG STATEMENTS
                //     perm.swap(straighten, _length + lower - straighten)
                // }
                // println!("** Permutation :: {:?}",  perm);
                return perm
            }
        }
    }
    println!("Reversing to original have reached max perms");
    perm.into_iter().rev().collect()
}

fn factorial(n:usize) -> usize {
    (1..=n).product()
}


fn illustrate_solution(mut perm:Vec<i32>) {
    for i in 0..factorial(perm.len()) + 1 {
    // for i in 0..6 {
        println!("Permutation {} :: {:?}", i, perm);
        perm = next_permutation(perm);
    }
}

fn main() {
    let perm = vec![1,2,3,4];
    // let perm = vec![1,2,3];
    illustrate_solution(perm);
}
