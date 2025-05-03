
fn generate_grey(mut n:usize) -> Vec<i32> {
    let size = 1 << n;
    // let mut res = Vec::with_capacity(size);
    // // for i in 0..size {
    // //     // res.push(1<<i);
    // //     res.push(i^(i>>1));
    // // }
    // res
    (0..size).map(|i| i ^ (i >> 1)).collect()
}


fn grey_code(bytes:&[i32]) -> bool {
    // a ^ b == a xor b
    let len = bytes.len();
    if len <= 1 {return true};
    for i in 1..len {
        if (bytes[i] ^ bytes[i-1]).count_ones() > 1 {
            return false
        }
    }
    if (bytes[0] ^ bytes[len - 1]).count_ones() < 2 {
        true
    } else {
        false
    }
}


fn main() {
    let test = generate_grey(3);
    println!("Code {:?}", test);
    assert_eq!(grey_code(&[0,1,3,2]), true);
    assert_eq!(grey_code(&[0,1,2,3]), false);
    assert_eq!(grey_code(&[0,1,3,2]), true);
}
