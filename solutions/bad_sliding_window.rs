// use std::collections::HashMap;

// fn contains(x:&str, y:&str) -> bool {
//     // Some type of x contains y
//     // simple implementation
//     for ch in x.chars() {
//         if !y.contains(ch) {
//             return false
//         }
//     }
//     true
// }

// // fn minimum_window_substring<'a>(search:&'a str, corpus:&'a str) -> Option<&'a str> {
// //     if contains(search, corpus) {
// //         let left = minimum_window_substring(search, &corpus[1..]);
// //         let right = minimum_window_substring(search, &corpus[0..corpus.len() - 1]);
// //         match (left, right) {
// //             (Some(l), Some(r)) => {
// //                 if l.len() < r.len() {
// //                     return left;
// //                 } else {
// //                     return right;
// //                 }
// //             },
// //             (Some(_), None) => { return left; },
// //             (None, Some(_)) => { return right; },
// //             (_, _) => { return None; },
// //         }
// //     }
// //     None
// // }
// fn minimum_window_substring<'a>(search:&'a str, corpus:&'a str) -> Option<&'a str> {
//     // Selects a window in which there is at least one occurance for every element in 'search'
//     // Ie 'abba' will match 'ab'
//     let bounds = [0, corpus.len() - 1];
//     let min_range = min_logic(&bounds, search, corpus, &mut HashMap::new());
//     match min_range {
//         Some(range) => {Some(&corpus[range[0]..range[1]])},
//         None => {return None},
//     }
    
// }

// fn min_logic<'a>(bounds:&[usize;2], search:&str, corpus:&str, memo:&mut HashMap<[usize;2], Option<[usize;2]>>) -> Option<[usize;2]> {
//     if let Some(result) = memo.get(bounds) {
//         return *result;
//     }
//     let window = &corpus[bounds[0]..bounds[1]];
//     if bounds[0] >= bounds[1] || !contains(search, window) {
//         memo.insert(*bounds, None);
//         return None
//     }
//     let l = [bounds[0] + 1, bounds[1]];
//     let r = [bounds[0], bounds[1] - 1];
//     let left = min_logic(&l, search, corpus, memo);
//     let right = min_logic(&r, search, corpus, memo);
//     let result = match (left, right) {
//         (Some(l), Some(r)) => {
//             if l[1] - l[0] < r[1] - r[0] { left } else { right }
//         },
//         (Some(_), None) => { left},
//         (None, Some(_)) => { right },
//         (_, _) => {
//             Some(*bounds)
//         },
//     };
//     memo.insert(*bounds, result);
//     result
// }

use std::collections::HashMap;

fn contains_all(needle: &str, haystack: &str) -> bool {
    let mut freq = HashMap::new();
    for ch in needle.chars() {
        *freq.entry(ch).or_insert(0) += 1;
    }
    for ch in haystack.chars() {
        if let Some(count) = freq.get_mut(&ch) {
            *count -= 1;
        }
    }
    freq.values().all(|&v| v <= 0)
}

fn minimum_window_substring<'a>(search: &str, corpus: &'a str) -> Option<&'a str> {
    let mut memo = HashMap::new();
    let bounds = [0, corpus.len()];
    min_window(&bounds, search, corpus, &mut memo).map(|[start, end]| &corpus[start..end])
}

fn min_window<'a>(
    bounds: &[usize; 2],
    search: &str,
    corpus: &'a str,
    memo: &mut HashMap<[usize; 2], Option<[usize; 2]>>
) -> Option<[usize; 2]> {
    if let Some(&result) = memo.get(bounds) {
        return result;
    }
    if bounds[0] >= bounds[1] || bounds[1] > corpus.len() {
        return None;
    }

    let window = &corpus[bounds[0]..bounds[1]];
    if !contains_all(search, window) {
        memo.insert(*bounds, None);
        return None;
    }

    let left_bounds = [bounds[0] + 1, bounds[1]];
    let right_bounds = [bounds[0], bounds[1] - 1];

    let left = min_window(&left_bounds, search, corpus, memo);
    let right = min_window(&right_bounds, search, corpus, memo);

    let result = match (left, right) {
        (Some(l), Some(r)) => {
            if l[1] - l[0] <= r[1] - r[0] { Some(l) } else { Some(r) }
        }
        (Some(l), None) => Some(l),
        (None, Some(r)) => Some(r),
        (None, None) => Some(*bounds),
    };

    memo.insert(*bounds, result);
    result
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
