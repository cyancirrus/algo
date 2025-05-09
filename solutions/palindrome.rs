// fn pallindrome(x:usize) -> bool {
//     let bin = format!("{:b}", x);
//     let bytes = bin.as_bytes();
//     let len = bytes.len();

//     for i in 0..len / 2 {
//         if bytes[i] != bytes[len-i] {
//             return false
//         }
//     }
//     true
// }

fn pallindrome(mut x:usize) -> bool {
    let mut digits:Vec<usize> = Vec::new();
    while x > 0 {
        // Repeated division by base 10
        let m = x % 10;
        x = x / 10;
        digits.push(m)
    }
    let len = digits.len();
    for i in 0..len / 2 {
        if digits[i] != digits[len - i - 1] {
            return false
        }
    }
    true
}

fn pallindrome_base_two_v1(x:u32) -> bool {
    // usize is archetecture dependent so we need unsigned 32
    // using 32 to handle the case when all zeros
    let digits = 32 - x.leading_zeros();
    let mut offset = 1;
    while offset < digits {
        // get the value in the ones position
        let right = (x >> digits - offset - 1) & 1;
        let left = (x >> offset ) & 1;
        if right != left {
            return false
        }
        offset+=1;
    }
    true
}

fn pallindrome_base_two(x:u32) -> bool {
    if x == 0 {return true};
    let mut left = 31 - x.leading_zeros();
    let mut right = 0;
    while left > right {
        let left_bit = (x >> left) & 1;
        let right_bit = (x >> right) & 1;
        if left_bit != right_bit {
            return false
        }
        left-=1;
        right+=1;
    }
    return true
}

fn main() {
    assert_eq!(true, pallindrome_base_two(0));
    assert_eq!(true, pallindrome_base_two(1));
    assert_eq!(true, pallindrome_base_two(3));
    assert_eq!(true, pallindrome_base_two(9));
    assert_eq!(true, pallindrome_base_two(27));
    assert_eq!(false, pallindrome_base_two(10));
}
