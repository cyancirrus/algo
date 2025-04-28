// // TODO: after figure out this version make it handle range of i32
// fn integer_divide(num:i32, den:i32) -> i32 {
//     // multiplication, division and mod are all unavailable
//     let mut res = 0;
//     let mut _num = num.clone();
//     while _num  >= den {
//         res += 1;
//         _num -= den;
//     }
//     res
// }

// fn integer_divide(num:i32, den:i32) -> i32 {
//     let mut res = 0;
//     let mut _num = num.clone();
//     let mut _mult = den.leading_zeros() as i32 - num.leading_zeros() as i32;
//     let mut _base = (0.._mult).map(|_| den).sum::<i32>();
//     while  den  <= _num {
//         if _base <= _num {
//             res += _mult;
//             _num -= _base;
//         } else {
//             _base -= den;
//             _mult -= 1;
//         }
//     }
//     res
// }

// fn integer_divide(num:i32, den:i32) -> i32 {
//     let mut res = 0;
//     let mut _num = num.clone();
//     while den <= _num {
//         let mut temp = den;
//         let mut mult = 1;
//         while (temp << 1) <= _num {
//             temp <<= 1;
//             mult <<= 1;
//         }
//         res += mult;
//         _num -= temp;
//     }
//     res
// }

fn integer_divide(num:i32, den:i32) -> i32 {
    let mut res = 0;
    let mut _num = num.clone().abs();
    let _den= den.clone().abs();
    while _den <= _num {
        let mut temp = _den;
        let mut mult = 1;
        while (temp << 1) <= _num {
            temp <<= 1;
            mult <<= 1;
        }
        res += mult;
        _num -= temp;
    }
    if num.signum() != den.signum() {
        -res
    } else {
        res
    }
}

fn main() {
    println!("Hello world");
    assert_eq!(integer_divide(4,10), 0);
    assert_eq!(integer_divide(21,7), 3);
    assert_eq!(integer_divide(3,2), 1);
    assert_eq!(integer_divide(4,2), 2);
    assert_eq!(integer_divide(5,2), 2);
    assert_eq!(integer_divide(100_000,2), 50_000);
    assert_eq!(integer_divide(-4,10), 0);
    assert_eq!(integer_divide(-4,-10), 0);
    assert_eq!(integer_divide(4,-10), 0);
    assert_eq!(integer_divide(-5,2), -2);
    assert_eq!(integer_divide(-3,2), -1);
    assert_eq!(integer_divide(5,-2), -2);
    assert_eq!(integer_divide(-5,-2), 2);
    assert_eq!(integer_divide(4,-2), -2);
}

