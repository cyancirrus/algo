use std::mem;
// use std::time::Instant;
// use std::hint::black_box;


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
    assert_eq!(true, jump_game(&[2,3,1,1,4]));
    assert_eq!(false, jump_game(&[3,2,1,0,4]));
    assert_eq!(jump_game(&[0]), true);
}
