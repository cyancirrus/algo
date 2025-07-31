use std::str::from_utf8;
use std::mem;

fn sort_colors(colors: &mut[usize]) -> &[usize]{
    let mut low = 0;
    let mut idx = 0; 
    let mut high = colors.len()-1;
    while idx <= high {
        match colors[idx] {
            0 => {
                colors.swap(low, idx);
                low += 1;
                idx += 1;
            },
            1 => {
                idx += 1;
            },
            2 => {
                colors.swap(idx, high);
                high -= 1;
            },
            _ => { break }
        }
    }
    colors
}

// fn sort_colors(colors: &mut[usize]) -> &[usize]{
//     let mut ones = 0;
//     let mut twos = colors.len()-1;
//     let mut idx = 0; 
//     while idx < colors.len() {
//         match colors[idx] {
//             0 => {
//                 if idx > ones {
//                     colors.swap(ones, idx);
//                     ones += 1;
//                     continue;
//                 }
//             },
//             1 => {
//                 if idx < ones {
//                     colors.swap(ones, idx);
//                     ones -= 1; 
//                     continue;
//                 } else if idx > twos {
//                     colors.swap(twos, idx);
//                     twos += 1;
//                     continue;
//                 }
//             },
//             2 => {
//                 if idx < twos {
//                     colors.swap(idx, twos);
//                     twos-=1;
//                     continue
//                 }
//             },
//             _ => { break }
//         }
//         idx += 1;
//     }
//     colors
// }

fn main() {
    assert_eq!(&vec![0,0,1,1,2,2,2], sort_colors(&mut [0,2,1,2,2,1,0]));
    assert_eq!(&vec![0,1,2], sort_colors(&mut [2,0,1]));
    assert_eq!(&vec![0,0,1,2], sort_colors(&mut [2,0,1,0]));
}
