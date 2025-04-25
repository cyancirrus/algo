use std::time::Instant;

// #[derive(Debug)]
// pub struct Matcher {
//     index:usize,
//     cursor:usize,
// }

// fn substring_search(haystack:&str, needle:&str) -> i16 {
//     for i in 0..=haystack.len() - needle.len() {
//         if haystack[i..i+needle.len()] == *needle {
//             return i as i16
//         }
//     }
//     -1
// }

// fn substring_search(haystack:&str, needle:&str) -> i16 {
//     if needle.is_empty() {
//         return 0 as i16
//     }
//     let _haystack = haystack.as_bytes();
//     let _needle = needle.as_bytes();
//     let mut potentials:Vec<Matcher> = Vec::with_capacity(haystack.len() - needle.len());
//     for i in 0..=haystack.len() - needle.len() {
//         for j in 0..potentials.len() {
//             let potential = &mut potentials[j];
//             if _haystack[i] == _needle[potential.cursor] {
//                 potential.cursor += 1;
//                 if potential.cursor == needle.len(){
//                     return potential.index as i16
//                 }
//             } else {
//                 potentials.remove(j);
//             }
//         }
//         if _haystack[i] == _needle[0] {
//             potentials.push( Matcher{ index:i, cursor:1} )
//         }
//         println!("potentials {:?}", potentials);
//     }
//     -1
// }

// fn substring_search(haystack:&str, needle:&str) -> i16 {
//     if needle.is_empty() {
//         return 0 as i16
//     }
//     let _haystack = haystack.as_bytes();
//     let _needle = needle.as_bytes();
//     let mut potential:Matcher = Matcher{ index:0, cursor: 0 };
//     for i in 0..haystack.len() {
//         if _haystack[i] == _needle[potential.cursor] {
//             if potential.cursor < _needle.len() - 1 {
//                 potential.cursor +=1;
//             } else {
//                 return potential.index as i16
//             }
//         } else if {
//                 !(i == potential.index)
//                 & (_haystack[potential.cursor] == _haystack[potential.index])
//             } {
//             potential.index += 1;
//         } else {
//             potential.index = i+1;
//             potential.cursor = 0;
//         }
//     }
//     -1 as i16
// }

// fn substring_search(haystack:&str, needle:&str) -> i16 {
//     if needle.is_empty() {
//         return 0 as i16
//     }
//     let _haystack = haystack.as_bytes();
//     let _needle = needle.as_bytes();
//     let mut index = 0;
//     let mut cursor = 0;
//     for i in 0..haystack.len() {
//         if _haystack[i] == _needle[cursor] {
//             cursor +=1;
//             if cursor == _needle.len() {
//                 return index as i16
//             }
//         } else if {
//                 !(i == index)
//                 & (_haystack[cursor] == _haystack[index])
//             } {
//             index += 1;
//         } else {
//             index = i+1;
//             cursor = 0;
//         }
//     }
//     -1 as i16
// }
// fn rolling_hash(byte:u8) -> u32 {
//     // let mut hash = 0u32;
//     // hash ^= (byte as u32) << 3;
//     // (byte as u32).wrapping_mul(0x9E373) & 0xFFFFFFFF
//     byte as u32
// }

const BASE:i32 = 256;
const MOD:i32 = 101;

fn rolling_hash(bytes:&[u8]) -> i32 {
    let mut hash = 0;
    let length = bytes.len();
    for i in 0..length {
        hash = (hash * BASE + bytes[i] as i32) % MOD
    }
    hash
}

fn mod_pow(base:&i32, modu:&i32, n:&usize) -> i32 {
    let mut result = 1 as i32;
    let mut _power = n.clone();
    let mut _base = base.clone() % modu;
    
    while _power > 0 {
        if _power % 2 == 1 {
            result = ( result * _base ) % modu;
        }
        _power /= 2;
        _base = (_base * _base) % modu;
    }
    result
}

fn advance_hash_window(new:&u8, old:&u8, n:usize, mut hash:i32) -> i32 {
    let removed = (*old as i32 * mod_pow(&BASE, &MOD, &(n - 1)) ) % MOD;
    hash = (hash + MOD - removed) % MOD;
    hash = (hash * BASE + *new as i32) % MOD;
    hash
}

fn substring_search(haystack:&str, needle:&str) -> i16 {
    if needle.is_empty() {
        return -1
    } else if needle.len() > haystack.len() {
        return -1
    }
    let _hay= haystack.as_bytes();
    let _needle = needle.as_bytes();
    let n_len = needle.len(); 
    
    let mut hash_hay = rolling_hash(&_hay[..n_len]);
    let hash_needle = rolling_hash(_needle);
    for i in 0..=haystack.len() - needle.len() {
        if hash_hay == hash_needle {
            return i as i16
        }
        else if i < haystack.len() - needle.len()  {
            hash_hay = advance_hash_window(&_hay[i + n_len], &_hay[i], n_len, hash_hay);
        }
    }
    -1
}



fn main() {
    let haystack = "abcd".repeat(5_000) + "abcdwhatneedle" + &"a".repeat(10_000);
    let needle = "abcdwhatneedle";

    let start_std = Instant::now();
    let std_result = (&haystack).find(needle);
    let duration_std = start_std.elapsed();

    let start_rk = Instant::now();
    // let rk_result = substring_search(&haystack, needle);
    let rk_result = substring_search(&haystack, needle);
    let duration_rk = start_rk.elapsed();

    println!("std::find result: {:?}, took {:?}", std_result, duration_std);
    println!("Rabin-Karp result: {:?}, took {:?}", rk_result, duration_rk);
}




// fn custom_adder(x:usize) -> impl Fn(usize) -> usize {
//     move |y| x + y
// }



// fn main() {
//     let x = 32;
//     let adder = custom_adder(x);
//     println!("hello world");

//     println!("Should be 64 {}", adder(32));
// }
