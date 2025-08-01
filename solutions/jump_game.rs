use std::mem;
// use std::time::Instant;
// use std::hint::black_box;


fn jump_canonical(nums:&[usize]) -> usize {
    // Appears to be the accepted solution
    if nums.len() <= 1 { return 0; }
    let mut jumps = 0;
    let mut reachable = 0;
    let mut prev = 0;
    for i in 0..nums.len() - 1 {
        reachable = reachable.max(i + nums[i]);
        if i == prev {
            jumps += 1;
            prev = reachable;
            if prev >= nums.len() - 1 {
                break
            }
        }
    }
    jumps
}



fn jump(nums:&[usize]) -> usize {
    // Might be functionally equivalent to the cononical version
    let mut reachable = 0; 
    let mut prev = 0;
    let mut jumps = 0; 
    for (i, &n) in nums.iter().enumerate() {
        if i > reachable {
            // problem constraint says this shouldn't happen
            return usize::MAX;
        }
        if prev < i {
            jumps+=1;
            prev = reachable;
        }
        reachable = reachable.max(i + n);
    }
    jumps
}


fn jump_game_idiomatic(nums:&[i32]) -> bool {
    // a little more idiomatic
    let mut reachable = 0;
    for (i, &n) in nums.iter().enumerate() {
        if i > reachable {
            return false;
        }
        reachable = reachable.max(i + n as usize);
    }
    true
}




fn jump_game(nums:&[i32]) -> bool {
    let mut steps = 0;
    for &n in &nums[..nums.len()-1] {
        steps = n.max(steps-1);
        if steps == 0 {
            return false
        }
    }
    true
}




fn main() {
    // assert_eq!(true, jump_game(&[2,3,1,1,4]));
    // assert_eq!(false, jump_game(&[3,2,1,0,4]));
    // assert_eq!(true, jump_game(&[0]));
    assert_eq!(2, jump(&[2,3,1,1,4]));
    assert_eq!(2, jump(&[2,3,0,1,4]));
    assert_eq!(3, jump(&[1,1,1,1,]));
    assert_eq!(3, jump(&[1,2,3,4,5]));
    assert_eq!(2, jump(&[2, 3, 1, 1, 4]));
    assert_eq!(2, jump(&[2, 3, 0, 1, 4]));
    assert_eq!(3, jump(&[1, 1, 1, 1]));
    assert_eq!(3, jump(&[1, 2, 3, 4, 5]));
    assert_eq!(0, jump(&[0]));
    assert_eq!(1, jump(&[1, 0]));
}
