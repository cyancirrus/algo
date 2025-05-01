fn sqrt(x:usize) -> usize {
    let (mut l, mut u)  = (0,  (x + 1) / 2,);
    loop {
        // To help break ties and ensure always make progress
        let m = (l + u + 1) / 2;
        let square = m * m;
        if l == u || square == x{
            return m
        } else if square < x {
            // if we undershoot could still be minimum integer
            l = m;
        } else if square > x {
            // the highest integer which could be valid
            u = m - 1;
        }
    }
}

// fn pow(x:f32, n:usize) -> f32 {
//     // Current implementation for positive n
//     let mut k = 0;
//     let mut res = 1 as f32;
//     while k < n {
//         if k == 0 {
//             res *= x;
//             k += 1;
//         } else if (n - k) / k  > 0 {
//             // speeds up once but not again
//             res *= res;
//             k += k;
//         } else {
//             res *= x;
//             k += 1;
//         }
//     }
//     res
// }

// fn pow(x:f32, n:usize) -> f32 {
//     let mut k = 0;
//     let mut res = 1.0 ;
//     // Better speed up finds the max x^(2*k) s.t. 2*k < n
//     while k < n {
//         let mut p = 0;
//         let mut temp = 1.0;
//         loop {
//             if p == 0  {
//                 temp *= x;
//                 p += 1;
//             } else if (n - k - p)/p > 0 {
//                 temp *= temp;
//                 p += p;
//             } else {
//                 k += p;
//                 res *= temp;
//                 break;
//             }
//         }
//     }
//     res
// }

fn pow(mut x:f32, mut n:i32) -> f32 {
    let mut res = 1.0;
    while n.abs() > 0 {
        if n.abs() % 2 == 1 {
            if n > 0 {
                res *= x;
            } else {
                res /= x;
            }
        }
        n /= 2;
        x *= x;
    }
    res
}

fn main() {
    // assert_eq!(3, sqrt(12),"12 failed");
    // assert_eq!(3, sqrt(13),"13 failed");
    // assert_eq!(3, sqrt(14),"14 failed");
    // assert_eq!(3, sqrt(15),"15 failed");
    // assert_eq!(3, sqrt(10),"10 failed");
    // //
    // // Current passes
    // assert_eq!(3, sqrt(9),"9 failed");
    // assert_eq!(4, sqrt(16),"16 failed");
    // assert_eq!(1, sqrt(1),"1 failed");
    // assert_eq!(0, sqrt(0),"0 failed");

    assert_eq!(pow(2.0, 0), 1.0);
    assert_eq!(pow(2.0, 1), 2.0);
    assert_eq!(pow(2.0, 3), 8.0);
    assert_eq!(pow(5.0, 4), 625.0);
    assert!((pow(3.0, 5) - 243.0).abs() < 1e-6);
    assert_eq!(pow(2.0, -1), 0.5);
    assert_eq!(pow(2.0, -2), 0.25);
    assert_eq!(pow(2.0, -3), 0.125);
}
