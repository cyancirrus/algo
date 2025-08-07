use std::collections::HashMap;
use std::cmp::Ordering;

#[derive(PartialEq,Eq)]
enum RomanDigit {
    I,
    V,
    X,
    L,
}

impl RomanDigit {
    fn value(&self) -> u8 {
        match self {
            RomanDigit::I => 1,
            RomanDigit::V => 5,
            RomanDigit::X => 10,
            RomanDigit::L => 50,
        }
    }
    fn order(&self) -> usize {
        match self {
            RomanDigit::I => 0,
            RomanDigit::V => 1,
            RomanDigit::X => 2,
            RomanDigit::L => 3,
        }
    }
}

impl Ord for RomanDigit {
    fn cmp(&self, other:&Self) -> Ordering {
        self.order().cmp(&other.order())
    }
}

impl PartialOrd for RomanDigit {
    fn partial_cmp(&self, other:&Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl TryFrom<u8> for RomanDigit {
    type Error = ();

    fn try_from(byte:u8) -> Result<Self, Self::Error> {
        match byte {
            b'I' => Ok(RomanDigit::I),
            b'V' => Ok(RomanDigit::V),
            b'X' => Ok(RomanDigit::X),
            b'L' => Ok(RomanDigit::L),
            _ => Err(())
        }
    }
}

fn parse_tokens(digits: &[RomanDigit]) -> Vec<(usize, usize)>{
    let mut words = vec![];
    let mut start = 0; 
    let mut idx = 0;
    loop {
        if idx == digits.len() {
            words.push((start,idx-1));
            break;
        }
        else if digits[idx] >= digits[start] {
            idx +=1;
            continue
        }
        words.push((start,idx-1));
        start +=1;
        idx +=1;
    }
    words
}

fn eval_roman(nums: &[u8]) -> u8 {
    let digits: Vec<RomanDigit> = nums.iter().copied()
        .map(RomanDigit::try_from)
        .collect::<Result<_, _>>().ok().unwrap();
    let tokens = parse_tokens(&digits);
    let mut sum = 0;
    for (start, end) in tokens {
        for idx in (start..=end).rev() {
            if digits[idx] <= digits[end] {
                sum += digits[idx].value();
            } else {
                sum -= digits[idx].value();
            }
        }
    }
    sum
}

// fn increasing_triplet(nums:&[i32]) -> bool {
//     let mut min = nums[0];
//     let mut med = i32::MAX;
    
//     for &n in nums {
//         if n < min {
//             min = n; 
//         } else if n < med {
//             med = n;
//         } else {
//             return true;
//         }
//     }
//     false
// }

// fn increasing_triplet(nums:&[i32]) -> bool {
//     let mut med_stack = Vec::new();
//     let mut min = nums[0];
//     let mut med = i32::MAX;
    
//     for &n in nums {
//         if n < min {
//             min = n; 
//         } else if n < med {
//             med_stack.push(n);
//             med = n;
//         } else {
//             while let Some(v) = med_stack.pop() {
//                 if v < n {
//                     return true;
//                 }
//             }
//         }
//     }
//     false
// }


fn wiggle_subsequence(nums:&[i32]) -> u32 {
    let n = nums.len();
    let mut up = 1;
    let mut down = 1;

    for i in 1..n {
        if nums[i] > nums[i-1] {
            up = down + 1;
        } else if nums[i-1] > nums[i] {
            down = up + 1;
        }
    }
    up.max(down)
}

// fn wiggle_subsequence(nums:&[i32]) -> u32 {
//     let n = nums.len();
//     let mut dp_inc:Vec<u32> = vec![1;n];
//     let mut dp_dec:Vec<u32> = vec![1;n];
    
//     for i in 1..n {
//         for j in 0..i {
//             if nums[i] > nums[j] {
//                 dp_dec[i] = dp_dec[i].max(dp_inc[j] + 1);
//             } else if nums[j] > nums[i] {
//                 dp_inc[i] = dp_inc[i].max(dp_dec[j] + 1);
//             }
//         }
//     }
//     let mut length = 0;
//     for i in 0..n {
//         length = length.max(dp_inc[i]).max(dp_dec[i]);
//     }
//     length
// }

// fn wiggle_subsequence(nums:&[i32]) -> u32 {
//     let n = nums.len();
//     let mut dp = vec![1;n];
//     let mut deltas:Vec<i32> = vec![0;n];
//     let mut update;
//     let mut temp = 0;
    
//     for i in 1..n {
//         update = false;
//         for j in 0..i {
//             let d = nums[i] - nums[j];
//             if deltas[i].signum() != d.signum() {
//                 dp[i] = dp[i].max(dp[j] + 1);
//                 update = true;
//                 temp = d;
//             }
//         }
//         if update {
//             deltas[i] = temp;
//         }
//     }
//     println!("deltas {deltas:?}");
//     println!("dp {dp:?}");
//     *dp.iter().max().unwrap()
// }


fn wiggle_series(nums:&[i32]) -> u32 {
    let n = nums.len();
    if n <= 2 {
        return 2;
    }
    let mut delta = nums[1] - nums[0];
    let mut longest = 2;
    let mut current = 2;
    for idx in 2..n {
        let d = nums[idx] - nums[idx-1];
        if delta.signum() == d.signum() {
            current = 1;
            longest = longest.max(current);
        }
        delta = d;
        current += 1;
    }
    longest.max(current)
}


fn min_coins(coins:&[u32], amt:u32) -> u32 {
    let mut amounts = vec![u32::MAX;(amt+1) as usize];
    amounts[0] = 0;
    for &c in coins {
        for a in c..=amt{
            let a_idx = a as usize;
            let prev = (a - c) as usize;
            amounts[a as usize] = amounts[a_idx].min(amounts[prev] + 1);
        }
    }
    amounts[amt as usize]
}

fn longest_possible_subsequence(nums:&[u32]) -> u32 {
    let n = nums.len();
    let mut dp = vec![1;n];
    for i in 1..n {
        for j in 0..i {
            if nums[j] < nums[i] {
                dp[i] = dp[i].max(dp[j] + 1);
            }
        }
    }
    *dp.iter().max().unwrap()
}

// fn test() {

//     println!("signum 1 == 0 {}", -1i32.signum() == 0i32.signum());
// }

fn main() {
    println!("eval roman lol {:?}", eval_roman(b"XII"));
    // println!("increasing {:?}", increasing_triplet(&[2,1,5,0,4,6]));
    // println!("increasing {:?}", increasing_triplet(&[5,4,3,2,1,]));
    // println!("increasing {:?}", increasing_triplet(&[1,4,2,35]));
    // println!("increasing {:?}", increasing_triplet(&[1,2,35]));
    // println!("increasing {:?}", increasing_triplet(&[1,2]));
    // println!("increasing {:?}", increasing_triplet(&[1]));
    // println!("increasing {:?}", increasing_triplet(&[]));
    // println!("wiggle {:?}", wiggle_subsequence(&[3,5,5,3]));
    // println!("wiggle {:?}", wiggle_subsequence(&[5,2,5,3]));
    // println!("wiggle {:?}", wiggle_subsequence(&[1,7,4,9,2,5]));
    // println!("wiggle {:?}", wiggle_subsequence(&[1,17,5,10,13,15,10,5,16,8]));
    // println!("testing {:?}", longest_possible_subsequence(&[10,1,2,3,0]));
    // println!("testing {:?}", longest_increasing_subsequence_length(&[1,10,3,]));
    // println!("testing {:?}", longest_increasing_subsequence_length(&[1,2,10,1, 3,]));
    // println!("testing {:?}", longest_increasing_subsequence_length(&[1,2,3,]));
    // println!("testing {:?}", longest_increasing_subsequence_length(&[5,1,2,]));
    // println!("testing {:?}", longest_increasing_subsequence_length(&[1,2,10,3,]));
    // println!("testing {:?}", longest_increasing_subsequence_length(&[1,4,5,100,2,10,3]));
    // println!("min_coins {:?}", min_coins(&[1,5,10], 27));
    // println!("min_coins {:?}", min_coins(&[1, 5, 10], 5));
    // println!("min_coins {:?}", min_coins(&[1], 5));
}
