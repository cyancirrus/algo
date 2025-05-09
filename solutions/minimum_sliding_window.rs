use std::collections::HashMap;

// fn minimum_window_substring<'a>(needle:&'a str, haystack:&'a str) -> Option<&'a str> {
//     let mut freq:HashMap<u8, i32> = HashMap::new();
//     let hay = haystack.as_bytes();
//     for ch in needle.bytes() {
//         *freq.entry(ch).or_insert(0) -= 1
//     }
//     let (mut l, mut r) = (0, 0);
//     let mut formed = false;
//     let mut position = [0, haystack.len()];

//     while l <= r && r < hay.len() {
//         if formed {
//             println!("l,r ({}, {})", l,r);
//             let letter = hay[l];
//             if freq.contains_key(&letter) {
//                 *freq.get_mut(&letter).unwrap() -=1;
//                 if freq[&letter] < 0 {
//                     formed = false;
//                     }
//                 }
//             if r - l < position[1] - position[0] {
//                 position = [l, r];
//             }
//             l += 1;
//         } else {
//             let letter = hay[r];
//             if freq.contains_key(&letter) {
//                 *freq.get_mut(&letter).unwrap() += 1 ;
//             }
//             if freq.values().all(|&v| v >= 0) {
//                 formed = true;
//             }
//             r += 1;
//         }
//     }
//     if position[1] < haystack.len() {
//         Some(&haystack[position[0]..position[1]])
//     } else {
//         None
//     }
// }

fn minimum_window_substring<'a>(needle:&'a str, haystack:&'a str) -> Option<&'a str> {
    let mut freq:HashMap<u8, i32> = HashMap::new();
    let hay = haystack.as_bytes();
    for ch in needle.bytes() {
        *freq.entry(ch).or_insert(0) -= 1
    }
    let (mut l, mut r) = (0, 0);
    let mut have = 0;
    let need = freq.len();
    let mut formed = false;
    let mut position = [0, haystack.len()];

    while r < hay.len() {
        if formed {
            let letter = hay[l];
            if let Some(val) = freq.get_mut(&letter) {
                *val -=1;
                if freq[&letter] < 0 {
                    have -= 1;
                    formed = false;
                    }
                }
            if r - l < position[1] - position[0] {
                position = [l, r];
            }
            l += 1;
        } else {
            let letter = hay[r];
            if let Some(val) = freq.get_mut(&letter) {
                *val += 1;
                if *val == 0 {
                    have +=1;
                }
                if have == need {
                    formed = true;
                }
            }
            r += 1;
        }
    }
    if position[1] < haystack.len() {
        Some(&haystack[position[0]..position[1]])
    } else {
        None
    }
}

fn main() {
    let test = minimum_window_substring("hello", "  hell world");
    match test {
        Some(r) => {
            println!("Found it! {:?}", r);
        },
        _ => {
            println!("Match not found");
        }
    }
}
