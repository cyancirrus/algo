fn permutation_sequence(mut n:usize, mut k:usize) -> String {
    k-=1;
    let mut nums:Vec<usize> = (1..=n).collect();
    let mut result = String::new();
    let mut fact = vec![1;n];
    for i in 1..n {
        fact[i] = fact[i-1] * i;
    }
    for i in (1..=n).rev() {
        let block = fact[i-1];
        let idx = k / block;
        k %= block;
        
        let digit = nums.remove(idx);
        result.push_str(&digit.to_string());
    }
    result
}



// fn permutation_sequence(mut n:usize, mut k:usize) -> String {
//     let mut nums:Vec<usize> = (1..=n).collect();
//     let mut cur = 0;
//     // this one doesn't work b/c like the sequence post swap isn't sorted
//     while k > 0  {
//         println!("n {n:?}, nums {nums:?}");
//         let idx = ( k + 1 ) / n;
//         k = k.saturating_sub(idx * n);
//         nums.swap(cur + idx, cur);
//         cur += 1;
//         n -= 1;
//     }
//     println!("nums {nums:?}");
//     nums.iter().fold(String::new(), |mut acc, &digit| {
//         acc.push_str(word + b'0');
//         acc
//     })
//     println!("nums {nums:?}");
//     String::new()
// }





// fn permutation_sequence(n:u8, mut k:u8) -> String {
//     let mut nums:Vec<u8> = (1..=n).collect();

//     fn bt(k:&mut u8, start:usize, nums:&mut [u8]) {
//         if *k == 0 {
//             return;
//         } else if start == nums.len() {
//             *k -= 1;
//             return;
//         }
//         for i in start..nums.len() {
//             nums.swap(i, start);
//             bt(k, i+1, nums);
//             nums.swap(i, start);
//         }
//     }
//     bt(&mut k, 0 as usize, &mut nums);
//     println!("nums {nums:?}");
//     // nums.iter().fold(String::new(), |mut acc, &digit| {
//     //     acc.push_str(word + b'0');
//     //     acc
//     // })
//     String::new()
// }

// fn permutation_sequence(n:u8, mut k:u8) -> String {
//     let mut nums:Vec<u8> = (1..=n).collect();

//     fn heap(k:&mut u8, n:usize, nums:&mut [u8]) {
//         if *k == 0 {
//             return;
//         } else if n == 1 {
//             *k -= 1;
//             return;
//         }
//         for i in 0..n {
//             heap(k, n-1, nums);
//             if n & 1 == 0 {
//                 nums.swap(i, n-1);
//             } else {
//                 nums.swap(0, n-1); }
//         }
//     }
//     heap(&mut k, n as usize, &mut nums);
//     println!("nums {nums:?}");
//     // nums.iter().fold(String::new(), |mut acc, &digit| {
//     //     acc.push_str(word + b'0');
//     //     acc
//     // })
//     String::new()
// }


fn main() {
    permutation_sequence(3, 2);
    // permutation_sequence(4, 9);
}
