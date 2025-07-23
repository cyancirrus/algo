// strings are mapped 1->a, 26->z
// strings both can be higher (if left (1,0..9), (2,0..6)
// but also strings can post-determine if encountering a zero
//
// but if we post encounter a zero we can just -=1
// so i think we just need a like previous and scan

// 1110
// 1 -> 1
// 11 -> 2
// 111 -> 3
// 1110 -> 2
// (1, 1, 10), (11, 10)


// 1111
// 1 -> 1
// 11 -> 2
// 111 -> 3
// 1111 -> 4
// (1, 1, 1, 1), (11, 1, 1) (11, 11), (1, 11, 1), (1,1, 11)

// slide 2 window if we hit a zero, we only have one option -> do nothing
// if we hit a number between 10 and 26 add this as valid as well
// 10 included b/c we skip on b'0'

fn decode(encoded:&str) -> u8 {
    let encoded = encoded.as_bytes();
    if encoded[0] == b'0' {
        return 0;
    }
    let mut prev = 0;
    let mut curr = 1;

    for i in 0..encoded.len() {
        let mut ways = 0;
        if b'0' != encoded[i] {
            ways += curr;
        }
        // check if we have 2 digit number in history 
        if i > 0 {
            let num = (encoded[i-1] - b'0')* 10 + encoded[i] - b'0';
            if 10 <= num && num <= 26 {
                ways += prev;
            }
        }
        prev = curr;
        curr = ways;
    }
    curr
}


// minus one would work but would need a 2 window to handle case like
// 10 ie we only minus if we previous added another way (2 window)
// fn decode(encode:&str) -> u8 {
//     let encode = encode.as_bytes();
//     let mut ways = 0;
//     if encode[0] == b'0' {
//         return ways;
//     }
//     ways+=1;
//     let mut previous = encode[0];
//     for &s in encode[1..].iter() {
//         println!("ways {ways:}");
//         if s == b'0' {
//             if b'1' == previous || previous == b'2' {
//                 ways -= 1;
//             } else {
//                 return 0;
//             }
//         } else if previous == b'1' {
//             ways += 1;
//         } else if previous == b'2' && b'0' < s && s <= b'6' {
//             ways += 1;
//         } else if s < b'0' || s > b'9' {
//             return 0;
//         }
//         previous = s;
//     }
//     ways
// }


fn main() {
    assert_eq!(2, decode(&"1110"));
    assert_eq!(0, decode(&"0321"));
    assert_eq!(3, decode(&"111"));
    assert_eq!(2, decode(&"12"));
    assert_eq!(3, decode(&"121"));
    assert_eq!(3, decode(&"226"));
    assert_eq!(1, decode(&"10110"));
    assert_eq!(1, decode(&"1010"));
}
