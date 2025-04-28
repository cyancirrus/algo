fn byte_distance(byte:u8) -> u32 {
    assert!(byte >= b'0' && byte <= b'9', "Byte must be a digit");
    (byte - b'0') as u32
}


// fn string_to_integer(s: &str) -> u32 {
//     let mut position = 1;
//     let mut sum = 0;
//     for byte in s.as_bytes().iter().rev() {
//         sum += byte_distance(*byte) * position;
//         position *= 10;
//     }
//     sum
// }

// fn string_to_integer(s: &str) -> u32 {
//     let mut sum = 0;
//     for byte in s.bytes() {
//         sum = sum * 10 + byte_distance(byte);
//     }
//     sum
// }

fn string_to_integer(s:&str) -> Result<i32, &'static str> {
    let mut sum = 0;
    let mut sign:i32 = 1;
    let mut started = false;
    for byte in s.bytes() {
        if byte == b'-' {
            if started {
                return Err("found - in the middle of the number");
            }
            sign = -1;
            started = true;
        } else if b'0' <= byte && byte <= b'9' {
            let digit = (byte - b'0') as i32;
            if sign == -1 {
                if sum < (i32::MIN + digit) / 10 {
                    return Err("Underflow")
                }
            } else if sign == 1 {
                if sum > (i32::MAX - digit) / 10 {
                    return Err("Overflow")
                }
            }
            started = true;
            sum = sum * 10 + sign * digit;
        } else {
            return Err("Byte out of range");
        }
    }
    Ok(sum)
}



fn main() {
    let num = string_to_integer("-1234");
    match num {
        Ok(value) => {
            println!("Conversion 1234 {}", value);
        },
        Err(msg) => {
            println!("Unsuccessful parsing {}", msg);
        }
    }
}
