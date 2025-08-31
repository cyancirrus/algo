
fn maximum_gap(nums:&[usize]) -> usize {
    let n = nums.len();
    if n < 2 { return 0; }
    let mut buckets = vec![None;n-1];
    let mut min = usize::MAX;
    let mut max = usize::MIN;
    for &n in nums {
        min = min.min(n);
        max = max.max(n);
    }
    let gap = (max - min)/(n-1);
    for &n in nums {
        if n == min || n == max { continue; }
        let idx = (n - min) /gap;
        buckets[idx] = Some(match buckets[idx]{
            Some((mn, mx)) => { ( n.min(mn), n.max(mx)) },
            None => { (n,n) },
        })
    }
    let mut prev = min;
    let mut res = 0;
    for b in buckets {
        if let Some((bmin, bmax)) = b {
            res = res.max(bmin - prev);
            prev = bmax;
        }
    }
    res = res.max(max - prev);
    res
}
// fn maximum_gap(nums:&[usize]) -> usize {
//     let n = nums.len();
//     let mut buckets = vec![(None, None);n-1];
//     let mut min = usize::MAX;
//     let mut max = usize::MIN;
//     for &n in nums {
//         min = min.min(n);
//         max = max.max(n);
//     }
//     let gap = (max - min)/(n-1);
//     for &n in nums {
//         if n == min || n == max { continue; }
//         let idx = (n - min) /gap;
//         match buckets[idx] {
//             (Some(mn), Some(mx)) => {
//                 buckets[idx] = ( Some(n.min(mn)), Some(n.max(mx)),);
//             },
//             (None, None) => {
//                 buckets[idx] = (Some(n), Some(n));
//             },
//             _ => {},
//         }
//     }
//     let mut prev = min;
//     let mut res = 0;
//     for b in buckets {
//         match b {
//             (Some(bmin), Some(bmax)) => {
//                 res = res.max(bmin - prev);
//                 prev = bmax;
//             },
//             _ => { continue; }
//         }
//     }
//     res = res.max(max - prev);
//     res

// }


fn main() {
    println!(
        "result {:?}",
        maximum_gap(&[3,6,9, 1])
    );
}
