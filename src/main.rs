use std::mem;

// shows memoization pattern



fn fib_iter(mut n:usize) -> impl FnMut() -> Option<usize> {
    let mut l = 0;
    let mut u = 1;
    
    move || {
        if n == 0 {
            None
        } else {
            n-=1;
            mem::swap(&mut u, &mut l);
            u = l + u;
            Some(u)
        }
    }
}

fn run_fib(steps: usize) -> usize {
    let mut f = fib_iter(steps);
    let mut last:usize = 0;
    for _ in 0usize..steps {
        last = f().unwrap();
    }
    last
}



// use std::collections::HashSet;

// fn gray_code_powerset(nums: &[usize]) -> impl Iterator<Item = HashSet<usize>> {
//     let n = nums.len();
//     let mut set = HashSet::with_capacity(n);
//     let mut idx = 0;

//     std::iter::from_fn(move || {
//         if idx >= (1 << n) {
//             return None;
//         }

//         let g = idx ^ (idx >> 1);

//         if idx > 0 {
//             let prev = (idx - 1) ^ ((idx - 1) >> 1);
//             let changed = g ^ prev;
//             let bit = changed.trailing_zeros() as usize;

//             if g & (1 << bit) != 0 {
//                 set.insert(nums[bit]);
//             } else {
//                 set.remove(&nums[bit]);
//             }
//         }

//         idx += 1;
//         Some(set.clone()) // optional: return &set if lifetimes allow
//     })
// }



// fn super_cool_next_element_gray_code() {

// let mut subset = Vec::new();
// let mut current_sum = 0.0;

// for i in 0..(1 << n) {
//     let gray = i ^ (i >> 1);
//     let diff = gray ^ prev_gray;

//     let idx = diff.trailing_zeros() as usize;
//     if (gray & diff) != 0 {
//         // Added idx
//         subset.push(data[idx]);
//         current_sum += data[idx];
//     } else {
//         // Removed idx
//         let pos = subset.iter().position(|&x| x == data[idx]).unwrap();
//         subset.remove(pos);
//         current_sum -= data[idx];
//     }

//     let current_mean = current_sum / subset.len() as f64;
//     // do something with current_mean...

//     prev_gray = gray;
// }



fn powerset_dp(nums:&[usize]) -> Vec<Vec<usize>> {
    if nums.is_empty() { return vec![vec![]] };
    let n = nums.len();
    let mut sets:Vec<Vec<usize>> = vec![vec![];1<<n];
    
    for i in 0usize..1<<n {
        let low_bit= i & (!i + 1);
        let idx = low_bit.count_zeros() as usize;
        sets[i] = sets[i^low_bit].clone();
        sets[i].push(nums[idx]);
    }
    sets
}

fn powerset(nums:&[usize]) -> Vec<Vec<usize>> {
    let mut pset = vec![];
    _powerset_(0, &mut vec![], nums, &mut pset);
    pset
}

fn _powerset_(idx:usize, cset:&mut Vec<usize>, nums:&[usize], sets:&mut Vec<Vec<usize>>) {
    if idx == nums.len() {
        sets.push(cset.clone());
        return
    };
    _powerset_(idx + 1, cset, nums, sets);
    cset.push(nums[idx]);
    _powerset_(idx+1, cset, nums, sets);
    cset.pop();
}


fn main() {
    // assert_eq!(vec![vec![]], powerset(&[]));
    // assert_eq!(vec![vec![], vec![1]], powerset(&[1]));
    // assert_eq!(vec![vec![], vec![1], vec![2], vec![1,2]], powerset(&[1,2]));
    let mut next_fib = fib_iter(10);

    while let Some(val) = next_fib() {
        println!("{}", val);
    }

    println!("Run fib 10_000 :: {}", run_fib(90));

}
