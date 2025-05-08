// fn three_sum_closest(target:i32, nums:&[i32]) -> Option<i32> {
//     if nums.len() < 3 { return None };
//     let mut closest:i32 = i32::MAX;
//     let mut distance = i32::MAX ;
//     for i in 0..nums.len() {
//         for j in 0..i {
//             for k in 0..j {
//                 let val = nums[i] + nums[j] + nums[k];
//                 let dist = (target - val).abs();
//                 if dist == 0 {
//                     return Some(target)
//                 } else if dist < distance {
//                     closest = val;
//                     distance = dist;
//                 }
//             }
//         }
//     }
//     Some(closest)
// }

fn three_sum_closest(target:i32, nums:&mut [i32]) -> Option<i32> {
    if nums.len() < 3 { return None };
    // O(nlogn) < O(n^3) trivial cost for speedup
    nums.sort();
    let mut closest:i32 = i32::MAX;
    let mut distance:i32 = i32::MAX;
    for i in 2..nums.len() {
        let mut left = 0;
        let mut right = i - 1;
        while left < right {
            let sum = nums[left] + nums[i] + nums[right];
            let dist = (target - sum).abs();
            if sum < target {
                left += 1;
            } else if sum > target {
                right -= 1;
            } else if dist == 0 {
                return Some(sum)
            }
            // less than or equal to handle nums=&[i32::MIN]
            if dist <= distance {
                distance=dist;
                closest=sum;
            }

        }
    }
    Some(closest)
}


fn main() {
    println!("hello world");
    let mut nums = vec![-1, 2, 1, -4];
    assert_eq!(three_sum_closest(1, &mut nums), Some(2)); // Closest to 1 is 2
}





#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_exact_match() {
        let mut nums = vec![-1, 2, 1, -4];
        assert_eq!(three_sum_closest(2, &mut nums), Some(2)); // -1 + 2 + 1 = 2
    }

    #[test]
    fn test_basic_closest_no_match() {
        let mut nums = vec![-1, 2, 1, -4];
        assert_eq!(three_sum_closest(1, &mut nums), Some(2)); // Closest to 1 is 2
    }

    #[test]
    fn test_multiple_possible_closest() {
        let mut nums = vec![1, 1, 1, 0];
        assert_eq!(three_sum_closest(-100, &mut nums), Some(2)); // Smallest possible sum is 1+1+0 = 2
    }

    #[test]
    fn test_all_positives() {
        let mut nums = vec![1, 2, 5, 10, 11];
        assert_eq!(three_sum_closest(12, &mut nums), Some(13)); // 1+2+10
    }

    #[test]
    fn test_all_negatives() {
        let mut nums = vec![-8, -6, -5, -2, -1];
        assert_eq!(three_sum_closest(-14, &mut nums), Some(-14)); // -6+-5+-3 or similar
    }

    #[test]
    fn test_exact_match_multiple_times() {
        let mut nums = vec![0, 0, 0];
        assert_eq!(three_sum_closest(1, &mut nums), Some(0)); // Only one possible sum
    }

    #[test]
    fn test_insufficient_elements() {
        let mut nums = vec![1, 2];
        assert_eq!(three_sum_closest(3, &mut nums), None); // Not enough elements
    }

    #[test]
    fn test_empty() {
        let mut nums = vec![];
        assert_eq!(three_sum_closest(0, &mut nums), None);
    }
}

