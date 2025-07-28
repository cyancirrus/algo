//🔹 1. Numerical Stability & Floating Point
    // IEEE-754, error propagation, Kahan summation
    // Matrix condition numbers, stability of ops like QR/SVD
    // When to use fixed-point or arbitrary precision
    // Why it matters: You’re building tools that manipulate math. Understanding rounding error and representational quirks separates black-box ML from real tool-building.
// 🔹 4. Compilers / DSLs / Codegen
// 🔹 5. Control Theory & Signal Processing
use std::mem;
fn count_and_say(n:u8) -> Vec<u8> {
    
    let mut result = vec![1];
    let mut repr = vec![];
    for _ in 0..n-1 {
        let n = result.len();
        let mut prev = result[0];
        let mut counter = 1;
        for idx in 1..n  {
            if result[idx] == prev {
                counter += 1;
            }
            else {
                repr.push(counter);
                repr.push(prev);
                counter = 1;
                prev = result[idx];
            }
        }
        repr.push(counter);
        repr.push(prev);
        mem::swap(&mut result, &mut repr);
        repr.clear();
    }
    result
}

// 1
// 11
// 21
// 1211 
// 111221
// 312211

fn main() {
    println!("count and say {:?}", count_and_say(3));
    // 11 12 21
}
