fn one_three_two(nums:&[i32]) -> bool {
    let mut second = i32::MIN;
    let mut third = i32::MIN;

    for i in (0..nums.len()).rev() {
        if nums[i] < third {
            return true;
        }
        if nums[i] > second {
            // best number is always closest to max
            third = second;
            second = nums[i];
        } else if nums[i] > third {
            // initialize if not initialized
            third = nums[i];
        }
    }
    false
}

fn main() {
    // println!("What i'm confused {:?}", one_three_two(&[1, 4, 0, -1, -2, -3, 2, 1]));
    // println!("What i'm confused {:?}", one_three_two(&[3, 5, 0, 3, 4]));
    // println!("What i'm confused {:?}", one_three_two(&[1,2,3]));
    // println!("What i'm confused {:?}", one_three_two(&[1,5, 2,3]));
    // println!("What i'm confused {:?}", one_three_two(&[5,2,3]));
    // println!("What i'm confused {:?}", one_three_two(&[5,2]));
    // println!("What i'm confused {:?}", one_three_two(&[1,2]));
    // println!("What i'm confused {:?}", one_three_two(&[8, 10, 5, 7, 2, 4]));
    // println!("What i'm confused {:?}", one_three_two(&[5, 3, 7, 0, 2, 1]));
    // println!("What i'm confused {:?}", one_three_two(&[1,0,1,-4,-3]));
    // println!("What i'm confused {:?}", one_three_two(&[1,4,0,-1,-2,-3,-1,-2]));
    // println!("What i'm confused {:?}", one_three_two(&[6, 12, 3, 4, 6, 11, 20]));
    // println!("What i'm confused {:?}", one_three_two(&[1,2,0,1,2,1]));

    println!("What i'm confused {:?}", one_three_two(&[-2,1,2,-2,1,2]));
    println!("What i'm confused {:?}", one_three_two(&[1,2,3,3,3,4,5,3]));

    
}
