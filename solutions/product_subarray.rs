use std::mem;

fn maximum_product(nums:&[i32]) -> i32 {
    if nums.is_empty() { return 0 };
    let mut curr_max = nums[0];
    let mut curr_min = nums[0];
    let mut result = nums[0];

    for &val in &nums[1..] {
        if val < 0 {
            mem::swap(&mut curr_max, &mut curr_min);
        }

        curr_max = val.max(curr_max * val);
        curr_min = val.min(curr_min * val);
        result = result.max(curr_max);
    }
    result
}


// fn maximum_product(nums:&[i32]) -> i32 {
//     // doesn't work with a series of negatives
//     if nums.is_empty() { return 0 };
//     let mut max = 0;
//     // handles even runs
//     let mut even_product = 0;
//     // handles odd runs
//     let mut odd_product = 0;
//     // run product is a strict accumulator
//     let mut run_product = 1;

//     for &val in nums {
//         // reinitialize products
//         if val == 0 {
//             even_product = 0;
//             odd_product = 0;
//             run_product = 1;
//         } else {
//             // initialize even to accumulate
//             if val > 0 && even_product == 0 {
//                 even_product = 1;
//             // initialize odd to accumulate
//             } else if val < 0 && odd_product == 0 {
//                 odd_product = 1;
//             }
//             // accumulate and reset if switch signs
//             run_product *= val;
//             even_product *= val.max(0);
//             odd_product *= val.min(0);
//         }
//         // track the maximum
//         if run_product > max {
//             max = run_product;
//         } else if even_product > max {
//             max = even_product;
//         } else if odd_product > max {
//             max = odd_product;
//         }
//     }
//     max
// }

fn main() {
    assert_eq!(10, maximum_product(&[-2, 5, 2]));
    assert_eq!(10, maximum_product(&[-2, 5, -1]));
    assert_eq!(10, maximum_product(&[-10, 2, 5]));
    assert_eq!(10, maximum_product(&[2, 5]));
    assert_eq!(10, maximum_product(&[2, 5, -3]));
    assert_eq!(15, maximum_product(&[2, 5, -3, 0, 3, 5]));
    assert_eq!(15, maximum_product(&[2, 5, -3, 0, -1, 3, 5]));
    assert_eq!(15, maximum_product(&[2, 5, -3, 3, 5]));
    assert_eq!(150, maximum_product(&[2, 5, -3, -1, 5]));
    assert_eq!(900, maximum_product(&[2, 5, -3, 3, 5,-1, 2]));
    assert_eq!(60, maximum_product(&[2, -5, -2, 3]));
    assert_eq!(12, maximum_product(&[-2, -3, -4]));

}

// need to do something smarter -- how can i track a new even product 
// "even product" -> hit odd number, initialized "odd product"
// "even numer" -> reinitialize even need to think 

// assert_eq!(150, maximum_product(&[2, 5, -3, -1, 5]));
// even 10, odd 3, max = even * odd,
// if odd > 0 => 

// assert_eq!(900, maximum_product(&[2, 5, -3, 3, 5,-1, 2]));


// lets take an example i might have been mistaken
// +, -, +, ... , +, -
// +, +, +, 0, +, +, -, +, +, -, 0, ... , +, -

// -, +, ..., +
// -, +, ..., +, -
// odd
// even

// it does seem that we need to track the first odd and even product

// states
// - initializing the even product


// if we hit a 0 then we can check if the result is bigger
// if we hit a non-zero opposite sign we should start to track that product
// we should also track the competing product including the max product







// fn unique_paths(grid:&[Vec<usize>]) -> usize {
//     if grid.is_empty() || grid[0].len() == 0 { return 0 }; 
//     let m = grid.len();
//     let n = grid[0].len();
//     let mut prev = vec![0;n];
//     let mut curr = vec![0;n];
//     // 1 is designated as a bomb (1<->0)
//     prev[0] = grid[0][0] ^ 1;
//     for j in 1..m {
//         prev[j] = if grid[0][j] == 1 { 0 } else { prev[j-1] }
        
//     };
//     for i in 1..m {
//         curr[0] = prev[0];
//         for j in 1..n {
//             if grid[i][j] == 1 {
//                 curr[j] = 0;
//             } else {
//                 curr[j] = curr[j-1] + prev[j];
//             }
//         }
//         mem::swap(&mut curr, &mut prev);
//     }
//     prev[n-1]
// }


// fn rob(nums:&mut [usize]) -> usize {
//     let n = nums.len();
//     if n == 0 { return 0 };
//     if n == 1 { return nums[0] };
//     if n >= 3 {
//         nums[n-3] += nums[n-1];
//     }
//     for i in (0..n-3).rev() {
//         nums[i] += nums[i+2].max(nums[i+3]);
//     }
//     nums[0].max(nums[1])
// }


// fn main() {
//     // assert_eq!(2, unique_paths(&vec![vec![0,0,0],vec![0,1,0],vec![0,0,0]]));
//     // assert_eq!(rob(& mut[100,2,3,100]), 200);  // Rob houses 1 and 3 (1-based)
//     // assert_eq!(rob(& mut[1,2,3,1]), 4);  // Rob houses 1 and 3 (1-based)
//     // assert_eq!(rob(& mut[2,7,9,3,1]), 12); // Rob houses 2, 4, and 5
// }
