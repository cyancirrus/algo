fn three_sum(target:i32, nums:&mut [i32]) -> Option<[i32;3]> {
    // O(nlogn) << O(n^3)
    nums.sort();
    let len = nums.len();
    for i in 0..len - 2{
        if nums[i] + nums[i+1] + nums[i+2] > target {
            return None;
        }
        for j in i+1..len -1  {
            // all considered results would be too large
            if nums[i] + nums[j] + nums[j+1] > target {
                break; // nothing ahead will be better
            }
            let psum = nums[i] + nums[j];
            let search = binary_search(target - psum, &nums[j+1..]);
            if let Some(result) = search {
                return Some([nums[i], nums[j], result])
            }
        }
    }
    None
}
fn binary_search(target:i32, nums:&[i32]) -> Option<i32> {
    let len = nums.len();
    if len == 0 { return None};
    let (mut l, mut u) = (0, nums.len() - 1);
    while l <= u {
        let c = (l + u) / 2;
        let num = nums[c];
        if target == num {
            return Some(target)
        }
        if num < target {
            l= c + 1;
        } else if num > target {
            u= c - 1;
        }
    }
    None
}

fn main() {
    // Test case 1: A simple test case where a valid triplet exists
    let mut nums = vec![-1, 2, 1, -4];
    assert_eq!(three_sum(2, &mut nums), Some([-1, 1, 2]));
    println!("Test 1 passed!");

    // // Test case 2: A case where the closest sum is less than the target (no exact match)
    // let mut nums = vec![1, 2, 3, 4, 5];
    // assert_eq!(three_sum(10, &mut nums), Some([3, 4, 5]));
    // println!("Test 2 passed!");

    // // Test case 3: Case with negative numbers, should return closest sum
    // let mut nums = vec![-5, -3, -2, -1, 1, 2, 3, 4];
    // assert_eq!(three_sum(1, &mut nums), Some([-5, 4, 2]));
    // println!("Test 3 passed!");

    // // Test case 4: Case where no triplet can sum up to the target (return None)
    // let mut nums = vec![1, 1, 1, 1];
    // assert_eq!(three_sum(5, &mut nums), None);
    // println!("Test 4 passed!");

    // // Test case 5: Case with duplicates, expecting a valid triplet
    // let mut nums = vec![1, 1, -1, 0];
    // assert_eq!(three_sum(0, &mut nums), Some([1, -1, 0]));
    // println!("Test 5 passed!");

    // // Test case 6: Edge case with fewer than 3 numbers
    // let mut nums = vec![1, 2];
    // assert_eq!(three_sum(5, &mut nums), None);
    // println!("Test 6 passed!");

    // // Test case 7: Case where sum of triplet is zero
    // let mut nums = vec![-1, 0, 1, 2, -1, -4];
    // assert_eq!(three_sum(0, &mut nums), Some([-1, 0, 1]));
    // println!("Test 7 passed!");

    // // Test case 8: All numbers are the same, should return a valid triplet
    // let mut nums = vec![2, 2, 2, 2, 2];
    // assert_eq!(three_sum(6, &mut nums), Some([2, 2, 2]));
    // println!("Test 8 passed!");
}
