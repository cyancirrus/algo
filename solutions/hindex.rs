use std::collections::BinaryHeap;
// max h s.t. h papers with h citations
// fn hindex(references:&[usize]) -> usize {
//     let mut cross_ref = 0;
//     let n = references.len();
//     let mut ordered = BinaryHeap::with_capacity(n);
//     for s in references {
//         ordered.push(s);
//     }
//     while let Some(c) = ordered.pop() {
//         cross_ref += 1;
//         if *c < cross_ref {
//             return cross_ref - 1
//         }
//     }
//     cross_ref
// }

// fn hindex(references:&[usize]) -> usize {
//     let mut cross_ref = 0;
//     let n = references.len();
//     let mut ordered = BinaryHeap::with_capacity(n);
//     for s in references {
//         ordered.push(s);
//     }
//     while let Some(c) = ordered.pop() {
//         cross_ref += 1;
//         if *c < cross_ref {
//             return cross_ref - 1
//         }
//     }
//     cross_ref
// }

fn hindex(references:&[usize]) -> usize {
    let n = references.len();
    let mut cites = vec![0;n+1];
    for &s in references {
        if s >= n {
            cites[n] += 1;
        } else {
            cites[s] += 1;
        }
    }
    let mut cum = 0;
    for (idx, c) in cites.iter().enumerate().rev() {
        cum+= c;
        if idx <= cum {
            return idx
        }
    }
    0
}


fn main() {
    assert_eq!(3, hindex(&[3,0,5,1,4]));
    assert_eq!(3, hindex(&[4,0,5,1,4]));
    assert_eq!(1, hindex(&[1,2,1]));
    assert_eq!(2, hindex(&[1,2,3]));
}
