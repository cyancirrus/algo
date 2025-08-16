// 1 2 3
// 1 3 2
// 2 1 3
// 2 3 1
// 3 1 2
// 3 2 1


// 1
// 2, 1 && swap 0, 1
// (2,1), (1, 2)
//
// 3 (2, 1), 3 (1, 2)
// 3 swap 1, swap 2 -> 231, 123
// 132, 213


fn permutations(mut nums:&mut [u32]) -> Vec<Vec<u32>> {
    let mut res = Vec::new();
    heap(nums.len(), &mut nums, &mut res);
    res

}

fn heap(n:usize, nums:&mut [u32], res: &mut Vec<Vec<u32>>) {
    if n == 1 {
        res.push(nums.to_vec());
    }
    for i in 0..n {
        heap(n-1, nums, res);
        if n & 1 == 1 {
            nums.swap(i, n - 1);
        } else {
            nums.swap(0, n-1)
        }
    }
}

fn permutations_recurs(mut nums:&mut [u32]) -> Vec<Vec<u32>> {
    let mut res = Vec::new();
    bt(0, &mut nums, &mut res);
    res
}

fn bt(start: usize, nums: &mut [u32], res: &mut Vec<Vec<u32>>) {
    if start == nums.len() {
        res.push(nums.to_vec());
        return;
    }
    // for every cursor swap with every number including itself
    for i in start..nums.len() {
        nums.swap(start, i);
        bt(start + 1, nums, res);
        nums.swap(start, i);
    }
}

fn permutations_recurrance(nums:&mut Vec<u32>) -> Vec<Vec<u32>> {
    let mut perms = vec![];
    while let Some(n) = nums.pop() {
        perms = add_number(n, &mut perms); 
    }
    perms
}


fn add_number(num:u32, perms:&mut Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    if perms.is_empty() {
        return vec![vec![num]];
    }
    let mut solutions = vec![];
    let l = perms[0].len();
    while let Some(p) = &mut perms.pop() {
        p.push(num);
        for i in 0..=l {
            p.swap(i, l);
            solutions.push(p.clone());
            p.swap(l, i);
        }
    }
    solutions
}

fn main() {
    println!("perms {:?}", permutations(&mut vec![2,1,0]));
}
