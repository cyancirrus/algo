//🔹 1. Numerical Stability & Floating Point
    // IEEE-754, error propagation, Kahan summation
    // Matrix condition numbers, stability of ops like QR/SVD
    // When to use fixed-point or arbitrary precision
    // Why it matters: You’re building tools that manipulate math. Understanding rounding error and representational quirks separates black-box ML from real tool-building.
// 🔹 4. Compilers / DSLs / Codegen
// 🔹 5. Control Theory & Signal Processing


fn single_number(nums:&[i32]) -> i32 {
    let mut single = 0;
    let mut double = 0;
    
    for &n in nums {
        double |= n & single;
        single ^= n;
        let both = single & double;
        single &= !both;
        double &= !both;
    }
    single
}



fn main() {
    println!("test {:?}", single_number(&[1,1,3,2,3,2,9,3,1]));
}
