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
    // with.extend(*without);
    cset.pop();
}


fn main() {
    // assert_eq!(vec![vec![]], powerset(&[]));
    assert_eq!(vec![vec![], vec![1]], powerset(&[1]));
    assert_eq!(vec![vec![], vec![1], vec![2], vec![1,2]], powerset(&[1,2]));
}
