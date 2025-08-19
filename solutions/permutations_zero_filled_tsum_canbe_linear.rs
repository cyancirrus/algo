
// 3 6 -> looks like it's (n+1)!
// 1 1 - > 3 
// 111 -> 7?
//  111, 11, 11, (one overlap in the middle)
//  could reframe this as like every option has the ability to included or not included
//  n c 1 + n c 2 + ....
//  n ! (1!) n! / (2!) + ... + n! / n !
//  => n! ^c^-1
//  => n! (1!) + 2!

// f(n) = ( n + 1 )/ 2


fn number_zero_filled(nums:&[usize]) -> usize {
    let mut cnt = 0;
    let mut total = 0;
    for &n in nums {
        if n == 0 {
            cnt += 1;
            total += cnt;
        } else {
            cnt = 0;
        }
    }
    total
}

// // adding one more = previous + previous + 1 ?
// fn number_zero_filled(nums:&[usize]) -> usize {
//     let mut cnt = 0;
//     let mut total = 0;
//     for &n in nums {
//         if n == 0 {
//             cnt += 1;
//         } else {
//             total += cnt * (cnt + 1) / 2;
//             cnt = 0;
//         }
//     }
//     total += cnt * (cnt + 1) / 2;
//     total
// }


fn main() {
    println!("testing {:?}", number_zero_filled(&[1,3,0,0,2,0,0,4]));
    println!("testing {:?}", number_zero_filled(&[0,0,0,2,0,0]));
}
