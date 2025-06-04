fn one_three_two(nums:&[i32]) -> bool {
    let mut stack = vec![];
    let mut third = i32::MIN;

    for &num in nums.iter().rev() {
        if num < third {
            return true;
        }
        while let Some(&top) = stack.last() {
            if num > top {
                third = stack.pop().unwrap();
            } else {
                break
            }
        }
        stack.push(num);
    }
    false
}

fn main() {
    println!("What i'm confused {:?}", one_three_two(&[1, 4, 0, -1, -2, -3, 2, 1]));
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

    // println!("What i'm confused {:?}", one_three_two(&[-2,1,2,-2,1,2]));
    // println!("What i'm confused {:?}", one_three_two(&[1,2,3,3,3,4,5,3]));
    // println!("What i'm confused {:?}", one_three_two(&[0,1,3]));
    
}
