use std::mem;


// swapped wrong vals
fn unique_paths(mut m:usize, mut n:usize) -> usize {
    if m > n { mem::swap(&mut m, &mut n) };
    // initialize to one for all m x 1 or 1 x m
    let mut prev:Vec<usize> = vec![1;m];
    let mut curr:Vec<usize> = vec![1;m];
    for _ in 1..n {
        for i in 1..m {
            curr[i] = prev[i] + curr[i-1];
        }
        mem::swap(&mut prev,&mut curr);
    }
    // swapped at the end
    prev[m-1]
}

// O efficient but not memory optimized
// fn unique_paths(mut m:usize, mut n:usize) -> usize {
//     if m > n { mem::swap(&mut m, &mut n) };
//     // initialize to one for all m x 1 or 1 x m
//     let mut dp = vec![vec![1;n];m];
//     for i in 1..m {
//         for j in 1..n {
//             dp[i][j] = dp[i-1][j] + dp[i][j-1];
//         }
//     }
//     dp[m-1][n-1]
// }



// memoized version
// fn unique_paths(mut m:usize, mut n:usize) -> usize {
//     if m > n { mem::swap(&mut m, &mut n) };
//     _unique_paths_(m, n, &mut vec![vec![0;n];m])
// }

// fn _unique_paths_(mut m:usize, mut n:usize, cache:&mut [Vec<usize>]) -> usize {
//     if m == 1 || n == 1 {
//         return 1
//     }
//     if m > n { mem::swap(&mut m, &mut n) };

//     if cache[m-1][n-1] != 0 {
//         cache[m-1][n-1]
//     } else {
//         let result =  _unique_paths_(m-1, n, cache) + _unique_paths_(m, n-1, cache);
//         cache[m-1][n-1] = result;
//         result
//     }
// }

fn main() {
    assert_eq!(1, unique_paths(2,1));
    assert_eq!(1, unique_paths(1,2));
    assert_eq!(2, unique_paths(2,2));
    assert_eq!(28, unique_paths(3,7));
}

// // memoized version
// fn unique_paths(mut m:usize, mut n:usize) -> usize {
//     if m > n { mem::swap(&mut m, &mut n) };
//     _unique_paths_(m, n, &mut vec![vec![0;n];m])
// }


// fn _unique_paths_(mut m:usize, mut n:usize, cache:&mut [Vec<usize>]) -> usize {
//     if m == 1 || n == 1 {
//         return 1
//     }
//     if m > n { mem::swap(&mut m, &mut n) };

//     if cache[m-1][n-1] != 0 {
//         cache[m-1][n-1]
//     } else {
//         let result =  _unique_paths_(m-1, n, cache) + _unique_paths_(m, n-1, cache);
//         cache[m-1][n-1] = result;
//         result
//     }
// }
