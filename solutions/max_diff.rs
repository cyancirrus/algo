use std::collections::BinaryHeap;
use std::cmp::Reverse;

// minimize Sum(left) - Sum(right)
// .. = max Sum(right), min Sum(left)

fn min_difference(nums:&[i32]) -> i32 {
    let size = nums.len() / 3;
    let mut lefts:BinaryHeap<i32> = BinaryHeap::new();
    let mut rights:BinaryHeap<Reverse<i32>> = BinaryHeap::new();
    let mut lsum = 0;
    let mut rsum = 0;
    let mut min = i32::MAX;
    for &num in &nums[0..size] {
        lefts.push(num);
        lsum += num;
    }
    for &num in &nums[2*size..] {
        rights.push(Reverse(num));
        rsum += num;
    }
    let mut leftmin = vec![0;size];
    let mut rightmax = vec![0;size];

    for (i, &num) in nums[size..2*size].iter().enumerate() {
        if num < *lefts.peek().unwrap() {
            lsum += num - lefts.pop().unwrap();
            lefts.push(num);
        }
        leftmin[i]  = lsum;
    }
    for (i, &num) in nums[size..2*size].iter().enumerate().rev() {     
        if num > rights.peek().unwrap().0 {
            rsum += num - rights.pop().unwrap().0;
            rights.push(Reverse(num));
        }
        rightmax[i] = rsum;
    }
    for i in 0..size {
        min = min.min(leftmin[i] - rightmax[i]);
    }
    min 
}

fn main() {
}
